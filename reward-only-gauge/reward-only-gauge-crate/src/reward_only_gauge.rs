use crate::alloc::string::ToString;
use crate::data::{
    self,
    Allowances,
    Balances,
    ClaimData,
    ClaimSig,
    RewardBalances,
    RewardIntegral,
    RewardIntegralFor,
    // DECIMALS,
    // admin, n_gauge_types, n_gauges, time_total, voting_escrow, ChangeSum, ChangesWeight,
    // GaugeTypeNames, GaugeTypes_, Gauges, LastUserVote, Point, PointsSum, PointsTotal,
    // PointsTypeWeight, PointsWeight, TimeSum, TimeTypeWeight, TimeWeight, VoteUserPower,
    // VoteUserSlopes, VotedSlope,
    // MULTIPLIER,
    // WEEK,
    // WEIGHT_VOTE_DELAY,
    RewardTokens,
    RewardsReceiver,
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
    Minted {
        recipient: Key,
        gauge: Key,
        minted: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    NewTypeWeight {
        type_id: U128,
        time: U256,
        weight: U256,
        total_weight: U256,
    },
    NewGaugeWeight {
        gauge_address: Key,
        time: U256,
        weight: U256,
        total_weight: U256,
    },
    AddType {
        name: String,
        type_id: U128,
    },
    VoteForGauge {
        time: U256,
        user: Key,
        gauge_addr: Key,
        weight: U256,
    },
    NewGauge {
        addr: Key,
        gauge_type: U128,
        weight: U256,
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
            REWARDONLYGAUGEEvent::Minted {
                recipient: _,
                gauge: _,
                minted: _,
            } => "minted",
            REWARDONLYGAUGEEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            REWARDONLYGAUGEEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            REWARDONLYGAUGEEvent::NewTypeWeight {
                type_id: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewTypeWeight",
            REWARDONLYGAUGEEvent::NewGaugeWeight {
                gauge_address: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewGaugeWeight",
            REWARDONLYGAUGEEvent::AddType {
                name: _,
                type_id: _,
            } => "AddType",
            REWARDONLYGAUGEEvent::VoteForGauge {
                time: _,
                user: _,
                gauge_addr: _,
                weight: _,
            } => "VoteForGauge",
            REWARDONLYGAUGEEvent::NewGauge {
                addr: _,
                gauge_type: _,
                weight: _,
            } => "NewGauge",
            REWARDONLYGAUGEEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            REWARDONLYGAUGEEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "erc20_transfer",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,536 for (UniswapV2 Core ERC20 EXPIRED)
    RewardOnlyGaugeEXPIRED = 0,
    /// 65,537 for (UniswapV2 Core ERC20 Signature Verification Failed)
    RewardOnlyGaugeSignatureVerificationFailed = 1,
    /// 65,538 for (UniswapV2 Core ERC20 OverFlow1)
    RewardOnlyGaugeOverFlow1 = 2,
    /// 65,539 for (UniswapV2 Core ERC20 OverFlow2)
    RewardOnlyGaugeOverFlow2 = 3,
    /// 65,540 for (UniswapV2 Core ERC20 OverFlow3)
    RewardOnlyGaugeOverFlow3 = 4,
    /// 65,541 for (UniswapV2 Core ERC20 OverFlow4)
    RewardOnlyGaugeOverFlow4 = 5,
    /// 65,542 for (UniswapV2 Core ERC20 UnderFlow1)
    RewardOnlyGaugeUnderFlow1 = 6,
    /// 65,543 for (UniswapV2 Core ERC20 UnderFlow2)
    RewardOnlyGaugeUnderFlow2 = 7,
    /// 65,544 for (UniswapV2 Core ERC20 UnderFlow3)
    RewardOnlyGaugeUnderFlow3 = 8,
    /// 65,545 for (UniswapV2 Core ERC20 UnderFlow4)
    RewardOnlyGaugeUnderFlow4 = 9,
    /// 65,546 for (UniswapV2 Core ERC20 UnderFlow5)
    RewardOnlyGaugeUnderFlow5 = 10,
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
        ClaimSig::init();
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

    fn claim_sig(&mut self, index: U256) -> Bytes {
        ClaimSig::instance().get(&index)
    }

    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        self.make_transfer(self.get_caller(), recipient, amount)
    }

    fn approve(&mut self, spender: Key, amount: U256) {
        self._approve(self.get_caller(), spender, amount);
    }

    fn _approve(&mut self, owner: Key, spender: Key, amount: U256) {
        Allowances::instance().set(&owner, &spender, amount);
        self.emit(&REWARDONLYGAUGEEvent::Approval {
            owner: owner,
            spender: spender,
            value: amount,
        });
    }

    fn allowance(&mut self, owner: Key, spender: Key) -> U256 {
        Allowances::instance().get(&owner, &spender)
    }

    fn reward_integral_for(&mut self, reward_token: Key, claiming_address: Key) -> U256 {
        RewardIntegralFor::instance().get(&reward_token, &claiming_address)
    }

    fn claim_data(&mut self, user: Key, claiming_address: Key) -> U256 {
        ClaimData::instance().get(&user, &claiming_address)
    }

    fn increase_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();
        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);
        let new_allowance: U256 = spender_allowance
            .checked_add(amount)
            .ok_or(Error::RewardOnlyGaugeOverFlow1)
            .unwrap_or_revert();

        if owner != spender {
            self._approve(owner, spender, new_allowance);
            return Ok(());
        } else {
            return Err(4);
        }
    }

    fn decrease_allowance(&mut self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowances = Allowances::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &spender);

        let new_allowance: U256 = spender_allowance
            .checked_sub(amount)
            .ok_or(Error::RewardOnlyGaugeUnderFlow1)
            .unwrap_or_revert();

        if new_allowance >= 0.into() && new_allowance < spender_allowance && owner != spender {
            self._approve(owner, spender, new_allowance);
            return Ok(());
        } else {
            return Err(4);
        }
    }

    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        let ret: Result<(), u32> = self.make_transfer(owner, recipient, amount);
        if ret.is_ok() {
            let allowances = Allowances::instance();
            let spender_allowance: U256 = allowances.get(&owner, &self.get_caller());
            let new_allowance: U256 = spender_allowance
                .checked_sub(amount)
                .ok_or(Error::RewardOnlyGaugeUnderFlow2)
                .unwrap_or_revert();
            if new_allowance >= 0.into()
                && new_allowance < spender_allowance
                && owner != self.get_caller()
            {
                self._approve(owner, self.get_caller(), new_allowance);
                return Ok(());
            } else {
                return Err(4);
            }
        }
        ret
    }
    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        if sender == recipient {
            return Err(4); // Same sender recipient error
        }

        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }

        let balances: Balances = Balances::instance();
        let sender_balance: U256 = balances.get(&sender);
        let recipient_balance: U256 = balances.get(&recipient);
        balances.set(
            &sender,
            sender_balance
                .checked_sub(amount)
                .ok_or(Error::RewardOnlyGaugeUnderFlow5)
                .unwrap_or_revert(),
        );
        balances.set(
            &recipient,
            recipient_balance
                .checked_add(amount)
                .ok_or(Error::RewardOnlyGaugeOverFlow4)
                .unwrap_or_revert(),
        );
        self.emit(&REWARDONLYGAUGEEvent::Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
        Ok(())
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

    fn reward_data(&mut self) -> U256 {
        data::reward_data()
    }

    // fn commit_transfer_ownership(&mut self, addr: Key) {
    //     if self.get_caller() != self.admin() {
    //         //Gauge Controller Only Admin
    //         runtime::revert(Error::RewardOnlyGaugeOnlyAdmin1);
    //     }
    //     data::set_future_admin(addr);
    //     self.emit(&REWARDONLYGAUGEEvent::CommitOwnership { admin: addr });
    // }
    // fn apply_transfer_ownership(&mut self) {
    //     if self.get_caller() != self.admin() {
    //         //Gauge Controller Only Admin
    //         runtime::revert(Error::RewardOnlyGaugeOnlyAdmin2);
    //     }
    //     let _admin = self.future_admin();
    //     if _admin == data::zero_address() {
    //         //Gauge Controller Admin Not Set
    //         runtime::revert(Error::RewardOnlyGaugeAdminNotSet);
    //     }
    //     data::set_admin(_admin);
    //     self.emit(&REWARDONLYGAUGEEvent::ApplyOwnership { admin: _admin });
    // }

    fn lp_token(&mut self) -> Key {
        data::lp_token()
    }
    fn admin(&mut self) -> Key {
        data::admin()
    }

    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }

    fn emit(&mut self, reward_only_gauge_event: &REWARDONLYGAUGEEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match reward_only_gauge_event {
            REWARDONLYGAUGEEvent::Minted {
                recipient,
                gauge,
                minted,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("gauge", gauge.to_string());
                event.insert("minted", minted.to_string());
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
            REWARDONLYGAUGEEvent::NewTypeWeight {
                type_id,
                time,
                weight,
                total_weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("type_id", type_id.to_string());
                event.insert("time", time.to_string());
                event.insert("weight", weight.to_string());
                event.insert("total_weight", total_weight.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::NewGaugeWeight {
                gauge_address,
                time,
                weight,
                total_weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("gauge_address", gauge_address.to_string());
                event.insert("time", time.to_string());
                event.insert("weight", weight.to_string());
                event.insert("total_weight", total_weight.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::AddType { name, type_id } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("name", name.to_string());
                event.insert("type_id", type_id.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::NewGauge {
                addr,
                gauge_type,
                weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("addr", addr.to_string());
                event.insert("gauge_type", gauge_type.to_string());
                event.insert("weight", weight.to_string());
                events.push(event);
            }
            REWARDONLYGAUGEEvent::VoteForGauge {
                time,
                user,
                gauge_addr,
                weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", reward_only_gauge_event.type_name());
                event.insert("time", time.to_string());
                event.insert("user", user.to_string());
                event.insert("gauge_addr", gauge_addr.to_string());
                event.insert("weight", weight.to_string());
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
