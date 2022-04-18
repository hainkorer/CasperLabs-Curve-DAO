use crate::data::{self};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use erc20_crate::{self, data as erc20_data, ERC20};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use contract_utils::{ContractContext, ContractStorage};

#[repr(u16)]
pub enum Error {
    InvalidMinter=0,
    OnlyMinterAllowed=1,
    AdminOnly=2
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub enum ERC20CRV_EVENT {
    Transfer { from: Key, to: Key,value:U256},
    Approval{ owner: Key, spender: Key,value:U256},
    UpdateMiningParameters{ time:U256,rate:U256,supply:U256},
    SetMinter{minter:Key},
    SetAdmin{admin:Key}
}


impl ERC20CRV_EVENT {
    pub fn type_name(&self) -> String {
        match self {
            ERC20CRV_EVENT::Transfer {
                from: _,
                to: _,
                value:_,
            } => "transfer",
            ERC20CRV_EVENT::Approval {
                owner: _,
                spender: _,
                value:_,
            } => "approval",
            ERC20CRV_EVENT::UpdateMiningParameters {
                time: _,
                rate: _,
                supply:_,
            } => "update_mining_parameters",
            ERC20CRV_EVENT::SetMinter {
                minter:_,
            } => "set_minter",
            ERC20CRV_EVENT::SetAdmin {
                admin:_,
            } => "set_admin",
        }
        .to_string()
    }
}
pub trait ERC20CRV<Storage: ContractStorage>: 
ContractContext<Storage> + ERC20<Storage> {
    fn init(
        &self,
        name: String,
        symbol: String,
        decimal: u8,
        supply: U256,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        let base: i32 = 10;
        data::set_init_supply(supply * (base.pow(u32::from(decimal))));
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);

        ERC20::init(
            self,
            name,
            symbol,
            decimal,
            "".to_string(),
            "".to_string(),
            data::get_hash(),
            data::get_package_hash(),
        );
        erc20_data::Balances::instance().set(&self.get_caller(), 1000.into());

        erc20_data::set_total_supply(data::get_init_supply());
       data::set_admin(self.get_caller());
    }
    fn set_minter(&self, _minter: Key) {
        if !(self.get_caller() == _minter) {
            runtime::revert(ApiError::from(Error::InvalidMinter));
        }
        data::set_minter(_minter);
    }
    fn set_name(&self, _name: String, _symbol: String) {
        if !(data::get_minter() == self.get_caller()) {
            runtime::revert(ApiError::from(Error::OnlyMinterAllowed));
        }
        erc20_data::set_name(_name);
        erc20_data::set_symbol(_symbol);
    }
    fn burn_caller(&self, _value: U256) {
        if !(self.get_caller() == data::get_minter()) {
            runtime::revert(ApiError::from(Error::OnlyMinterAllowed));
        }
        ERC20::burn(self, self.get_caller(), _value);
    }
    fn set_admin(&self, _admin: Key) {
        if !(self.get_caller() == _admin) {
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        data::set_admin(_admin);
    }


  
    // fn vesting_escrow_simple_emit(&self, vesting_escrow_simple_event: &ERC20CRV_EVENT) {
    //     let mut events = Vec::new();
    //     let package = data::get_package_hash();
    //     match vesting_escrow_simple_event {
    //         VESTINGESCROWSIMPLE_EVENT::Fund { recipient, amount } => {
    //             let mut event = BTreeMap::new();
    //             event.insert("contract_package_hash", package.to_string());
    //             event.insert("event_type", vesting_escrow_simple_event.type_name());
    //             event.insert("recipient", recipient.to_string());
    //             event.insert("amount", amount.to_string());
    //             events.push(event);
    //         }
    //         VESTINGESCROWSIMPLE_EVENT::Claim { recipient, claimed } => {
    //             let mut event = BTreeMap::new();
    //             event.insert("contract_package_hash", package.to_string());
    //             event.insert("event_type", vesting_escrow_simple_event.type_name());
    //             event.insert("recipient", recipient.to_string());
    //             event.insert("claimed", claimed.to_string());
    //             events.push(event);
    //         }
    //         VESTINGESCROWSIMPLE_EVENT::ToggleDisable {
    //             recipient,
    //             disabled,
    //         } => {
    //             let mut event = BTreeMap::new();
    //             event.insert("contract_package_hash", package.to_string());
    //             event.insert("event_type", vesting_escrow_simple_event.type_name());
    //             event.insert("recipient", recipient.to_string());
    //             event.insert("disabled", disabled.to_string());
    //             events.push(event);
    //         }
    //         VESTINGESCROWSIMPLE_EVENT::CommitOwnership { admin } => {
    //             let mut event = BTreeMap::new();
    //             event.insert("contract_package_hash", package.to_string());
    //             event.insert("event_type", vesting_escrow_simple_event.type_name());
    //             event.insert("admin", admin.to_string());
    //             events.push(event);
    //         }
    //         VESTINGESCROWSIMPLE_EVENT::ApplyOwnership { admin } => {
    //             let mut event = BTreeMap::new();
    //             event.insert("contract_package_hash", package.to_string());
    //             event.insert("event_type", vesting_escrow_simple_event.type_name());
    //             event.insert("admin", admin.to_string());
    //             events.push(event);
    //         }
    //     };
    //     for event in events {
    //         let _: URef = storage::new_uref(event);
    //     }
    // }
}
