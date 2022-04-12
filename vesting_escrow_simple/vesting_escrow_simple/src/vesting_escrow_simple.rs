use crate::data::{self, DisableddAt, InitialLocked, TotalClaimed};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    OnlyInitializeOnce = 0,
    AdminOnly = 1,
    CannotDisable = 2,
    AdminNotSet = 3,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub enum VESTINGESCROWSIMPLE_EVENT {
    Fund { recipient: Key, amount: U256 },
    Claim { recipient: Key, claimed: U256 },
    ToggleDisable { recipient: Key, disabled: bool },
    CommitOwnership { admin: Key },
    ApplyOwnership { admin: Key },
}

impl VESTINGESCROWSIMPLE_EVENT {
    pub fn type_name(&self) -> String {
        match self {
            VESTINGESCROWSIMPLE_EVENT::Fund {
                recipient: _,
                amount: _,
            } => "fund",
            VESTINGESCROWSIMPLE_EVENT::Claim {
                recipient: _,
                claimed: _,
            } => "claim",
            VESTINGESCROWSIMPLE_EVENT::ToggleDisable {
                recipient: _,
                disabled: _,
            } => "toggle_disable",
            VESTINGESCROWSIMPLE_EVENT::CommitOwnership { admin: _ } => "commit_ownership",
            VESTINGESCROWSIMPLE_EVENT::ApplyOwnership { admin: _ } => "apply_ownership",
        }
        .to_string()
    }
}
pub trait VESTINGESCROWSIMPLE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, contract_hash: Key, package_hash: ContractPackageHash) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_admin(self.get_caller());
        data::DisableddAt::init();
        data::InitialLocked::init();
        data::TotalClaimed::init();

        //initialization for testing purposes
        let token: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap();

        data::set_token(token);
        data::set_admin(self.get_caller());
        data::set_start_time(U256::from(1));
        data::set_end_time(U256::from(5));
        data::set_can_disable(true);
    }
    // fn initialize(&self,admin:Key,token:Key,recipient:Key,amount:U256,start_time:U256,end_time:U256,can_disable:bool) ->bool{
    //     if!(data::get_admin()== data::ZERO_ADDRESS()){
    //         runtime::revert(ApiError::from(Error::OnlyInitializeOnce));
    //     }
    //     data::set_token(token);
    //     data::set_admin(admin);
    //     data::set_start_time(start_time);
    //     data::set_end_time(end_time);
    //     data::set_can_disable(can_disable);

    // let ret: Result<(), u32> = runtime::call_contract(
    //     data::get_staking_token()
    //         .into_hash()
    //         .unwrap_or_revert()
    //         .into(),
    //     "transfer_from",
    //     runtime_args! {
    //         "owner" => self.get_caller(),
    //         "recipient" => Key::from(data::get_package_hash()),
    //         "amount" => amount
    //     },
    // );
    // match ret {
    //     Ok(()) => {}
    //     Err(e) => runtime::revert(ApiError::User(e as u16)),
    // }

    //     true
    // }
    fn toggle_disable(&self, recipient: Key) {
        if !(data::get_admin() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        if !(data::get_can_disable()) {
            runtime::revert(ApiError::from(Error::CannotDisable));
        }
        let mut is_disabled: bool = false;
        let blocktime: u64 = runtime::get_blocktime().into();
        if (DisableddAt::instance().get(&recipient) == 0.into()) {
            is_disabled = true;
        }
        if (is_disabled == true) {
            DisableddAt::instance().set(&recipient, U256::from(blocktime))
        } else {
            DisableddAt::instance().set(&recipient, U256::from(0))
        }
        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::ToggleDisable {
            recipient: recipient,
            disabled: is_disabled,
        });
        // log ToggleDisable(_recipient, is_disabled)
    }
    fn disable_can_disable(&self) {
        if !(data::get_admin() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        data::set_can_disable(false);
    }
    fn _total_vested_of(&self, recipient: Key, time: U256) -> U256 {
        //deafault parameter time
        let start: U256 = data::get_start_time();
        let end: U256 = data::get_end_time();
        let locked: U256 = InitialLocked::instance().get(&recipient);
        if (time < start) {
            return 0.into();
        }
        let sub1: U256 = time.checked_sub(start).unwrap_or_revert(); //fix naming convention
        let sub2: U256 = end.checked_sub(start).unwrap_or_revert();
        let mul: U256 = locked.checked_mul(sub1).unwrap_or_revert();
        let div: U256 = mul.checked_div(sub2).unwrap_or_revert();
        div.min(locked)
    }
    fn _total_vested(&self) -> U256 {
        let start: U256 = data::get_start_time();
        let end: U256 = data::get_end_time();
        let locked: U256 = data::get_initial_locked_supply();
        let temp_blocktime: u64 = runtime::get_blocktime().into();
        let blocktime: U256 = U256::from(temp_blocktime);
        if (blocktime < start) {
            return 0.into();
        }
        let sub1: U256 = blocktime.checked_sub(start).unwrap_or_revert(); //fix naming convention
        let sub2: U256 = end.checked_sub(start).unwrap_or_revert();
        let mul: U256 = locked.checked_mul(sub1).unwrap_or_revert();
        let div: U256 = mul.checked_div(sub2).unwrap_or_revert();
        div.min(locked)
    }
    fn vested_supply(&self) -> U256 {
        self._total_vested()
    }
    fn locked_supply(&self) -> U256 {
        let initial_locked_supply = data::get_initial_locked_supply();
        let total_vested: U256 = self._total_vested();
        initial_locked_supply
            .checked_sub(total_vested)
            .unwrap_or_revert()
    }
    fn vested_of(&self, recipient: Key) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        self._total_vested_of(recipient, U256::from(blocktime))
    }
    fn balance_of_vest(&self, recipient: Key) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        let total_vested_of: U256 = self._total_vested_of(recipient, U256::from(blocktime));
        let self_total_claimed: U256 = TotalClaimed::instance().get(&recipient);
        total_vested_of
            .checked_sub(self_total_claimed)
            .unwrap_or_revert()
    }
    fn locked_of(&self, recipient: Key) -> U256 {
        let initial_locked = InitialLocked::instance().get(&recipient);
        let blocktime: u64 = runtime::get_blocktime().into();
        let total_vested_of: U256 = self._total_vested_of(recipient, U256::from(blocktime));
        initial_locked
            .checked_sub(total_vested_of)
            .unwrap_or_revert()
    }
    fn commit_transfer_ownership(&self, addr: Key) -> bool {
        if !(self.get_caller() == data::get_admin()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        data::set_future_admin(addr);
        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::CommitOwnership {
            admin: addr,
        });
        //log CommitOwnership(addr)
        true
    }
    fn apply_transfer_ownership(&self) -> bool {
        if !(self.get_caller() == data::get_admin()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        let mut _admin: Key = data::get_future_admin();
        if !(_admin != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::AdminNotSet));
        }
        data::set_admin(_admin);
        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::ApplyOwnership {
            admin: _admin,
        });
        //log ApplyOwnership(_admin);
        return true;
    }

    fn vesting_escrow_simple_emit(&self, vesting_escrow_simple_event: &VESTINGESCROWSIMPLE_EVENT) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match vesting_escrow_simple_event {
            VESTINGESCROWSIMPLE_EVENT::Fund { recipient, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            VESTINGESCROWSIMPLE_EVENT::Claim { recipient, claimed } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("claimed", claimed.to_string());
                events.push(event);
            }
            VESTINGESCROWSIMPLE_EVENT::ToggleDisable {
                recipient,
                disabled,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("disabled", disabled.to_string());
                events.push(event);
            }
            VESTINGESCROWSIMPLE_EVENT::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            VESTINGESCROWSIMPLE_EVENT::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", vesting_escrow_simple_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
