use crate::alloc::string::ToString;
use crate::data::{
    self, account_zero_address, zero_address, Allowances, Balances, ClaimData, ClaimDataStruct,
    RewardBalances, RewardData, RewardIntegral, RewardIntegralFor, RewardTokens, RewardsReceiver,
    CLAIM_FREQUENCY, MAX_REWARDS,
};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::bytesrepr::Bytes;
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};
use common::errors::*;

pub enum REWARDONLYGAUGEEvent {
    Withdraw {
        provider: Key,
        value: U256,
    },
    Deposit {
        provider: Key,
        value: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
}

impl REWARDONLYGAUGEEvent {
    pub fn type_name(&self) -> String {
        match self {
            REWARDONLYGAUGEEvent::Withdraw {
                provider: _,
                value: _,
            } => "withdraw",
            REWARDONLYGAUGEEvent::Deposit {
                provider: _,
                value: _,
            } => "deposit",
            REWARDONLYGAUGEEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            REWARDONLYGAUGEEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            REWARDONLYGAUGEEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            REWARDONLYGAUGEEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
        }
        .to_string()
    }
}


pub trait REWARDONLYGAUGE<Storage: ContractStorage>: ContractContext<Storage> {
    /// """
    /// @notice Contract constructor
    /// @param _admin Admin who can kill the gauge
    /// @param _lp_token Liquidity Pool contract address
    /// """
    fn init(
        &mut self,
        _admin: Key,
        _lp_token: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        lock: u64,
    ) {
        let _lp_token_hash_add_array = match _lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let _lp_token_package_hash = ContractPackageHash::new(_lp_token_hash_add_array);
        let symbol: String = runtime::call_versioned_contract(
            _lp_token_package_hash,
            None,
            "symbol",
            runtime_args! {},
        );
        let mut name: String = "Curve.fi ".to_string();
        let post_name: &str = "RewardGauge Deposit";
        name.push_str(symbol.as_str());
        name.push_str(post_name);
        let decimals: u8 = 9;
        let total_supply: U256 = 0.into();
        data::set_name(name);
        data::set_symbol(symbol + "-gauge");
        data::set_total_supply(total_supply);
        data::set_decimals(decimals);
        data::set_admin(_admin);
        data::set_lp_token(_lp_token);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_lock(lock);
        Allowances::init();
        Balances::init();
        RewardTokens::init();
        RewardBalances::init();
        RewardsReceiver::init();
        RewardIntegral::init();
        RewardIntegralFor::init();
        ClaimData::init();
    }

    fn balance_of(&mut self, owner: Key) -> U256 {
        Balances::instance().get(&owner)
    }
    fn reward_balances(&mut self, owner: Key) -> U256 {
        RewardBalances::instance().get(&owner)
    }
    fn rewards_receiver(&mut self, claimant: Key) -> Key {
        RewardsReceiver::instance().get(&claimant)
    }
    fn reward_integral(&mut self, reward_token: Key) -> U256 {
        RewardIntegral::instance().get(&reward_token)
    }
    fn reward_tokens(&mut self, index: U256) -> Key {
        RewardTokens::instance().get(&index)
    }

    fn transfer(&mut self, _to: Key, _value: U256) -> bool {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        self._transfer(self.get_caller(), _to, _value);
        data::set_lock(0);
        return true;
    }

    fn approve(&mut self, spender: Key, _value: U256) -> bool {
        self._approve(self.get_caller(), spender, _value)
    }

    fn _approve(&mut self, _owner: Key, _spender: Key, _value: U256) -> bool {
        Allowances::instance().set(&_owner, &_spender, _value);
        self.emit(&REWARDONLYGAUGEEvent::Approval {
            owner: _owner,
            spender: _spender,
            value: _value,
        });
        return true;
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn reward_integral_for(&mut self, reward_token: Key, claiming_address: Key) -> U256 {
        RewardIntegralFor::instance().get(&reward_token, &claiming_address)
    }

    fn claim_data(&mut self, user: Key, claiming_address: Key) -> ClaimDataStruct {
        ClaimData::instance().get(&user, &claiming_address)
    }

    fn increase_allowance(&mut self, _spender: Key, _added_value: U256) -> bool {
        let allowances = Allowances::instance();
        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &_spender);
        let new_allowance: U256 = spender_allowance
            .checked_add(_added_value)
            .ok_or(Error::RewardOnlyGaugeOverFlow1)
            .unwrap_or_revert();
        self._approve(owner, _spender, new_allowance);
        return true;
    }

