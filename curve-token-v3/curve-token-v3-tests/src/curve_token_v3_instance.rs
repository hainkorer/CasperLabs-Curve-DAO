use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractPackageHash, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};
use crv20::Address;
use hex::encode;
use std::{collections::BTreeMap, time::SystemTime};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
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

pub struct CURVETOKENV3Instance(TestContract);
#[allow(clippy::too_many_arguments)]
impl CURVETOKENV3Instance {
    pub fn instance(curvetokenv3: TestContract) -> CURVETOKENV3Instance {
        CURVETOKENV3Instance(curvetokenv3)
    }

    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        symbol: String,
    ) -> TestContract {
        TestContract::new(
            env,
            "curve-token-v3.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol
            },
            now(),
        )
    }

    // Result methods
    pub fn query<T: CLTyped + FromBytes>(&self, key: &str) -> T {
        self.0.query_named_key(key.into())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.package_hash().into()
    }
}
