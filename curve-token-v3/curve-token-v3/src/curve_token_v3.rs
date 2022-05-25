use crate::data;
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
use erc20_crate::{self, data as erc20_data, ERC20};
use common::errors::*;


pub enum CurveTokenV3Event {
    Transfer_crv3 { from: Key, to: Key, value: U256 },
}

impl CurveTokenV3Event {
    pub fn type_name(&self) -> String {
        match self {
            CurveTokenV3Event::Transfer_crv3 {
                from: _,
                to: _,
                value: _,
            } => "Transfer_crv3",
        }
        .to_string()
    }
}

pub trait CURVETOKENV3<Storage: ContractStorage>:
    ContractContext<Storage> + ERC20<Storage>
{
    fn init(
        &mut self,
        name: String,
        symbol: String,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);

        ERC20::init(
            self,
            name,
            symbol,
            9.into(),
            0.into(),
            "".to_string(),
            "".to_string(),
            data::get_hash(),
            data::get_package_hash(),
        );
        erc20_data::set_total_supply(10000.into());
        erc20_data::Balances::instance().set(&self.get_caller(), 1000.into());
        data::set_minter(self.get_caller());
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer_crv3 {
            from: data::zero_address(),
            to: self.get_caller(),
            value: 0.into(),
        });
    }
    fn mint_crv3(&mut self, _to: Key, _value: U256) {
        if !(self.get_caller() == data::minter()) {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed1));
        }
        ERC20::mint(self, _to, _value);
    }
    fn set_minter(&mut self, _minter: Key) {
        if !(self.get_caller() == data::minter()) {
            runtime::revert(ApiError::from(Error::CurveTokenV3InvalidMinter));
        }
        data::set_minter(_minter);
    }
    fn burn_from(&mut self, _to: Key, _value: U256) {
        if !(self.get_caller() == data::minter()) {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed2));
        }
        ERC20::burn(self, _to, _value);
    }
    fn set_name(&mut self, _name: String, _symbol: String) {
        if !(data::minter() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed3));
        }
        erc20_data::set_name(_name);
        erc20_data::set_symbol(_symbol);
    }
    fn curve_token_v3_emit(&self, curve_token_v3_event: &CurveTokenV3Event) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match curve_token_v3_event {
            CurveTokenV3Event::Transfer_crv3 { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_token_v3_event.type_name());
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
}
