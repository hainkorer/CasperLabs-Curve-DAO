use crate::alloc::string::ToString;
use crate::data::{
    self, zero_address, Allowances, Balances, ClaimData, ClaimDataStruct, RewardBalances,
    RewardData, RewardIntegral, RewardIntegralFor, RewardTokens, RewardsReceiver, MAX_REWARDS,
};
use alloc::collections::BTreeMap;
use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, system::mint::Error as MintError, ApiError, BlockTime, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{set_key, ContractContext, ContractStorage};
use cryptoxide::ed25519;
use hex::encode;
use renvm_sig::{hash_message, keccak256};

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

#[repr(u16)]
pub enum Error {
    /// 65,536 for (Reward Only Gauge EXPIRED)
    RewardOnlyGaugeEXPIRED = 0,
    /// 65,537 for (Reward Only Gauge Signature Verification Failed)
    RewardOnlyGaugeSignatureVerificationFailed = 1,
    /// 65,538 for (Reward Only Gauge OverFlow1)
    RewardOnlyGaugeOverFlow1 = 2,
    /// 65,539 for (Reward Only Gauge OverFlow2)
    RewardOnlyGaugeOverFlow2 = 3,
    /// 65,540 for (Reward Only Gauge OverFlow3)
    RewardOnlyGaugeOverFlow3 = 4,
    /// 65,541 for (Reward Only Gauge OverFlow4)
    RewardOnlyGaugeOverFlow4 = 5,
    /// 65,542 for (Reward Only Gauge UnderFlow1)
    RewardOnlyGaugeUnderFlow1 = 6,
    /// 65,543 for (Reward Only Gauge UnderFlow2)
    RewardOnlyGaugeUnderFlow2 = 7,
    /// 65,544 for (Reward Only Gauge UnderFlow3)
    RewardOnlyGaugeUnderFlow3 = 8,
    /// 65,545 for (Reward Only Gauge UnderFlow4)
    RewardOnlyGaugeUnderFlow4 = 9,
    /// 65,546 for (Reward Only Gauge UnderFlow5)
    RewardOnlyGaugeUnderFlow5 = 10,
    /// 65,540 for (Reward Only Gauge Only Admin1)
    RewardOnlyGaugeOnlyAdmin1 = 11,
    /// 65,540 for (Reward Only Gauge Only Admin2)
    RewardOnlyGaugeOnlyAdmin2 = 12,
    /// 65,540 for (Reward Only Gauge Only Future Admin)
    RewardOnlyGaugeOnlyFutureAdmin = 13,
    /// 65,540 for (Reward Only Gauge Cannot Redirect When Claiming For Another User)
    RewardOnlyGaugeCannotRedirectWhenClaimingForAnotherUser = 14,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero1 = 15,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero2 = 16,
    /// 65,540 for (Reward Only Gauge Reward Token Is Zero)
    RewardOnlyGaugeRewardTokenIsZeroAddress = 17,
    /// 65,540 for (Reward Only Gauge Cannot Modify Existing Reward Token)
    RewardOnlyGaugeCannotModifyExistingRewardToken = 18,
    /// 65,540 for (Reward Only Gauge Receiver Is Zero Address)
    RewardOnlyGaugeReceiverIsZeroAddress = 19,
    // /// 65,538 for (Gauge Controller Address Zero1)
    // RewardOnlyGaugeAddressZero1 = 2,
    // /// 65,539 for (Gauge Controller Address Zero2)
    // RewardOnlyGaugeAddressZero2 = 3,
    // /// 65,540 for (Gauge Controller Only Admin1)
    // RewardOnlyGaugeOnlyAdmin1 = 4,
    // /// 65,541 for (Gauge Controller Only Admin2)
    // RewardOnlyGaugeOnlyAdmin2 = 5,
    // /// 65,542 for (Gauge Controller Admin Not Set)
    // RewardOnlyGaugeAdminNotSet = 6,
    // /// 65,543 for (Gauge Controller Gauge Type Is Zero)
    // RewardOnlyGaugeGaugeTypeIsZero = 7,
    // /// 65,544 for (Gauge Controller Not Admin1)
    // RewardOnlyGaugeNotAdmin1 = 8,
    // /// 65,545 for (Gauge Controller Not Admin2)
    // RewardOnlyGaugeNotAdmin2 = 9,
    // /// 65,546 for (Gauge Controller Not Admin3)
    // RewardOnlyGaugeNotAdmin3 = 10,
    // /// 65,547 for (Gauge Controller Not Admin3)
    // RewardOnlyGaugeNotAdmin4 = 11,
    // /// 65,548 for (Gauge Controller cannot add same gauge twice)
    // RewardOnlyGaugeCannotAddSameGaugeTwice = 12,
    // /// 65,549 for (Gauge Controller gauge type is greater than equal to zero and less than n_gauge_types)
    // RewardOnlyGaugeGaugeType1 = 13,
    // /// 65,550 for (Gauge Controller Your token lock expires too soon)
    // RewardOnlyGaugeTokenLockExpiresTooSoon = 14,
    // /// 65,551 for (Gauge Controller You used all your voting power)
    // RewardOnlyGaugeUsedAllYourVotingPower = 15,
    // /// 65,552 for (Gauge Controller You Cannot vote so often)
    // RewardOnlyGaugeCannotVoteSoOften = 16,
    // /// 65,553 for (Gauge Controller Gauge not added)
    // RewardOnlyGaugeGaugeNotAdded = 17,
    // /// 65,554 for (Gauge Controller Used too much power)
    // RewardOnlyGaugeUsedTooMuchPower = 18,
    // /// 65,555 for (Gauge Controller OverFlow1)
    // RewardOnlyGaugeOverFlow1 = 19,
    // /// 65,556 for (Gauge Controller OverFlow2)
    // RewardOnlyGaugeOverFlow2 = 20,
    // /// 65,557 for (Gauge Controller OverFlow3)
    // RewardOnlyGaugeOverFlow3 = 21,
    // /// 65,558 for (Gauge Controller OverFlow4)
    // RewardOnlyGaugeOverFlow4 = 22,
    // /// 65,559 for (Gauge Controller OverFlow5)
    // RewardOnlyGaugeOverFlow5 = 23,
    // /// 65,560 for (Gauge Controller OverFlow6)
    // RewardOnlyGaugeOverFlow6 = 24,
    // /// 65,561 for (Gauge Controller OverFlow7)
    // RewardOnlyGaugeOverFlow7 = 25,
    // /// 65,562 for (Gauge Controller OverFlow8)
    // RewardOnlyGaugeOverFlow8 = 26,
    // /// 65,563 for (Gauge Controller OverFlow9)
    // RewardOnlyGaugeOverFlow9 = 27,
    // /// 65,564 for (Gauge Controller OverFlow10)
    // RewardOnlyGaugeOverFlow10 = 28,
    // /// 65,565 for (Gauge Controller OverFlow11)
    // RewardOnlyGaugeOverFlow11 = 29,
    // /// 65,566 for (Gauge Controller OverFlow12)
    // RewardOnlyGaugeOverFlow12 = 30,
    // /// 65,567 for (Gauge Controller OverFlow13)
    // RewardOnlyGaugeOverFlow13 = 31,
    // /// 65,568 for (Gauge Controller OverFlow14)
    // RewardOnlyGaugeOverFlow14 = 32,
    // /// 65,569 for (Gauge Controller OverFlow15)
    // RewardOnlyGaugeOverFlow15 = 33,
    // /// 65,570 for (Gauge Controller OverFlow16)
    // RewardOnlyGaugeOverFlow16 = 34,
    // /// 65,571 for (Gauge Controller OverFlow17)
    // RewardOnlyGaugeOverFlow17 = 35,
    // /// 65,572 for (Gauge Controller OverFlow18)
    // RewardOnlyGaugeOverFlow18 = 36,
    // /// 65,573 for (Gauge Controller OverFlow19)
    // RewardOnlyGaugeOverFlow19 = 37,
    // /// 65,574 for (Gauge Controller OverFlow20)
    // RewardOnlyGaugeOverFlow20 = 38,
    // /// 65,575 for (Gauge Controller OverFlow21)
    // RewardOnlyGaugeOverFlow21 = 39,
    // /// 65,576 for (Gauge Controller OverFlow22)
    // RewardOnlyGaugeOverFlow22 = 40,
    // /// 65,577 for (Gauge Controller OverFlow23)
    // RewardOnlyGaugeOverFlow23 = 41,
    // /// 65,578 for (Gauge Controller OverFlow24)
    // RewardOnlyGaugeOverFlow24 = 42,
    // /// 65,579 for (Gauge Controller OverFlow25)
    // RewardOnlyGaugeOverFlow25 = 43,
    // /// 65,580 for (Gauge Controller OverFlow26)
    // RewardOnlyGaugeOverFlow26 = 44,
    // /// 65,581 for (Gauge Controller OverFlow27)
    // RewardOnlyGaugeOverFlow27 = 45,
    // /// 65,582 for (Gauge Controller UnderFlow1)
    // RewardOnlyGaugeUnderFlow1 = 46,
    // /// 65,583 for (Gauge Controller UnderFlow2)
    // RewardOnlyGaugeUnderFlow2 = 47,
    // /// 65,584 for (Gauge Controller UnderFlow3)
    // RewardOnlyGaugeUnderFlow3 = 48,
    // /// 65,585 for (Gauge Controller UnderFlow4)
    // RewardOnlyGaugeUnderFlow4 = 49,
    // /// 65,586 for (Gauge Controller UnderFlow5)
    // RewardOnlyGaugeUnderFlow5 = 50,
    // /// 65,587 for (Gauge Controller UnderFlow6)
    // RewardOnlyGaugeUnderFlow6 = 51,
    // /// 65,588 for (Gauge Controller UnderFlow7)
    // RewardOnlyGaugeUnderFlow7 = 52,
    // /// 65,589 for (Gauge Controller UnderFlow8)
    // RewardOnlyGaugeUnderFlow8 = 53,
    // /// 65,590 for (Gauge Controller UnderFlow9)
    // RewardOnlyGaugeUnderFlow9 = 54,
    // /// 65,591 for (Gauge Controller UnderFlow10)
    // RewardOnlyGaugeUnderFlow10 = 55,
    // /// 65,592 for (Gauge Controller UnderFlow11)
    // RewardOnlyGaugeUnderFlow11 = 56,
    // /// 65,593 for (Gauge Controller UnderFlow12)
    // RewardOnlyGaugeUnderFlow12 = 57,
    // /// 65,594 for (Gauge Controller UnderFlow13)
    // RewardOnlyGaugeUnderFlow13 = 58,
    // /// 65,595 for (Gauge Controller UnderFlow14)
    // RewardOnlyGaugeUnderFlow14 = 59,
    // /// 65,596 for (Gauge Controller UnderFlow15)
    // RewardOnlyGaugeUnderFlow15 = 60,
    // /// 65,597 for (Gauge Controller UnderFlow16)
    // RewardOnlyGaugeUnderFlow16 = 61,
    // /// 65,598 for (Gauge Controller UnderFlow17)
    // RewardOnlyGaugeUnderFlow17 = 62,
    // /// 65,599 for (Gauge Controller UnderFlow18)
    // RewardOnlyGaugeUnderFlow18 = 63,
    // /// 65,600 for (Gauge Controller UnderFlow19)
    // RewardOnlyGaugeUnderFlow19 = 64,
    // /// 65,601 for (Gauge Controller UnderFlow20)
    // RewardOnlyGaugeUnderFlow20 = 65,
    // /// 65,602 for (Gauge Controller UnderFlow21)
    // RewardOnlyGaugeUnderFlow21 = 66,
    // /// 65,603 for (Gauge Controller UnderFlow22)
    // RewardOnlyGaugeUnderFlow22 = 67,
    // /// 65,604 for (Gauge Controller UnderFlow23)
    // RewardOnlyGaugeUnderFlow23 = 68,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
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
        self._transfer(self.get_caller(), _to, _value);
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
        let allowances = Allowances::instance();
        let _allowance: U256 = allowances.get(&_from, &self.get_caller());
        if _allowance != U256::MAX {
            let new_allowance: U256 = _allowance
                .checked_sub(_value)
                .ok_or(Error::RewardOnlyGaugeUnderFlow2)
                .unwrap_or_revert();
            self._approve(_from, self.get_caller(), new_allowance);
            // allowances.set(&, &, new_allowance);
        }
        self._transfer(_from, _to, _value);
        return true;
    }

    fn _transfer(&mut self, _from: Key, _to: Key, _value: U256) {
        let reward_data: RewardData = self.reward_data();
        let reward_contract = reward_data.address;
        if _value != 0.into() {
            let total_supply = self.total_supply();
            // self._chechkpoint_rewards(_from, total_supply, false, zero_address());
            let balances: Balances = Balances::instance();
            let _from_balance: U256 = balances.get(&_from);
            let from_new_balance = _from_balance
                .checked_sub(_value)
                .ok_or(Error::RewardOnlyGaugeUnderFlow5)
                .unwrap_or_revert();
            balances.set(&_from, from_new_balance);

            // self._chechkpoint_rewards(_to, total_supply, false, zero_address());
            let balances: Balances = Balances::instance();
            let _to_balance: U256 = balances.get(&_to);
            let to_new_balance = _from_balance
                .checked_sub(_value)
                .ok_or(Error::RewardOnlyGaugeUnderFlow5)
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
        let reward_token = self.reward_tokens(0.into());
        if reward_token != zero_address() {
            let total_supply = self.total_supply();
            // self._chechkpoint_rewards(_addr, total_supply, false, zero_address());
        }
        self.claim_data(_addr, _addr).claimable_amount
    }

    // lock
    fn claim_rewards(&mut self, _addr: Option<Key>, _receiver: Option<Key>) {
        let mut addr: Key;
        let mut receiver: Key;
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
        self._checkpoint_rewards(addr, total_supply, true, receiver)
    }

    // lock
    fn withdraw(&mut self, _value: U256, _claim_rewards: Option<bool>) {
        let mut claim_rewards: bool;
        if _claim_rewards.is_none() {
            claim_rewards = false;
        } else {
            claim_rewards = true;
        }
        if _value == 0.into() {
            // Reward Only Gauge Value Is Zero
            runtime::revert(Error::RewardOnlyGaugeValueIsZero1);
        }
        let reward_Data: RewardData = self.reward_data();
        let reward_contract: Key = reward_Data.address;
        let mut total_supply = self.total_supply();
        self._checkpoint_rewards(
            self.get_caller(),
            total_supply,
            claim_rewards,
            zero_address(),
        );
        total_supply = total_supply
            .checked_sub(_value)
            .ok_or(Error::RewardOnlyGaugeUnderFlow5)
            .unwrap_or_revert();
        let balance = self.balance_of(self.get_caller());
        let new_balance = balance
            .checked_sub(_value)
            .ok_or(Error::RewardOnlyGaugeUnderFlow5)
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
    }
    fn deposit(&mut self, _value: U256, _addr: Option<Key>, _claim_rewards: Option<bool>) {
        let mut claim_rewards: bool;
        if _claim_rewards.is_none() {
            claim_rewards = false;
        } else {
            claim_rewards = _claim_rewards.unwrap();
        }
        let mut addr: Key;
        if _addr.is_none() {
            addr = self.get_caller();
        } else {
            addr = _addr.unwrap();
        }
        if _value == 0.into() {
            // Reward Only Gauge Value Is Zero
            runtime::revert(Error::RewardOnlyGaugeValueIsZero2);
        }
        let reward_Data: RewardData = self.reward_data();
        let reward_contract: Key = reward_Data.address;
        let mut total_supply = self.total_supply();
        self._checkpoint_rewards(addr, total_supply, claim_rewards, zero_address());
        total_supply = total_supply
            .checked_add(_value)
            .ok_or(Error::RewardOnlyGaugeOverFlow4)
            .unwrap_or_revert();
        let balance = self.balance_of(self.get_caller());
        let new_balance = balance
            .checked_add(_value)
            .ok_or(Error::RewardOnlyGaugeOverFlow4)
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
            runtime_args! {"_from" => self.get_caller(),"_to" => runtime::get_caller(),"_value" => _value},
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
    }
    fn set_rewards(
        &mut self,
        _reward_contract: Key,
        _claim_sig: Bytes,
        _reward_tokens: Vec<String>,
    ) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::RewardOnlyGaugeOnlyAdmin2);
        }
        let mut reward_tokens: Vec<Key> = Vec::new();
        for i in 0..(_reward_tokens.len()) {
            reward_tokens.push(Key::from_formatted_str(&_reward_tokens[i]).unwrap());
        }
        let lp_token = self.lp_token();
        let current_reward_contract = self.reward_data().address;
        let total_supply = self.total_supply();
        self._checkpoint_rewards(zero_address(), total_supply, false, zero_address());
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
        for i in 0..(8) {
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
            self._checkpoint_rewards(zero_address(), total_supply, false, zero_address())
        }
        // RewardsReceiver::instance().set(&self.get_caller(), _receuver)
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
                > (reward_data.time_stamp + U256::from(3600))
        {
            let reward_contract = reward_data.address;
            // raw_call(reward_contract, self.claim_sig)
            reward_data.address = reward_contract;
            reward_data.time_stamp = U256::from(u64::from(runtime::get_blocktime()))
        }
        let mut receiver = _receiver;
        if _claim && receiver == zero_address() {
            // # if receiver is not explicitly declared, check for default receiver
            receiver = self.rewards_receiver(_user);
            if receiver == zero_address() {
                receiver = _user;
            }
            // //Reward Only Gauge Reciver is Zero Address
            // runtime::revert(Error::RewardOnlyGaugeReceiverIsZeroAddress);
        }
        let user_balance = self.balance_of(_user);
        for i in 0..(8) {
            let token = self.reward_tokens(i.into());
            if token == zero_address() {
                break;
            }
            let mut dI: U256 = 0.into();
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
                    runtime_args! {"owner" => runtime::get_caller()},
                );
                dI = U256::from(1000000000)
                    * (token_balance
                        .checked_sub(self.reward_balances(token))
                        .ok_or(Error::RewardOnlyGaugeUnderFlow1)
                        .unwrap_or_revert()
                        / _total_supply);
                RewardBalances::instance().set(&token, token_balance);
                if _user == zero_address() {
                    if dI != 0.into() {
                        let reward_integral = self.reward_integral(token);
                        RewardIntegral::instance().set(
                            &token,
                            reward_integral
                                .checked_add(dI)
                                .ok_or(Error::RewardOnlyGaugeOverFlow1)
                                .unwrap_or_revert(),
                        )
                    }
                }
            }
            let integral = self
                .reward_integral(token)
                .checked_add(dI)
                .ok_or(Error::RewardOnlyGaugeOverFlow1)
                .unwrap_or_revert();
            if dI != 0.into() {
                RewardIntegral::instance().set(&token, integral);
            }
            let integral_for: U256 = self.reward_integral_for(token, _user);
            let mut new_claimable: U256 = 0.into();
            if integral_for < integral {
                RewardIntegralFor::instance().set(&token, &_user, integral);
                new_claimable = user_balance
                    * (integral
                        .checked_sub(integral_for)
                        .ok_or(Error::RewardOnlyGaugeUnderFlow1)
                        .unwrap_or_revert())
                    / U256::from(1000000000);
            }
            let mut claim_data: ClaimDataStruct = self.claim_data(_user, token);
            let total_claimable: U256 = claim_data
                .claimable_amount
                .checked_add(new_claimable)
                .ok_or(Error::RewardOnlyGaugeOverFlow1)
                .unwrap_or_revert();
            if total_claimable > 0.into() {
                let total_claimed = claim_data.claimed_amount;
                if _claim {
                    // response: Bytes[32] = raw_call(
                    //     token,
                    //     concat(
                    //         method_id("transfer(address,uint256)"),
                    //         convert(receiver, bytes32),
                    //         convert(total_claimable, bytes32),
                    //     ),
                    //     max_outsize=32,
                    // )
                    // if len(response) != 0:
                    //     assert convert(response, bool)

                    let latest_total_claimable = self
                        .reward_balances(token)
                        .checked_sub(total_claimable)
                        .ok_or(Error::RewardOnlyGaugeUnderFlow1)
                        .unwrap_or_revert();
                    RewardBalances::instance().set(&token, latest_total_claimable);
                    claim_data.claimed_amount = total_claimed
                        .checked_add(total_claimable)
                        .ok_or(Error::RewardOnlyGaugeOverFlow1)
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
