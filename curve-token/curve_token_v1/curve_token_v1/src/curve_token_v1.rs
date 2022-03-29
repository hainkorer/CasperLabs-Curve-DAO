use crate::data;
use alloc::{vec::Vec, string::{String, ToString}};
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};
use erc20_crate::{self,data as erc20_data,ERC20};



#[repr(u16)]
pub enum Error {
    
    OnlyMinterAllowed=0

    
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait CURVETOKENV1<Storage: ContractStorage>:
ContractContext<Storage> + ERC20<Storage>
{
    fn init(&mut self,name:String,symbol:String,decimal:u8,supply:U256, contract_hash: Key, package_hash: ContractPackageHash) {
       let base:i32=10;
      
        data::set_init_supply(supply *(base.pow(u32::from(decimal))));
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        
        ERC20::init(self,name,symbol,decimal,"".to_string(),"".to_string(),data::get_hash(),data::get_package_hash());
        erc20_data::Balances::instance().set(&self.get_caller(), 1000.into());
        
        erc20_data::set_total_supply(data::init_supply());
        data::set_minter(self.get_caller());
    }
    fn set_minter(&self,_minter:Key){
        if !(self.get_caller()==_minter){
            runtime::revert(ApiError::from(Error::OnlyMinterAllowed));
        }
        data::set_minter(_minter);
        
     }
    fn burn_from(&self,_to:Key,_value:U256){
        if !(self.get_caller()==data::minter()){
            runtime::revert(ApiError::from(Error::OnlyMinterAllowed));
        }
        ERC20::burn(self, _to, _value);
    }
    

}

