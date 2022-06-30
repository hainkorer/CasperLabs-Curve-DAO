use crate::alloc::string::ToString;
use crate::data::{self, MIN_VESTING_DURATION};
use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::format;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::*;
use vesting_escrow_simple_crate::entry_points::get_entry_points;
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

pub trait VESTINGESCROWFACTORY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        _target: Key,
        _admin: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_target(_target);
        data::set_admin(_admin);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
    }

    fn deploy_vesting_contract(
        &mut self,
        _token: Key,
        _recipient: Key,
        _amount: U256,
        _can_disable: bool,
        _vesting_duration: U256,
        _vesting_start: Option<U256>,
        // _vesting_escrow_simple_contract: Key,
    ) -> Key {
        // data::set_vesting_escrow_simple_contract(_vesting_escrow_simple_contract);
        let vesting_start: U256 = if let Some(..) = _vesting_start {
            _vesting_start.unwrap()
        } else {
            U256::from(u64::from(runtime::get_blocktime()))
        };

        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowFactoryOnlyAdmin3);
        } else if vesting_start < U256::from(u64::from(runtime::get_blocktime())) {
            //Vesting Escrow Start Time Too Soon
            runtime::revert(Error::VestingEscrowFactoryStartTimeTooSoon);
        } else if _vesting_duration < MIN_VESTING_DURATION {
            //Vesting Escrow Duration Too Soon
            runtime::revert(Error::VestingEscrowFactoryDurationTooShort);
        } else {
            // let _contract: Key = _vesting_escrow_simple_contract;
            let name: String = "VESTINGESCROWSIMPLE".to_string();
            let (package_hash, _) = storage::create_contract_package_at_hash();
            let (contract_hash, _) =
                storage::add_contract_version(package_hash, get_entry_points(), Default::default());
            runtime::put_key(&format!("{}_contract", name), contract_hash.into());
            // info.staking_rewards = Key::from(package_hash);
            // Access
            let constructor_access: URef = storage::create_contract_user_group(
                package_hash,
                "constructor",
                1,
                Default::default(),
            )
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

            let end_time = vesting_start
                .checked_add(_vesting_duration)
                .ok_or(Error::VestingEscrowFactoryOverFlow1)
                .unwrap_or_revert();

            // Call the constructor entry point
            let _: () = runtime::call_versioned_contract(
                package_hash,
                None,
                "constructor",
                runtime_args! {
                    "admin"=> self.admin(),
                    "token" => _token,
                    "recipient" => _recipient,
                    "amount" => _amount,
                    "start_time" => vesting_start,
                    "end_time" => end_time,
                    "can_disable" => _can_disable,
                    "contract_hash" => contract_hash,
                    "package_hash"=> package_hash
                },
            );

            // // Remove all URefs from the constructor group, so no one can call it for the second time.
            let mut urefs = BTreeSet::new();
            urefs.insert(constructor_access);
            storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
                .unwrap_or_revert();

            let token_hash_add_array = match _token {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };

            let token_package_hash = ContractPackageHash::new(token_hash_add_array);
            let _ret: () = runtime::call_versioned_contract(
                token_package_hash,
                None,
                "approve",
                runtime_args! {"spender" =>  Key::from(package_hash),"amount" => _amount},
            );

            // let _ret: bool = runtime::call_versioned_contract(
            //     package_hash,
            //     None,
            //     "initialize",
            //     runtime_args! {
            //     "admin" => self.admin(),
            //     "token" =>  _token,
            //     "recipient" =>  _recipient,
            //     "amount" => _amount,
            //     "start_time" => vesting_start,
            //     "end_time" => end_time,
            //     "can_disable" => _can_disable},
            // );
            Key::from(package_hash)
        }
    }

    fn commit_transfer_ownership(&mut self, addr: Key) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Factroy Only Admin
            runtime::revert(Error::VestingEscrowFactoryOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&VESTINGESCROWFACTORYEvent::CommitOwnership { admin: addr });
        true
    }

    fn apply_transfer_ownership(&mut self) -> bool {
        if self.get_caller() != self.admin() {
            //Vesting Escrow Only Admin
            runtime::revert(Error::VestingEscrowFactoryOnlyAdmin2);
        }
        let _admin = self.future_admin();
        if _admin == data::zero_address() {
            //Vesting Escrow Admin Not Set
            runtime::revert(Error::VestingEscrowFactoryAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&VESTINGESCROWFACTORYEvent::ApplyOwnership { admin: _admin });
        true
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
