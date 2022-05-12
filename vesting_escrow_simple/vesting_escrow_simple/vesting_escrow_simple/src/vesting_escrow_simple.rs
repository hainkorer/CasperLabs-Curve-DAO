use crate::data::*;
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
    IsLocked = 4,
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
    fn init(&self, token: Key, contract_hash: Key, package_hash: ContractPackageHash) {
        set_hash(contract_hash);
        set_package_hash(package_hash);
        DisableddAt::init();
        InitialLocked::init();
        TotalClaimed::init();

        //initialization for testing purposes

        set_token(token);
        set_admin(self.get_caller());
        set_future_admin(self.get_caller());
        set_start_time(U256::from(1));
        set_end_time(U256::from(5));
        set_can_disable(true);
        InitialLocked::instance().set(&ZERO_ADDRESS(), U256::from(100));
        set_initial_locked_supply(U256::from(100));
    }
    fn initialize(
        &self,
        admin: Key,
        token: Key,
        recipient: Key,
        amount: U256,
        start_time: U256,
        end_time: U256,
        can_disable: bool,
    ) -> bool {
        // if!(get_admin()== ZERO_ADDRESS()){
        //     runtime::revert(ApiError::from(Error::OnlyInitializeOnce));
        // }
        set_token(token);
        set_admin(admin);
        set_start_time(start_time);
        set_end_time(end_time);
        set_can_disable(can_disable);

        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" =>self.get_caller(),
                "recipient" => Key::from(get_package_hash()),
                "amount" => amount
            },
        );
        ret.unwrap_or_revert();
        InitialLocked::instance().set(&recipient, amount);
        set_initial_locked_supply(amount);
        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::Fund {
            recipient: recipient,
            amount: amount,
        });

        true
    }
    fn toggle_disable(&self, recipient: Key) {
        if !(get_admin() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        if !(get_can_disable()) {
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
    }
    fn disable_can_disable(&self) {
        if !(get_admin() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        set_can_disable(false);
    }
    fn _total_vested_of(&self, recipient: Key, time: U256) -> U256 {
        let start: U256 = get_start_time();
        let end: U256 = get_end_time();
        let locked: U256 = InitialLocked::instance().get(&recipient);
        if (time < start) {
            return 0.into();
        }
        let ans: U256 = locked
            .checked_mul(time.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(end.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert();
        ans.min(locked)
    }
    fn _total_vested(&self) -> U256 {
        let start: U256 = get_start_time();
        let end: U256 = get_end_time();
        let locked: U256 = get_initial_locked_supply();
        let temp_blocktime: u64 = runtime::get_blocktime().into();
        let blocktime: U256 = U256::from(temp_blocktime);
        if (blocktime < start) {
            return 0.into();
        }
        let ans: U256 = locked
            .checked_mul(blocktime.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(end.checked_sub(start).unwrap_or_revert())
            .unwrap_or_revert();
        ans.min(locked)
    }
    fn vested_supply(&self) -> U256 {
        self._total_vested()
    }
    fn locked_supply(&self) -> U256 {
        let initial_locked_supply = get_initial_locked_supply();
        let total_vested: U256 = self._total_vested();
        initial_locked_supply
            .checked_sub(total_vested)
            .unwrap_or_revert()
    }
    fn vested_of(&self, recipient: Key) -> U256 {
        let blocktime: U256 = 1000.into();
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
        if !(get_admin() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        set_future_admin(addr);
        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::CommitOwnership {
            admin: addr,
        });

        true
    }
    fn claim(&self, addr: Key) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        let mut t: U256 = DisableddAt::instance().get(&addr);
        let blocktime: U256 = 1000.into();
        if (t == U256::from(0)) {
            t = U256::from(blocktime);
        }
        let _total_vested_of: U256 = self._total_vested_of(addr, t);
        let _total_claimed: U256 = TotalClaimed::instance().get(&addr);
        let claimable: U256 = _total_vested_of
            .checked_sub(_total_claimed)
            .unwrap_or_revert();
        let updated_total_claimed = _total_claimed.checked_add(claimable).unwrap_or_revert();
        TotalClaimed::instance().set(&addr, updated_total_claimed);
        let token: Key = get_token();
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" =>addr,
                "amount" => claimable,
            },
        );
        match ret {
            Ok(()) => {}
            Err(e) => runtime::revert(ApiError::User(e as u16)),
        }

        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::Claim {
            recipient: addr,
            claimed: claimable,
        });
    }

    fn apply_transfer_ownership(&self) -> bool {
        // if !(self.get_caller() == get_admin()) {
        //     runtime::revert(ApiError::from(Error::AdminOnly));
        // }
        let mut _admin: Key = get_future_admin();
        if !(_admin != ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::AdminNotSet));
        }
        set_admin(_admin);
        self.vesting_escrow_simple_emit(&VESTINGESCROWSIMPLE_EVENT::ApplyOwnership {
            admin: _admin,
        });
        return true;
    }

    fn vesting_escrow_simple_emit(&self, vesting_escrow_simple_event: &VESTINGESCROWSIMPLE_EVENT) {
        let mut events = Vec::new();
        let package = get_package_hash();
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
