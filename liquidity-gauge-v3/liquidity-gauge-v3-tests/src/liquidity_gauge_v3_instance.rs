use std::collections::BTreeMap;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash, bytesrepr::{ToBytes, FromBytes}, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct LIQUIDITYGUAGEV3INSTANCEInstance(TestContract);

impl LIQUIDITYGUAGEV3INSTANCEInstance {
    pub fn instance(liquidity_gauge_v3: TestContract) -> LIQUIDITYGUAGEV3INSTANCEInstance {
        LIQUIDITYGUAGEV3INSTANCEInstance(liquidity_gauge_v3)
    }

   
   

    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        admin: Key,
        
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-v3.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter"=>minter,
                "admin" => admin,
            },
            0,
        )
    }
    

  
   
pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn package_hash(&self) -> [u8; 32] {
    self.0.package_hash()
}

// Get stored key values
pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
    self.0.query_named_key(key)
}

}
