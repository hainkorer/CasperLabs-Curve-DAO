use crate::{data::*, event::LiquidityGaugeRewardWrapperEvent};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{
        runtime::{self, get_blocktime},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use common::errors::*;
use contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDITYGAUGEREWARDWRAPPER<Storage: ContractStorage>: ContractContext<Storage> {
    // @notice Contract constructor
    // @param _name Token full name
    // @param _symbol Token symbol
    // @param _gauge Liquidity gauge contract address
    // @param _admin Admin who can kill the gauge
    fn init(
        &self,
        name: String,
        symbol: String,
        gauge: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        set_name(name);
        set_symbol(symbol);
        set_decimals(9);
        let lp_token: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "lp_token",
            runtime_args! {},
        );
        let () = runtime::call_versioned_contract(
            lp_token.into_hash().unwrap_or_revert().into(),
            None,
            "approve",
            runtime_args! {
                "spender" => gauge,
                "amount" => U256::MAX
            },
        );
        let minter: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "minter",
            runtime_args! {},
        );
        set_minter(minter);
        let crv_token: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "crv_token",
            runtime_args! {},
        );
        set_crv_token(crv_token);
        let rewarded_token: Key = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "rewarded_token",
            runtime_args! {},
        );
        set_rewarded_token(rewarded_token);
        set_lp_token(lp_token);
        set_gauge(gauge);
        set_admin(admin);
        Allowances::init();
        BalanceOf::init();
        RewardIntegralFor::init();
        CrvIntegralFor::init();
        ClaimableCrv::init();
        ClaimableRewards::init();
        ApprovedToDeposit::init();
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }
    fn _checkpoint(&self, addr: Key) {
        let gauge: Key = get_gauge();
        let mut token: Key = get_crv_token();
        let total_balance: U256 = get_total_supply();
        let mut d_reward: U256 = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => self.get_caller()
            },
        );
        let () = runtime::call_versioned_contract(
            get_minter().into_hash().unwrap_or_revert().into(),
            None,
            "mint",
            runtime_args! {
                "gauge_addr" => gauge
            },
        );
        let mut d_reward_updated: U256 = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => self.get_caller()
            },
        );
        d_reward = d_reward_updated.checked_sub(d_reward).unwrap_or_revert();
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert()
                .checked_div(total_balance)
                .unwrap_or_revert();
        }
        let mut i: U256 = get_crv_integral().checked_add(di).unwrap_or_revert();
        set_crv_integral(i);
        let balance_of: U256 = BalanceOf::instance().get(&addr);
        let crv_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        ClaimableCrv::instance().set(
            &addr,
            ClaimableCrv::instance()
                .get(&addr)
                .checked_add(balance_of)
                .unwrap_or_revert()
                .checked_mul(i.checked_sub(crv_integral_for).unwrap_or_revert())
                .unwrap_or_revert()
                .checked_div(U256::from(TEN_E_NINE))
                .unwrap_or_revert(),
        );
        CrvIntegralFor::instance().set(&addr, i);
        token = get_rewarded_token();
        d_reward = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => self.get_caller()
            },
        );
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "claim_rewards",
            runtime_args! {},
        );
        d_reward_updated = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => self.get_caller()
            },
        );
        d_reward = d_reward_updated.checked_sub(d_reward).unwrap_or_revert();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert()
                .checked_div(total_balance)
                .unwrap_or_revert();
        }
        i = get_reward_integral().checked_add(di).unwrap_or_revert();
        set_reward_integral(i);
        let reward_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        ClaimableRewards::instance().set(
            &addr,
            ClaimableRewards::instance()
                .get(&addr)
                .checked_add(balance_of)
                .unwrap_or_revert()
                .checked_mul(i.checked_sub(reward_integral_for).unwrap_or_revert())
                .unwrap_or_revert()
                .checked_div(U256::from(TEN_E_NINE))
                .unwrap_or_revert(),
        );
        RewardIntegralFor::instance().set(&addr, i);
    }
    // @notice Record a checkpoint for `addr`
    // @param addr User address
    // @return bool success
    fn user_checkpoint(&self, addr: Key) -> bool {
        if !((self.get_caller() == addr) || (self.get_caller() == get_minter())) {
            runtime::revert(ApiError::from(Error::RewardWrapperUnauthorized));
        }
        self._checkpoint(addr);
        return true;
    }

    // @notice Get the number of claimable tokens per user
    // @dev This function should be manually changed to "view" in the ABI
    // @return uint256 number of claimable tokens per user
    fn claimable_tokens(&self, addr: Key) -> U256 {
        let d_reward: U256 = runtime::call_versioned_contract(
            get_gauge().into_hash().unwrap_or_revert().into(),
            None,
            "claimable_tokens",
            runtime_args! {},
        );
        let total_balance: U256 = get_total_supply();
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert()
                .checked_div(total_balance)
                .unwrap_or_revert();
        }
        let i: U256 = get_crv_integral().checked_add(di).unwrap_or_revert();
        let balance_of: U256 = BalanceOf::instance().get(&addr);
        let crv_integral_for: U256 = CrvIntegralFor::instance().get(&addr);
        let claimable_crv: U256 = ClaimableCrv::instance().get(&addr);
        return claimable_crv
            .checked_add(balance_of)
            .unwrap_or_revert()
            .checked_mul(i.checked_sub(crv_integral_for).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(U256::from(TEN_E_NINE))
            .unwrap_or_revert();
    }

    // @notice Get the number of claimable reward tokens per user
    // @dev This function should be manually changed to "view" in the ABI
    // @return uint256 number of claimable tokens per user
    #[allow(non_snake_case)]
    fn claimable_reward(&self, addr: Key) -> U256 {
        let gauge: Key = get_gauge();
        let claimable_reward: U256 = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "claimable_reward",
            runtime_args! {
                "addr" => Key::from(get_package_hash())
            },
        );
        let claimed_rewards_for: U256 = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "claimed_rewards_for",
            runtime_args! {
                "addr" => Key::from(get_package_hash())
            },
        );
        let d_reward: U256 = claimable_reward
            .checked_sub(claimed_rewards_for)
            .unwrap_or_revert();
        let total_balance: U256 = get_total_supply();
        let mut di: U256 = 0.into();
        if total_balance > 0.into() {
            di = U256::from(TEN_E_NINE)
                .checked_mul(d_reward)
                .unwrap_or_revert()
                .checked_div(total_balance)
                .unwrap_or_revert();
        }
        let i: U256 = get_reward_integral().checked_add(di).unwrap_or_revert();
        let balance_of: U256 = BalanceOf::instance().get(&addr);
        let reward_integral_for: U256 = RewardIntegralFor::instance().get(&addr);
        let claimable_rewards: U256 = ClaimableRewards::instance().get(&addr);
        return claimable_rewards
            .checked_add(balance_of)
            .unwrap_or_revert()
            .checked_mul(i.checked_sub(reward_integral_for).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(U256::from(TEN_E_NINE))
            .unwrap_or_revert();
    }

    /// @notice Kick `addr` for abusing their boost
    /// @dev Only if either they had another voting event, or their voting escrow lock expired
    /// @param addr Address to kick
    fn claim_tokens(&self, addr: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsLocked));
        }
        set_lock(true);
        self._checkpoint(addr);
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_crv_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => addr,
                "amount" => ClaimableCrv::instance().get(&addr)
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::from(ret.err().unwrap_or_revert()));
        }
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_rewarded_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => addr,
                "amount" => ClaimableRewards::instance().get(&addr)
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::from(ret.err().unwrap_or_revert()));
        }
        ClaimableCrv::instance().set(&addr, 0.into());
        ClaimableRewards::instance().set(&addr, 0.into());
        set_lock(false);
    }

    // @notice Set whether `addr` can deposit tokens for `self.get_caller()`
    // @param addr Address to set approval on
    // @param can_deposit bool - can this account deposit for `self.get_caller()`?
    fn set_approve_deposit(&self, addr: Key, can_deposit: bool) {
        ApprovedToDeposit::instance().set(&addr, &self.get_caller(), can_deposit);
    }

    /// @notice Deposit `_value` LP tokens
    /// @param _value Number of tokens to deposit
    /// @param addr Address to deposit for
    fn deposit(&self, value: U256, addr: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsLocked));
        }
        set_lock(true);
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsKilled));
        }
        if addr != self.get_caller() {
            if !(ApprovedToDeposit::instance().get(&self.get_caller(), &addr)) {
                runtime::revert(ApiError::from(Error::RewardWrapperNotApproved));
            }
        }
        self._checkpoint(addr);
        if value != 0.into() {
            let balance: U256 = BalanceOf::instance()
                .get(&addr)
                .checked_add(value)
                .unwrap_or_revert();
            let supply: U256 = get_total_supply().checked_add(value).unwrap_or_revert();
            BalanceOf::instance().set(&addr, balance);
            set_total_supply(supply);
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => self.get_caller(),
                    "recipient" => Key::from(get_package_hash()),
                    "amount" => value
                },
            );
            if ret.is_err() {
                runtime::revert(ApiError::from(ret.err().unwrap_or_revert()));
            }
            let () = runtime::call_versioned_contract(
                get_gauge().into_hash().unwrap_or_revert().into(),
                None,
                "deposit",
                runtime_args! {
                    "value" => value
                },
            );
        }
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Deposit {
                provider: addr,
                value,
            },
        );
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Transfer {
                from: zero_address(),
                to: addr,
                value,
            },
        );
        set_lock(false);
    }

    /// @notice Withdraw `_value` LP tokens
    /// @param _value Number of tokens to withdraw
    fn withdraw(&self, value: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsLocked));
        }
        set_lock(true);
        self._checkpoint(self.get_caller());
        if value != 0.into() {
            let balance: U256 = BalanceOf::instance()
                .get(&self.get_caller())
                .checked_sub(value)
                .unwrap_or_revert();
            let supply: U256 = get_total_supply().checked_sub(value).unwrap_or_revert();
            BalanceOf::instance().set(&self.get_caller(), balance);
            set_total_supply(supply);
            let () = runtime::call_versioned_contract(
                get_gauge().into_hash().unwrap_or_revert().into(),
                None,
                "withdraw",
                runtime_args! {
                    "value" => value,
                    "claim_rewards" => true
                },
            );
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => self.get_caller(),
                    "amount" => value
                },
            );
            if ret.is_err() {
                runtime::revert(ApiError::from(ret.err().unwrap_or_revert()));
            }
        }
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Withdraw {
                provider: self.get_caller(),
                value,
            },
        );
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Transfer {
                from: self.get_caller(),
                to: zero_address(),
                value,
            },
        );
        set_lock(false);
    }
    // @dev Function to check the amount of tokens that an owner allowed to a spender.
    // @param _owner The address which owns the funds.
    // @param _spender The address which will spend the funds.
    // @return An uint256 specifying the amount of tokens still available for the spender.
    fn allowance(&self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }
    fn _transfer(&self, owner: Key, recipient: Key, amount: U256) {
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::RewardWrapperIsKilled));
        }
        self._checkpoint(owner);
        self._checkpoint(recipient);
        if amount != 0.into() {
            let balance_owner: U256 = BalanceOf::instance()
                .get(&owner)
                .checked_sub(amount)
                .unwrap_or_revert();
            BalanceOf::instance().set(&owner, balance_owner);
            let balance_recipient: U256 = BalanceOf::instance()
                .get(&recipient)
                .checked_add(amount)
                .unwrap_or_revert();

            BalanceOf::instance().set(&recipient, balance_recipient);
        }
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Transfer {
                from: owner,
                to: recipient,
                value: amount,
            },
        );
    }
    // @dev Transfer token for a specified address
    // @param _to The address to transfer to.
    // @param _value The amount to be transferred.
    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        self._transfer(self.get_caller(), recipient, amount);
        return Ok(());
    }
    // @dev Transfer tokens from one address to another.
    // @param _from address The address which you want to send tokens from
    // @param _to address The address which you want to transfer to
    // @param _value uint256 the amount of tokens to be transferred
    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowances::instance().get(&owner, &self.get_caller());
        if allowance != U256::MAX {
            Allowances::instance().set(
                &owner,
                &self.get_caller(),
                allowance.checked_sub(amount).unwrap_or_revert(),
            );
        }
        self._transfer(owner, recipient, amount);
        return Ok(());
    }
    // @notice Approve the passed address to transfer the specified amount of
    //  tokens on behalf of msg.sender
    //  @dev Beware that changing an allowance via this method brings the risk
    //  that someone may use both the old and new allowance by unfortunate
    //  transaction ordering. This may be mitigated with the use of
    //  {increaseAllowance} and {decreaseAllowance}.
    //  https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
    // @param _spender The address which will transfer the funds
    // @param _value The amount of tokens that may be transferred
    fn approve(&self,spender: Key,amount: U256){
        Allowances::instance().set(&self.get_caller(), &spender, amount);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Approval {
                owner: self.get_caller(),
                spender: spender,
                value: amount,
            },
        );
    }
    // @notice Increase the allowance granted to `_spender` by the caller
    // @dev This is alternative to {approve} that can be used as a mitigation for
    //      the potential race condition
    // @param _spender The address which will transfer the funds
    // @param _added_value The amount of to increase the allowance
    // @return Result on success
    fn increase_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowances::instance()
            .get(&self.get_caller(), &spender)
            .checked_add(amount)
            .unwrap_or_revert();
        Allowances::instance().set(&self.get_caller(), &spender, allowance);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Approval {
                owner: self.get_caller(),
                spender: spender,
                value: allowance,
            },
        );
        return Ok(());
    }
    // @notice Decrease the allowance granted to `_spender` by the caller
    // @dev This is alternative to {approve} that can be used as a mitigation for
    //      the potential race condition
    // @param _spender The address which will transfer the funds
    // @param _subtracted_value The amount of to decrease the allowance
    // @return Result on success
    fn decrease_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowances::instance()
            .get(&self.get_caller(), &spender)
            .checked_sub(amount)
            .unwrap_or_revert();
        Allowances::instance().set(&self.get_caller(), &spender, allowance);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::Approval {
                owner: self.get_caller(),
                spender: spender,
                value: allowance,
            },
        );
        return Ok(());
    }
    fn kill_me(&self) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminOnly));
        }
        set_is_killed(!get_is_killed());
    }

    /// @notice Transfer ownership of GaugeController to `addr`
    /// @param addr Address to have ownership transferred to
    fn commit_transfer_ownership(&self, addr: Key) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminOnly));
        }
        set_future_admin(addr);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::CommitOwnership { admin: addr },
        );
    }

    /// @notice Apply pending ownership transfer
    fn apply_transfer_ownership(&self) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminOnly));
        }
        let admin: Key = get_future_admin();
        if !(admin != zero_address()) {
            runtime::revert(ApiError::from(Error::RewardWrapperAdminNotSet));
        }
        set_admin(admin);
        LIQUIDITYGAUGEREWARDWRAPPER::emit(
            self,
            &LiquidityGaugeRewardWrapperEvent::ApplyOwnership { admin },
        );
    }

    fn emit(&self, liquidity_gauge_reward_wrapper_event: &LiquidityGaugeRewardWrapperEvent) {
        let mut events = Vec::new();
        let tmp = get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match liquidity_gauge_reward_wrapper_event {
            LiquidityGaugeRewardWrapperEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeRewardWrapperEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeRewardWrapperEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            LiquidityGaugeRewardWrapperEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            LiquidityGaugeRewardWrapperEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeRewardWrapperEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert(
                    "event_type",
                    liquidity_gauge_reward_wrapper_event.type_name(),
                );
                event.insert("from", owner.to_string());
                event.insert("to", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}