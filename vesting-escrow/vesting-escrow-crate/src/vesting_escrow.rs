use crate::alloc::string::ToString;
use crate::data::{
    self, account_zero_address, zero_address, DisabledAt, FundAdmins, InitialLocked, TotalClaimed,
};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::bytesrepr::Bytes;
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};

pub enum VESTINGESCROWEvent {
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

impl VESTINGESCROWEvent {
    pub fn type_name(&self) -> String {
        match self {
            VESTINGESCROWEvent::Withdraw {
                provider: _,
                value: _,
            } => "withdraw",
            VESTINGESCROWEvent::Deposit {
                provider: _,
                value: _,
            } => "deposit",
            VESTINGESCROWEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            VESTINGESCROWEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            VESTINGESCROWEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            VESTINGESCROWEvent::Transfer {
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
    /// 65,538 for (Reward Only Gauge OverFlow1)
    RewardOnlyGaugeOverFlow1 = 0,
    /// 65,539 for (Reward Only Gauge OverFlow2)
    RewardOnlyGaugeOverFlow2 = 1,
    /// 65,540 for (Reward Only Gauge OverFlow3)
    RewardOnlyGaugeOverFlow3 = 2,
    /// 65,541 for (Reward Only Gauge OverFlow4)
    RewardOnlyGaugeOverFlow4 = 3,
    /// 65,541 for (Reward Only Gauge OverFlow5)
    RewardOnlyGaugeOverFlow5 = 4,
    /// 65,541 for (Reward Only Gauge OverFlow6)
    RewardOnlyGaugeOverFlow6 = 5,
    /// 65,541 for (Reward Only Gauge OverFlow7)
    RewardOnlyGaugeOverFlow7 = 6,
    /// 65,542 for (Reward Only Gauge UnderFlow1)
    RewardOnlyGaugeUnderFlow1 = 7,
    /// 65,543 for (Reward Only Gauge UnderFlow2)
    RewardOnlyGaugeUnderFlow2 = 8,
    /// 65,544 for (Reward Only Gauge UnderFlow3)
    RewardOnlyGaugeUnderFlow3 = 9,
    /// 65,545 for (Reward Only Gauge UnderFlow4)
    RewardOnlyGaugeUnderFlow4 = 10,
    /// 65,546 for (Reward Only Gauge UnderFlow5)
    RewardOnlyGaugeUnderFlow5 = 12,
    /// 65,546 for (Reward Only Gauge UnderFlow6)
    RewardOnlyGaugeUnderFlow6 = 13,
    /// 65,546 for (Reward Only Gauge UnderFlow7)
    RewardOnlyGaugeUnderFlow7 = 14,
    /// 65,546 for (Reward Only Gauge UnderFlow8)
    RewardOnlyGaugeUnderFlow8 = 15,
    /// 65,546 for (Reward Only Gauge UnderFlow9)
    RewardOnlyGaugeUnderFlow9 = 16,
    /// 65,540 for (Reward Only Gauge Only Admin1)
    RewardOnlyGaugeOnlyAdmin1 = 17,
    /// 65,540 for (Reward Only Gauge Only Admin2)
    RewardOnlyGaugeOnlyAdmin2 = 18,
    /// 65,540 for (Reward Only Gauge Only Future Admin)
    RewardOnlyGaugeOnlyFutureAdmin = 19,
    /// 65,540 for (Reward Only Gauge Cannot Redirect When Claiming For Another User)
    RewardOnlyGaugeCannotRedirectWhenClaimingForAnotherUser = 20,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero1 = 21,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero2 = 22,
    /// 65,540 for (Reward Only Gauge Reward Token Is Zero)
    RewardOnlyGaugeRewardTokenIsZeroAddress = 23,
    /// 65,540 for (Reward Only Gauge Cannot Modify Existing Reward Token)
    RewardOnlyGaugeCannotModifyExistingRewardToken = 24,
    /// 65,540 for (Reward Only Gauge Receiver Is Zero Address)
    RewardOnlyGaugeLocked1 = 25,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait VESTINGESCROW<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        _token: Key,
        _start_time: U256,
        _end_time: U256,
        _can_disable: bool,
        _fund_admins: Vec<String>,
        contract_hash: Key,
        package_hash: ContractPackageHash,
        lock: u64,
    ) {
        data::set_token(_token);
        data::set_admin(self.get_caller());
        data::set_start_time(_start_time);
        data::set_end_time(_end_time);
        data::set_can_disable(_can_disable);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_lock(lock);

        InitialLocked::init();
        TotalClaimed::init();
        DisabledAt::init();
        FundAdmins::init();
    }

    fn initial_locked(&mut self, owner: Key) -> U256 {
        InitialLocked::instance().get(&owner)
    }
    fn total_claimed(&mut self, owner: Key) -> U256 {
        TotalClaimed::instance().get(&owner)
    }
    fn disabled_at(&mut self, owner: Key) -> U256 {
        DisabledAt::instance().get(&owner)
    }
    fn fund_admins(&mut self, owner: Key) -> bool {
        FundAdmins::instance().get(&owner)
    }

    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Reward Only Gauge Only Admin
            runtime::revert(Error::RewardOnlyGaugeOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&VESTINGESCROWEvent::CommitOwnership { admin: addr });
    }

    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            //Reward Only Gauge Only Future Admin
            runtime::revert(Error::RewardOnlyGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&VESTINGESCROWEvent::ApplyOwnership { admin: _admin });
    }

    fn admin(&mut self) -> Key {
        data::admin()
    }

    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }

    fn can_disable(&mut self) -> bool {
        data::can_disable()
    }
    fn fund_admins_enabled(&mut self) -> bool {
        data::fund_admins_enabled()
    }
    fn unallocated_supply(&mut self) -> U256 {
        data::unallocated_supply()
    }
    fn initial_locked_supply(&mut self) -> U256 {
        data::initial_locked_supply()
    }
    fn end_time(&mut self) -> U256 {
        data::end_time()
    }
    fn start_time(&mut self) -> U256 {
        data::start_time()
    }
    fn token(&mut self) -> Key {
        data::token()
    }

    fn emit(&mut self, vesting_escrow_event: &VESTINGESCROWEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match vesting_escrow_event {
            VESTINGESCROWEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            VESTINGESCROWEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            VESTINGESCROWEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            VESTINGESCROWEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            VESTINGESCROWEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            VESTINGESCROWEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_event.type_name());
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
