use crate::data::*;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait LPTOKENWRAPPER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&self, uni: Key, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        set_uni(uni);
        set_hash(contract_hash);
        set_package_hash(package_hash);
        Balances::init();
    }
    fn total_supply(&self) -> U256 {
        return get_total_supply();
    }
    fn balance_of(&self, account: Key) -> U256 {
        return Balances::instance().get(&account);
    }
    fn stake(&mut self, amount: U256) {
        set_total_supply(get_total_supply().checked_add(amount).unwrap_or_revert());
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_add(amount)
                .unwrap_or_revert(),
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => self.get_caller(),
                "recipient" => Key::from(get_package_hash()),
                "amount" => amount
            },
        );
        match ret {
            Ok(()) => {}
            Err(e) => runtime::revert(ApiError::User(e as u16)),
        }
    }
    fn withdraw(&mut self, amount: U256) {
        set_total_supply(get_total_supply().checked_sub(amount).unwrap_or_revert());
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_sub(amount)
                .unwrap_or_revert(),
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => self.get_caller(),
                "amount" => amount
            },
        );
        match ret {
            Ok(()) => {}
            Err(e) => runtime::revert(ApiError::User(e as u16)),
        }
    }
}
