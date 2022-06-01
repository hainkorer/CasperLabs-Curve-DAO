use crate::{data::*, event::GaugeProxyEvent};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef,
};
use contract_utils::{ContractContext, ContractStorage};

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

    /// @notice Set ownership admin to `o_admin` and emergency admin to `e_admin`
    /// @param o_admin Ownership admin
    /// @param e_admin Emergency admin
    fn commit_set_admins(&self, o_admin: Key, e_admin: Key) {
        if !(self.get_caller() == get_ownership_admin()) {
            runtime::revert(ApiError::from(Error::AccessDenied));
        };
        set_future_ownership_admin(o_admin);
        set_future_emergency_admin(e_admin);
        self.emit(&GaugeProxyEvent::CommitAdmins {
            ownership_admin: o_admin,
            emergency_admin: e_admin,
        });
    }

    /// @notice Apply the effects of `commit_set_admins`
    /// @dev Only callable by the new owner admin
    fn accept_set_admins(&self) {
        if !(self.get_caller() == get_future_ownership_admin()) {
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

    /// @notice Transfer ownership for liquidity gauge `_gauge` to `new_owner`
    /// @param _gauge Gauge which ownership is to be transferred
    /// @param new_owner New gauge owner address
    fn commit_transfer_ownership(&self, gauge: Key, new_owner: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        if !(self.get_caller() == get_ownership_admin()) {
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

    /// @notice Apply transferring ownership of `_gauge`
    /// @param _gauge Gauge address
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

    /// @notice Set the killed status for `_gauge`
    /// @dev When killed, the gauge always yields a rate of 0 and so cannot mint CRV
    /// @param _gauge Gauge address
    /// @param _is_killed Killed status to set
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

    /// @notice Set the active reward contract for `_gauge`
    /// @param _gauge Gauge address
    /// @param _reward_contract Reward contract address. Set to ZERO_ADDRESS to disable staking.
    /// @param _sigs Four byte selectors for staking, withdrawing and claiming, right padded with zero bytes. If the reward contract
    ///     can be claimed from but does not require staking, the staking and withdraw selectors should be set to 0x00
    /// @param _reward_tokens List of claimable tokens for this reward contract
    fn set_rewards(&self, gauge: Key, reward_contract: Key, sigs: Bytes, reward_tokens: Vec<Key>) {
        if !(self.get_caller() == get_ownership_admin()) {
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
        let tmp = get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match gauge_proxy_event {
            GaugeProxyEvent::CommitAdmins {
                emergency_admin,
                ownership_admin,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
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
                event.insert("contract_package_hash", package_hash);
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
