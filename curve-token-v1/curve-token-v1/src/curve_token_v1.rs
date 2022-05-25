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
use common::errors::*;
use contract_utils::{ContractContext, ContractStorage};
use erc20_crate::{self, data as erc20_data, ERC20};

pub enum CurveTokenV1Event {
    Transfer_crv1 { from: Key, to: Key, value: U256 },
}

impl CurveTokenV1Event {
    pub fn type_name(&self) -> String {
        match self {
            CurveTokenV1Event::Transfer_crv1 {
                from: _,
                to: _,
                value: _,
            } => "Transfer_crv1",
        }
        .to_string()
    }
}

pub trait CURVETOKENV1<Storage: ContractStorage>:
    ContractContext<Storage> + ERC20<Storage>
{
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimal: u8,
        supply: U256,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        ERC20::init(
            self,
            name,
            symbol,
            decimal,
            0.into(),
            "".to_string(),
            "".to_string(),
            data::get_hash(),
            data::get_package_hash(),
        );
        let base: i32 = 10;
        data::set_init_supply(supply * (base.pow(u32::from(decimal))));

        erc20_data::Balances::instance().set(&self.get_caller(), 100000000.into());
        erc20_data::set_total_supply(data::get_init_supply());
        data::set_minter(self.get_caller());
        self.curve_token_v1_emit(&CurveTokenV1Event::Transfer_crv1 {
            from: data::ZERO_ADDRESS(),
            to: self.get_caller(),
            value: data::get_init_supply(),
        });
    }
    fn mint_crv1(&mut self, _to: Key, _value: U256) {
        if !(self.get_caller() == data::get_minter()) {
            runtime::revert(ApiError::from(Error::CurveTokenV1OnlyMinterAllowed1));
        }
        if !(_to != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::from(Error::CurveTokenV1ZeroAddressNotAllowed));
        }
        ERC20::mint(self, _to, _value);
    }
    fn set_minter(&mut self, _minter: Key) {
        if !(self.get_caller() == _minter) {
            runtime::revert(ApiError::from(Error::CurveTokenV1InvalidMinter));
        }
        data::set_minter(_minter);
    }
    fn burn_caller(&mut self, _value: U256) {
        if !(self.get_caller() == data::get_minter()) {
            runtime::revert(ApiError::from(Error::CurveTokenV1OnlyMinterAllowed2));
        }
        ERC20::burn(self, self.get_caller(), _value);
    }
    fn burn_from(&mut self, _to: Key, _value: U256) {
        if !(self.get_caller() == data::get_minter()) {
            runtime::revert(ApiError::from(Error::CurveTokenV1OnlyMinterAllowed3));
        }
        ERC20::burn(self, _to, _value);
    }

    fn curve_token_v1_emit(&self, curve_token_v1_event: &CurveTokenV1Event) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match curve_token_v1_event {
            CurveTokenV1Event::Transfer_crv1 { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_token_v1_event.type_name());
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
