use crate::data::*;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::ApiError;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use curve_erc20_crate::data::get_package_hash;
use curve_erc20_crate::{Address, CURVEERC20};

#[repr(u16)]
pub enum Error {
    /// 65,540 for (Lp Token Wrapper Subtraction Error 1)
    LpTokenWrapperSubtractionError1 = 11903,
    /// 65,540 for (Lp Token Wrapper Subtraction Error 2)
    LpTokenWrapperSubtractionError2 = 11904,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait LPTOKENWRAPPER<Storage: ContractStorage>:
    ContractContext<Storage> + CURVEERC20<Storage>
{
    fn init(&self, uni: Key) {
        set_uni(uni);
    }
    fn stake(&mut self, amount: U256) {
        self.mint(Address::from(self.get_caller()), amount)
            .unwrap_or_revert();
        runtime::call_versioned_contract::<()>(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer_from",
            runtime_args! {
                "owner" => Address::from(self.get_caller()),
                "recipient" => Address::Contract(get_package_hash()),
                "amount" => amount
            },
        );
    }
    fn withdraw(&mut self, amount: U256) {
        self.burn(Address::from(self.get_caller()), amount)
            .unwrap_or_revert();
        runtime::call_versioned_contract::<()>(
            get_uni().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => Address::from(self.get_caller()),
                "amount" => amount
            },
        );
    }
}
