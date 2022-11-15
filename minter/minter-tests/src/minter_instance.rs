use std::collections::BTreeMap;
use std::time::SystemTime;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, ContractPackageHash, Key,
    RuntimeArgs, U128, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct MINTERInstance(TestContract);
#[allow(clippy::too_many_arguments)]
impl MINTERInstance {
    pub fn instance(minter: TestContract) -> MINTERInstance {
        MINTERInstance(minter)
    }
    pub fn deploy_gauge_controller(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
        voting_escrow: Key,
        block_time:u64
    ) -> TestContract {
        TestContract::new(
            env,
            "gauge-controller-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "voting_escrow" => voting_escrow,
                "token" => token,
            },
            block_time,
        )
    }
    pub fn deploy_voting_escrow(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
        block_time:u64
    ) -> TestContract {
        TestContract::new(
            env,
            "voting-escrow.wasm",
            contract_name,
            sender,
            runtime_args! {
                "token_addr" => token_addr,
                "name" => name,
                "symbol" => symbol,
                "version" => version,
            },
            block_time,
        )
    }
    pub fn deploy_erc20(
        env: &TestEnv,
        sender: AccountHash,
        name: &str,
        symbol: &str,
        decimals: u8,
        supply: U256,
        block_time:u64
    ) -> TestContract {
        TestContract::new(
            env,
            "erc20-token.wasm",
            "proxy_test2",
            sender,
            runtime_args! {
                "initial_supply" => supply,
                "name" => name,
                "symbol" => symbol,
                "decimals" => decimals
            },
            block_time,
        )
    }
    pub fn deploy_liquidity_gauge(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        admin: Key,
        block_time:u64
    ) -> TestContract {
        TestContract::new(
            env,
            "liquidity-gauge-v3.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter" => minter,
                "admin" => admin,
            },
            block_time,
        )
    }
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        token: Key,
        controller: Key,
        block_time:u64
    ) -> TestContract {
        TestContract::new(
            env,
            "minter-token.wasm",
            contract_name,
            sender,
            runtime_args! {
                "controller" => controller,
                "token" => token,
            },
            block_time,
        )
    }
    pub fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash,block_time:u64) -> TestContract {
        TestContract::new(
            env,
            "erc20_crv.wasm",
            "erc20-crv",
            sender,
            runtime_args! {
                "name" => "CRV",
                "symbol" => "ERC20CRV",
                "decimals" => 9_u8,
            },
            block_time,
        )
    }
    pub fn constructor(
        &self,
        sender: AccountHash,
        name: &str,
        token: Key,
        controller: Key,
        reward_count: U256,
        block_time:u64
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "controller" => controller,
                "name" => name,
                "token" => token,
                "reward_count" => reward_count
            },
            block_time,
        );
    }

    pub fn mint<T: Into<Key>>(&self, sender: AccountHash, gauge_addr: T,block_time:u64) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "gauge_addr" => gauge_addr.into(),
            },
            block_time,
        );
    }
    pub fn mint_many(&self, sender: AccountHash, gauge_addrs: Vec<String>,block_time:u64) {
        self.0.call_contract(
            sender,
            "mint_many",
            runtime_args! {
                "gauge_addrs" => gauge_addrs,
            },
            block_time,
        );
    }
    pub fn mint_for<T: Into<Key>>(&self, sender: AccountHash, gauge_addr: T, _for: T,block_time:u64) {
        self.0.call_contract(
            sender,
            "mint_for",
            runtime_args! {
                "gauge_addr" => gauge_addr.into(),
                "for" => _for.into(),
            },
            block_time,
        );
    }
    pub fn toggle_approve_mint<T: Into<Key>>(&self, sender: AccountHash, minting_user: T,block_time:u64) {
        self.0.call_contract(
            sender,
            "toggle_approve_mint",
            runtime_args! {
                "minting_user" => minting_user.into(),
            },
            block_time,
        );
    }

    pub fn minted<T: Into<Key>>(&self, owner: T, spender: T) -> U256 {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("minted", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }
    pub fn allowed_to_mint_for<T: Into<Key>>(&self, owner: T, spender: T) -> bool {
        let owner: Key = owner.into();
        let spender: Key = spender.into();
        self.0
            .query_dictionary("allowed_to_mint_for", keys_to_str(&owner, &spender))
            .unwrap_or_default()
    }

    pub fn token(&self) -> Key {
        self.0.query_named_key(String::from("token"))
    }
    pub fn controller(&self) -> Key {
        self.0.query_named_key(String::from("controller"))
    }
    pub fn reward_count(&self) -> U256 {
        self.0.query_named_key(String::from("reward_count"))
    }
    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0
            .query_named_key(String::from("self_contract_package_hash"))
    }
    pub fn contract_hash(&self) -> Key {
        self.0.query_named_key(String::from("self_contract_hash"))
    }
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => panic!("Unexpected key type"),
    }
}

pub fn add_gauge<T: Into<Key>>(
    gauge_controller: &TestContract,
    sender: AccountHash,
    addr: T,
    gauge_type: (bool, U128),
    weight: Option<U256>,
    block_time:u64
) {
    gauge_controller.call_contract(
        sender,
        "add_gauge",
        runtime_args! {
            "addr" => addr.into(),
            "gauge_type" => gauge_type,
            "weight"=>weight
        },
        block_time,
    );
}
pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key_a.to_bytes().unwrap());
    hasher.update(key_b.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}

pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(key.to_bytes().unwrap());
    hasher.update(value.to_bytes().unwrap());
    let mut ret = [0u8; 32];
    hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
    hex::encode(ret)
}
