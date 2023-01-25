use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{bytesrepr::ToBytes, U256};
use curve_casper_erc20::Address;
use hex::encode;

pub const BALANCES: &str = "balances";
pub const ALLOWANCES: &str = "allowances";

pub const NAME: &str = "curve-erc20";
pub const SYMBOL: &str = "CURVE";
pub const DECIMALS: u8 = 9;
pub const INITIAL_SUPPLY: U256 = U256([0, 0, 0, 0]);

pub fn address_to_str(owner: &Address) -> String {
    let preimage = owner.to_bytes().unwrap();
    base64::encode(&preimage)
}

pub fn addresses_to_str(owner: Address, spender: Address) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(owner.to_bytes().unwrap());
    hasher.update(spender.to_bytes().unwrap());

    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));

    encode(ret)
}
