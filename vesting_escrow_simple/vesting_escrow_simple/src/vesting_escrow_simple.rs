
use crate::data::{self, DisableddAt,InitialLocked};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, U256};
use contract_utils::{ContractContext, ContractStorage};


#[repr(u16)]
pub enum Error {
   OnlyInitializeOnce=0,
   AdminOnly,
   CannotDisable
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait VESTINGESCROWSIMPLE<Storage: ContractStorage>:
        ContractContext<Storage>
{
    fn init(
        &self,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
       
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_admin(self.get_caller());
        data::DisableddAt::init();
        data::InitialLocked::init();
    }
    fn initialize(&self,admin:Key,token:Key,recipient:Key,amount:U256,start_time:U256,end_time:U256,can_disable:bool) ->bool{
        if!(data::get_admin()== data::ZERO_ADDRESS()){
            runtime::revert(ApiError::from(Error::OnlyInitializeOnce));
        }
        data::set_token(token);
        data::set_admin(admin);
        data::set_start_time(start_time);
        data::set_end_time(end_time);
        data::set_can_disable(can_disable);

        true
    }
    fn toggle_disable(&self,recipient:Key){

        if!(data::get_admin()== self.get_caller()){
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        if!(data::get_can_disable()){
            runtime::revert(ApiError::from(Error::CannotDisable));
        }
        let mut is_disabled:bool=false;
        let blocktime: u64 = runtime::get_blocktime().into();
        if (DisableddAt::instance().get(&recipient)==0.into()){
            is_disabled=true;
        }
        if (is_disabled==true){
            DisableddAt::instance().set(&recipient,U256::from(blocktime))
        }
        else{
            DisableddAt::instance().set(&recipient,U256::from(0))
        }
        // log ToggleDisable(_recipient, is_disabled)
        
    }
    fn disable_can_disable(&self){
        if!(data::get_admin()== self.get_caller()){
            runtime::revert(ApiError::from(Error::AdminOnly));
        }
        data::set_can_disable(false);
    }
    fn total_vested_of(&self,recipient:Key,time:U256)->U256{
        let start:U256= data::get_start_time();
        let end:U256= data::get_end_time();
        let locked:U256=InitialLocked::get(&recipient);
        if (time<start){
        return 0.into();
        }
        1.into();
        // U256::min(locked*())

    }



}
