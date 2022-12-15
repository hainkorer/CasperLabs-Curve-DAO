use crate::{data::*, event::GaugeProxyEvent};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;

use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    AccessDenied = 0,
    IsLocked,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait GAUGEPROXY<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &self,
        ownership_admin: Key,
        emergency_admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        set_ownership_admin(ownership_admin);
        set_emergency_admin(emergency_admin);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    fn commit_set_admins(&self, o_admin: Key, e_admin: Key) {
        if self.get_caller() != get_ownership_admin() {
            runtime::revert(ApiError::from(Error::AccessDenied));
        };
        set_future_ownership_admin(o_admin);
        set_future_emergency_admin(e_admin);
        self.emit(&GaugeProxyEvent::CommitAdmins {
            ownership_admin: o_admin,
            emergency_admin: e_admin,
        });
    }

    fn accept_set_admins(&self) {
        if self.get_caller() != get_future_ownership_admin() {
            runtime::revert(ApiError::from(Error::AccessDenied));
        };
        let e_admin: Key = get_future_emergency_admin();
        set_ownership_admin(self.get_caller());
        set_emergency_admin(e_admin);
        self.emit(&GaugeProxyEvent::ApplyAdmins {
            ownership_admin: self.get_caller(),
            emergency_admin: e_admin,
        });
    }

    fn commit_transfer_ownership(&self, gauge: Key, new_owner: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        if self.get_caller() != get_ownership_admin() {
            runtime::revert(ApiError::from(Error::AccessDenied));
        };
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => new_owner
            },
        );
        set_lock(false);
    }

    fn accept_transfer_ownership(&self, gauge: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "accept_transfer_ownership",
            runtime_args! {},
        );
        set_lock(false);
    }

    fn set_killed(&self, gauge: Key, is_killed: bool) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        if !(self.get_caller() == get_ownership_admin()
            || self.get_caller() == get_emergency_admin())
        {
            runtime::revert(ApiError::from(Error::AccessDenied));
        }
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "set_killed",
            runtime_args! {
                "is_killed" => is_killed
            },
        );
        set_lock(false);
    }

    fn set_rewards(&self, gauge: Key, reward_contract: Key, sigs: String, reward_tokens: Vec<Key>) {
        if self.get_caller() != get_ownership_admin() {
            runtime::revert(ApiError::from(Error::AccessDenied));
        };
        let () = runtime::call_versioned_contract(
            gauge.into_hash().unwrap_or_revert().into(),
            None,
            "set_rewards",
            runtime_args! {
                "reward_contract" => reward_contract,
                "sigs" => sigs,
                "reward_tokens" => reward_tokens,
            },
        );
    }

    fn emit(&self, gauge_proxy_event: &GaugeProxyEvent) {
        let mut events = Vec::new();
        let package = get_package_hash();
        match gauge_proxy_event {
            GaugeProxyEvent::CommitAdmins {
                emergency_admin,
                ownership_admin,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_proxy_event.type_name());
                event.insert("emergency_admin", emergency_admin.to_string());
                event.insert("ownership_admin", ownership_admin.to_string());
                events.push(event);
            }
            GaugeProxyEvent::ApplyAdmins {
                emergency_admin,
                ownership_admin,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_proxy_event.type_name());
                event.insert("emergency_admin", emergency_admin.to_string());
                event.insert("ownership_admin", ownership_admin.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
