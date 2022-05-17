use crate::alloc::string::ToString;
use crate::data::{self, account_zero_address, zero_address};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};

pub enum VESTINGESCROWFACTORYEvent {
    CommitOwnership { admin: Key },
    ApplyOwnership { admin: Key },
}

impl VESTINGESCROWFACTORYEvent {
    pub fn type_name(&self) -> String {
        match self {
            VESTINGESCROWFACTORYEvent::CommitOwnership { admin: _ } => "CommitOwnership",
            VESTINGESCROWFACTORYEvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,538 for (Vesting Escrow OverFlow1)
    VestingEscrowOverFlow1 = 0,
    /// 65,539 for (Vesting Escrow OverFlow2)
    VestingEscrowOverFlow2 = 1,
    /// 65,540 for (Vesting Escrow OverFlow3)
    VestingEscrowOverFlow3 = 2,
    /// 65,541 for (Vesting Escrow OverFlow4)
    VestingEscrowOverFlow4 = 3,
    /// 65,541 for (Vesting Escrow OverFlow5)
    VestingEscrowOverFlow5 = 4,
    /// 65,542 for (Vesting Escrow UnderFlow1)
    VestingEscrowUnderFlow1 = 7,
    /// 65,543 for (Vesting Escrow UnderFlow2)
    VestingEscrowUnderFlow2 = 8,
    /// 65,544 for (Vesting Escrow UnderFlow3)
    VestingEscrowUnderFlow3 = 9,
    /// 65,545 for (Vesting Escrow UnderFlow4)
    VestingEscrowUnderFlow4 = 10,
    /// 65,546 for (Vesting Escrow UnderFlow5)
    VestingEscrowUnderFlow5 = 12,
    /// 65,546 for (Vesting Escrow UnderFlow6)
    VestingEscrowUnderFlow6 = 13,
    /// 65,546 for (Vesting Escrow UnderFlow7)
    VestingEscrowUnderFlow7 = 15,
    /// 65,546 for (Vesting Escrow UnderFlow8)
    VestingEscrowUnderFlow8 = 16,
    /// 65,546 for (Vesting Escrow UnderFlow9)
    VestingEscrowUnderFlow9 = 17,
    /// 65,546 for (Vesting Escrow UnderFlow10)
    VestingEscrowUnderFlow10 = 18,
    /// 65,546 for (Vesting Escrow UnderFlow11)
    VestingEscrowUnderFlow11 = 19,
    /// 65,546 for (Vesting Escrow UnderFlow12)
    VestingEscrowUnderFlow12 = 20,
    /// 65,546 for (Vesting Escrow UnderFlow13)
    VestingEscrowUnderFlow13 = 21,
    /// 65,546 for (Vesting Escrow Cannot Disable)
    VestingEscrowCannotDisable = 22,
    /// 65,540 for (Vesting Escrow Only Admin1)
    VestingEscrowOnlyAdmin1 = 23,
    /// 65,540 for (Vesting Escrow Only Admin2)
    VestingEscrowOnlyAdmin2 = 24,
    /// 65,540 for (Vesting Escrow Only Admin3)
    VestingEscrowOnlyAdmin3 = 25,
    /// 65,540 for (Vesting Escrow Only Admin4)
    VestingEscrowOnlyAdmin4 = 26,
    /// 65,540 for (Vesting Escrow Only Admin5)
    VestingEscrowOnlyAdmin5 = 27,
    /// 65,540 for (Vesting Escrow Only Admin6)
    VestingEscrowOnlyAdmin6 = 28,
    /// 65,540 for (Vesting Escrow Only Admin7)
    VestingEscrowOnlyAdmin7 = 29,
    /// 65,540 for (Vesting Escrow Admin Not Set)
    VestingEscrowAdminNotSet = 30,
   
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait VESTINGESCROWFACTORY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        _target: Key,
        _admin: Key,
        _vesting_escrow_simple_contract: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_target(_target);
        data::set_admin(_admin);
        data::set_vesting_escrow_simple_contract(_vesting_escrow_simple_contract);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    fn commit_transfer_ownership(&mut self, addr: Key) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&VESTINGESCROWFACTORYEvent::CommitOwnership { admin: addr });
        return true;
    }

    fn apply_transfer_ownership(&mut self) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowOnlyAdmin2);
        }
        let _admin = self.future_admin();
        if _admin == data::zero_address() {
            //Vesting Escrow Admin Not Set
            runtime::revert(Error::VestingEscrowAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&VESTINGESCROWFACTORYEvent::ApplyOwnership { admin: _admin });
        return true;
    }

    fn admin(&mut self) -> Key {
        data::admin()
    }

    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }

    fn target(&mut self) -> Key {
        data::target()
    }

    fn vesting_escrow_simple_contract(&mut self) -> Key {
        data::vesting_escrow_simple_contract()
    }

    fn emit(&mut self, vesting_escrow_factory_event: &VESTINGESCROWFACTORYEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match vesting_escrow_factory_event {
            VESTINGESCROWFACTORYEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_factory_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            VESTINGESCROWFACTORYEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_factory_event.type_name());
                event.insert("admin", admin.to_string());
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
