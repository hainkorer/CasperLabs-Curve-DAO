use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDITYGAUGEREWARDInstance(TestContract);

impl LIQUIDITYGAUGEREWARDInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        reward_contract: Key,
        rewarded_token: Key,
        admin: Key,
    ) -> LIQUIDITYGAUGEREWARDInstance {
        LIQUIDITYGAUGEREWARDInstance(TestContract::new(
            env,
            "liquidity-gauge-reward.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter" => minter,
                "reward_contract" => reward_contract,
                "rewarded_token" => rewarded_token,
                "admin" => admin,
            },
            0,
        ))
    }

    pub fn user_checkpoint(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "user_checkpoint",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    // Get stored key values
    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