    fn decrease_allowance(&mut self, _spender: Key, _subtracted_value: U256) -> bool {
        let allowances = Allowances::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &_spender);

        let new_allowance: U256 = spender_allowance
            .checked_sub(_subtracted_value)
            .ok_or(Error::RewardOnlyGaugeUnderFlow1)
            .unwrap_or_revert();
        self._approve(owner, _spender, new_allowance);

        return true;
    }

    fn transfer_from(&mut self, _from: Key, _to: Key, _value: U256) -> bool {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let allowances = Allowances::instance();
        let _allowance: U256 = allowances.get(&_from, &self.get_caller());
        if _allowance != U256::MAX {
            let new_allowance: U256 = _allowance
                .checked_sub(_value)
                .ok_or(Error::RewardOnlyGaugeUnderFlow2)
                .unwrap_or_revert();
            self._approve(_from, self.get_caller(), new_allowance);
        }
        self._transfer(_from, _to, _value);
        data::set_lock(0);
        return true;
    }

    fn _transfer(&mut self, _from: Key, _to: Key, _value: U256) {
        let reward_data: RewardData = self.reward_data();
        let _reward_contract = reward_data.address;
        if _value != 0.into() {
            let total_supply = self.total_supply();
            self._checkpoint_rewards(_from, total_supply, false, account_zero_address());
            let balances: Balances = Balances::instance();
            let _from_balance: U256 = balances.get(&_from);
            let from_new_balance = _from_balance
                .checked_sub(_value)
                .ok_or(Error::RewardOnlyGaugeUnderFlow3)
                .unwrap_or_revert();
            balances.set(&_from, from_new_balance);

            self._checkpoint_rewards(_to, total_supply, false, account_zero_address());
            let balances: Balances = Balances::instance();
            let _to_balance: U256 = balances.get(&_to);
            let to_new_balance = _from_balance
                .checked_sub(_value)
                .ok_or(Error::RewardOnlyGaugeUnderFlow4)
                .unwrap_or_revert();
            balances.set(&_to, to_new_balance);
        }
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: _from,
            to: _to,
            value: _value,
        });
    }

    fn total_supply(&mut self) -> U256 {
        data::total_supply()
    }

    fn name(&mut self) -> String {
        data::name()
    }

    fn symbol(&mut self) -> String {
        data::symbol()
    }

    fn decimals(&mut self) -> u8 {
        data::decimals()
    }

    fn reward_data(&mut self) -> RewardData {
        data::reward_data()
    }
    fn claim_sig(&mut self) -> Bytes {
        data::claim_sig()
    }

    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Reward Only Gauge Only Admin
            runtime::revert(Error::RewardOnlyGaugeOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&REWARDONLYGAUGEEvent::CommitOwnership { admin: addr });
    }

    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            //Reward Only Gauge Only Future Admin
            runtime::revert(Error::RewardOnlyGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&REWARDONLYGAUGEEvent::ApplyOwnership { admin: _admin });
    }

    fn reward_contract(&mut self) -> Key {
        let address = self.reward_data().address;
        if address == zero_address() {
            zero_address()
        } else {
            address
        }
    }
    fn last_claim(&mut self) -> U256 {
        self.reward_data().time_stamp
    }
    fn claimed_reward(&mut self, _addr: Key, _token: Key) -> U256 {
        self.claim_data(_addr, _token).claimed_amount
    }
    fn claimable_reward(&mut self, _addr: Key, _token: Key) -> U256 {
        self.claim_data(_addr, _token).claimable_amount
    }

    // lock
    fn claimable_reward_write(&mut self, _addr: Key, _token: Key) -> U256 {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let reward_token = self.reward_tokens(0.into());
        if reward_token != zero_address() {
            let total_supply = self.total_supply();
            self._checkpoint_rewards(_addr, total_supply, false, account_zero_address());
        }
        data::set_lock(0);
        self.claim_data(_addr, _addr).claimable_amount
    }

    // lock
    fn claim_rewards(&mut self, _addr: Option<Key>, _receiver: Option<Key>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let addr: Key;
        let receiver: Key;
        if _addr.is_none() {
            addr = self.get_caller();
        } else {
            addr = _addr.unwrap();
        }
        if _receiver.is_none() {
            receiver = zero_address();
        } else {
            receiver = _receiver.unwrap();
        }
        if receiver != zero_address() {
            if addr != self.get_caller() {
                // Reward Only Gauge Cannot Redirect When Claiming For Another User
                runtime::revert(Error::RewardOnlyGaugeCannotRedirectWhenClaimingForAnotherUser);
            }
        }
        let total_supply = self.total_supply();
        self._checkpoint_rewards(addr, total_supply, true, receiver);
        data::set_lock(0);
    }

    // lock
    fn withdraw(&mut self, _value: U256, _claim_rewards: Option<bool>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let claim_rewards: bool;
        if _claim_rewards.is_none() {
            claim_rewards = false;
        } else {
            claim_rewards = true;
        }
        if _value == 0.into() {
            // Reward Only Gauge Value Is Zero
            runtime::revert(Error::RewardOnlyGaugeValueIsZero1);
        }
        let reward_data: RewardData = self.reward_data();
        let _reward_contract: Key = reward_data.address;
        let mut total_supply = self.total_supply();
        self._checkpoint_rewards(
            self.get_caller(),
            total_supply,
            claim_rewards,
            account_zero_address(),
        );
        total_supply = total_supply
            .checked_sub(_value)
            .ok_or(Error::RewardOnlyGaugeUnderFlow5)
            .unwrap_or_revert();
        let balance = self.balance_of(self.get_caller());
        let new_balance = balance
            .checked_sub(_value)
            .ok_or(Error::RewardOnlyGaugeUnderFlow6)
            .unwrap_or_revert();
        Balances::instance().set(&self.get_caller(), new_balance);
        data::set_total_supply(total_supply);
        let lp_token = self.lp_token();
        let token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
        let _result: () = runtime::call_versioned_contract(
            token_package_hash,
            None,
            "transfer",
            runtime_args! {"_to" => self.get_caller(),"_value" => _value},
        );

        self.emit(&REWARDONLYGAUGEEvent::Withdraw {
            provider: self.get_caller(),
            value: _value,
        });
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value: _value,
        });
        data::set_lock(0);
    }
    fn deposit(&mut self, _value: U256, _addr: Option<Key>, _claim_rewards: Option<bool>) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        let claim_rewards: bool;
        if _claim_rewards.is_none() {
            claim_rewards = false;
        } else {
            claim_rewards = _claim_rewards.unwrap();
        }
        let addr: Key;
        if _addr.is_none() {
            addr = self.get_caller();
        } else {
            addr = _addr.unwrap();
        }
        if _value == 0.into() {
            // Reward Only Gauge Value Is Zero
            runtime::revert(Error::RewardOnlyGaugeValueIsZero2);
        }
        let reward_data: RewardData = self.reward_data();
        let _reward_contract: Key = reward_data.address;
        let mut total_supply = self.total_supply();
        self._checkpoint_rewards(addr, total_supply, claim_rewards, account_zero_address());
        total_supply = total_supply
            .checked_add(_value)
            .ok_or(Error::RewardOnlyGaugeOverFlow4)
            .unwrap_or_revert();
        let balance = self.balance_of(self.get_caller());
        let new_balance = balance
            .checked_add(_value)
            .ok_or(Error::RewardOnlyGaugeOverFlow5)
            .unwrap_or_revert();
        Balances::instance().set(&self.get_caller(), new_balance);
        data::set_total_supply(total_supply);
        let lp_token = self.lp_token();
        let token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
        let _result: () = runtime::call_versioned_contract(
            token_package_hash,
            None,
            "transfer_from",
            runtime_args! {"_from" => self.get_caller(),"_to" =>  data::get_package_hash(),"_value" => _value},
        );

        self.emit(&REWARDONLYGAUGEEvent::Deposit {
            provider: self.get_caller(),
            value: _value,
        });
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value: _value,
        });
        data::set_lock(0);
    }
    fn set_rewards(
        &mut self,
        _reward_contract: Key,
        _claim_sig: Bytes,
        _reward_tokens: Vec<String>,
    ) {
        let lock = data::get_lock();
        if lock != 0 {
            //Reward Only Gauge: Locked
            runtime::revert(Error::RewardOnlyGaugeLocked1);
        }
        data::set_lock(1);
        if self.get_caller() != self.admin() {
            runtime::revert(Error::RewardOnlyGaugeOnlyAdmin2);
        }
        let mut reward_tokens: Vec<Key> = Vec::new();
        for i in 0..(_reward_tokens.len()) {
            reward_tokens.push(Key::from_formatted_str(&_reward_tokens[i]).unwrap());
        }
        let _lp_token = self.lp_token();
        let _current_reward_contract = self.reward_data().address;
        let total_supply = self.total_supply();
        self._checkpoint_rewards(
            account_zero_address(),
            total_supply,
            false,
            account_zero_address(),
        );
        if _reward_contract != zero_address() {
            let reward_token = self.reward_tokens(0.into());
            if reward_token == zero_address() {
                //Reward Only Gauge Reward Token Is Zero Address
                runtime::revert(Error::RewardOnlyGaugeRewardTokenIsZeroAddress);
            }
            //is Contract Check is missing
        }
        let mut reward_data = self.reward_data();
        reward_data.address = _reward_contract;
        data::set_claim_sig(_claim_sig);
        for i in 0..(MAX_REWARDS.as_usize()) {
            let current_token = self.reward_tokens(i.into());
            let new_token: Key = reward_tokens[i];

            if current_token != zero_address() {
                if current_token != new_token {
                    //Reward Only Gauge Cannot Modify Existing Reward Token
                    runtime::revert(Error::RewardOnlyGaugeCannotModifyExistingRewardToken);
                }
            } else if new_token != zero_address() {
                RewardTokens::instance().set(&i.into(), new_token);
            } else {
                break;
            }
        }
        if _reward_contract != zero_address() {
            // # do an initial checkpoint to verify that claims are working
            self._checkpoint_rewards(
                account_zero_address(),
                total_supply,
                false,
                account_zero_address(),
            )
        }
        data::set_lock(0);
    }

    fn set_rewards_receiver(&mut self, _receiver: Key) {
        RewardsReceiver::instance().set(&self.get_caller(), _receiver)
    }

    fn lp_token(&mut self) -> Key {
        data::lp_token()
    }
    fn admin(&mut self) -> Key {
        data::admin()
    }

    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }

    /// """
    /// @notice Claim pending rewards and checkpoint rewards for a user
    /// """

    fn _checkpoint_rewards(
        &mut self,
        _user: Key,
        _total_supply: U256,
        _claim: bool,
        _receiver: Key,
    ) {
        let mut reward_data: RewardData = self.reward_data();
        if _total_supply != 0.into()
            && reward_data.address != zero_address()
            && reward_data.time_stamp != 0.into()
            && U256::from(u64::from(runtime::get_blocktime()))
                > (reward_data.time_stamp + U256::from(CLAIM_FREQUENCY.as_u128()))
        {
            let reward_contract = reward_data.address;

            // raw_call(reward_contract, self.claim_sig)

            let reward_contract_hash_add_array = match reward_contract {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let reward_contract_package_hash =
                ContractPackageHash::new(reward_contract_hash_add_array);
            let () = runtime::call_versioned_contract(
                reward_contract_package_hash,
                None,
                "claim_sig",
                runtime_args! {},
            );
            reward_data.address = reward_contract;
            reward_data.time_stamp = U256::from(u64::from(runtime::get_blocktime()))
        }
        let mut receiver = _receiver;
        if _claim && receiver == account_zero_address() {
            // # if receiver is not explicitly declared, check for default receiver
            receiver = self.rewards_receiver(_user);
            if receiver == account_zero_address() {
                receiver = _user;
            }
        }
        let user_balance = self.balance_of(_user);
        for i in 0..(8) {
            let token = self.reward_tokens(i.into());
            if token == zero_address() {
                break;
            }
            let mut d_i: U256 = 0.into();
            if _total_supply != 0.into() {
                let token_hash_add_array = match token {
                    Key::Hash(package) => package,
                    _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                };
                let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                let token_balance: U256 = runtime::call_versioned_contract(
                    token_package_hash,
                    None,
                    "balance_of",
                    runtime_args! {"owner" => data::get_package_hash()},
                );
                d_i = U256::from(1000000000)
                    * (token_balance
                        .checked_sub(self.reward_balances(token))
                        .ok_or(Error::RewardOnlyGaugeUnderFlow7)
                        .unwrap_or_revert()
                        / _total_supply);
                RewardBalances::instance().set(&token, token_balance);
                if _user == zero_address() {
                    if d_i != 0.into() {
                        let reward_integral = self.reward_integral(token);
                        RewardIntegral::instance().set(
                            &token,
                            reward_integral
                                .checked_add(d_i)
                                .ok_or(Error::RewardOnlyGaugeOverFlow2)
                                .unwrap_or_revert(),
                        )
                    }
                }
            }
            let integral = self
                .reward_integral(token)
                .checked_add(d_i)
                .ok_or(Error::RewardOnlyGaugeOverFlow3)
                .unwrap_or_revert();
            if d_i != 0.into() {
                RewardIntegral::instance().set(&token, integral);
            }
            let integral_for: U256 = self.reward_integral_for(token, _user);
            let mut new_claimable: U256 = 0.into();
            if integral_for < integral {
                RewardIntegralFor::instance().set(&token, &_user, integral);
                new_claimable = user_balance
                    * (integral
                        .checked_sub(integral_for)
                        .ok_or(Error::RewardOnlyGaugeUnderFlow8)
                        .unwrap_or_revert())
                    / U256::from(1000000000);
            }
            let mut claim_data: ClaimDataStruct = self.claim_data(_user, token);
            let total_claimable: U256 = claim_data
                .claimable_amount
                .checked_add(new_claimable)
                .ok_or(Error::RewardOnlyGaugeOverFlow6)
                .unwrap_or_revert();
            if total_claimable > 0.into() {
                let total_claimed = claim_data.claimed_amount;
                if _claim {
                    let token_hash_add_array = match token {
                        Key::Hash(package) => package,
                        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                    };
                    let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                    let () = runtime::call_versioned_contract(
                        token_package_hash,
                        None,
                        "transfer",
                        runtime_args! {"to" => receiver,"amount" => total_claimable},
                    );
                    // if len(response) != 0:
                    //     assert convert(response, bool)

                    let latest_total_claimable = self
                        .reward_balances(token)
                        .checked_sub(total_claimable)
                        .ok_or(Error::RewardOnlyGaugeUnderFlow9)
                        .unwrap_or_revert();
                    RewardBalances::instance().set(&token, latest_total_claimable);
                    claim_data.claimed_amount = total_claimed
                        .checked_add(total_claimable)
                        .ok_or(Error::RewardOnlyGaugeOverFlow7)
                        .unwrap_or_revert();
                    ClaimData::instance().set(&_user, &token, claim_data);
                } else if new_claimable > 0.into() {
                    claim_data.claimed_amount = total_claimed;
                    claim_data.claimable_amount = total_claimable;
                    ClaimData::instance().set(&_user, &token, claim_data);
                }
            }
        }
    }
    fn emit(&mut self, reward_only_gauge_event: &REWARDONLYGAUGEEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match reward_only_gauge_event {
            REWARDONLYGAUGEEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };

        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
