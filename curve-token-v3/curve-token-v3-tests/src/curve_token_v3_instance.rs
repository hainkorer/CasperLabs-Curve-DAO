use std::collections::BTreeMap;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct CURVETOKENV3Instance(TestContract);

impl CURVETOKENV3Instance {
    pub fn instance(curvetokenv3: TestContract) -> CURVETOKENV3Instance {
        CURVETOKENV3Instance(curvetokenv3)
    }

    pub fn proxy(env: &TestEnv, curve_token_v3: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "crv3-proxy-token.wasm",
            "proxy_test",
            sender,
            runtime_args! {
                "curve_token_v3" => curve_token_v3,
            },
            0,
        )
    }
    pub fn proxy2(env: &TestEnv, curve_token_v3: Key, sender: AccountHash) -> TestContract {
        TestContract::new(
            env,
            "crv3-proxy-token.wasm",
            "proxy2_test",
            sender,
            runtime_args! {
                "curve_token_v3" => curve_token_v3,
            },
            0,
        )
    }

    pub fn new(
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
            0,
        )
    }

    pub fn constructor(&self, sender: AccountHash, name: String, symbol: String) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {

                "name" => name,
                "symbol" => symbol,
            },
            0,
        );
    }
    pub fn decimals(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "decimals", runtime_args! {}, 0);
    }

    pub fn get_total_supply_crv3(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "get_total_supply_crv3", runtime_args! {}, 0);
    }

    pub fn set_minter(&self, sender: AccountHash, _minter: Key) {
        self.0.call_contract(
            sender,
            "set_minter",
            runtime_args! {
                "_minter" => _minter
            },
            0,
        );
    }

    pub fn transfer(&self, sender: AccountHash, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer",
            runtime_args! {
                "recipient" => recipient,
                "amount" => amount

            },
            0,
        );
    }
    pub fn transfer_from(&self, sender: AccountHash, owner: Key, recipient: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "transfer_from",
            runtime_args! {
                "owner" => owner,
                "recipient" => recipient,
                "amount" => amount

            },
            0,
        );
    }
    pub fn approve(&self, sender: AccountHash, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "approve",
            runtime_args! {
                "spender" => spender,
                "amount" => amount

            },
            0,
        );
    }
    pub fn increase_allowance(&self, sender: AccountHash, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "increase_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount,

            },
            0,
        );
    }
    pub fn decrease_allowance(&self, sender: AccountHash, spender: Key, amount: U256) {
        self.0.call_contract(
            sender,
            "decrease_allowance",
            runtime_args! {
                "spender" => spender,
                "amount" => amount

            },
            0,
        );
    }
    pub fn mint(&self, sender: AccountHash, _to: Key, _value: U256) {
        self.0.call_contract(
            sender,
            "mint",
            runtime_args! {
                "_to" => _to,
                "_value"=>_value
            },
            0,
        );
    }
    pub fn burn(&self, sender: AccountHash, _value: U256) {
        self.0.call_contract(
            sender,
            "burn",
            runtime_args! {
                "_value"=>_value
            },
            0,
        );
    }
    pub fn burn_from(&self, sender: AccountHash, _to: Key, _value: U256) {
        self.0.call_contract(
            sender,
            "burn_from",
            runtime_args! {
                "_to" => _to,
                "_value"=>_value

            },
            0,
        );
    }
    pub fn set_name(&self, sender: AccountHash, _name: String, _symbol: String) {
        self.0.call_contract(
            sender,
            "set_name",
            runtime_args! {
                "_name" => _name,
                "_symbol"=>_symbol

            },
            0,
        );
    }

    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.package_hash().into()
    }
}

//     pub fn allowance_package_hash<T: Into<Key>>(
//         &self,
//         owner: ContractPackageHash,
//         spender: T,
//     ) -> U256 {
//         let owner: Key = owner.into();
//         let spender: Key = spender.into();
//         self.0
//             .query_dictionary("allowances", keys_to_str(&owner, &spender))
//             .unwrap_or_default()
//     }

//     pub fn name(&self) -> String {
//         self.0.query_named_key(String::from("name"))
//     }

//     pub fn symbol(&self) -> String {
//         self.0.query_named_key(String::from("symbol"))
//     }

//     pub fn decimals(&self) -> u8 {
//         self.0.query_named_key(String::from("decimals"))
//     }

//     pub fn total_supply(&self) -> U256 {
//         self.0.query_named_key(String::from("total_supply"))
//     }

//     pub fn contract_package_hash(&self) -> ContractPackageHash {
//         self.0
//             .query_named_key(String::from("contract_package_hash"))
//     }
//     pub fn contract_hash(&self) -> Key {
//         self.0.query_named_key(String::from("self_contract_hash"))
//     }

//     // Result methods
//     pub fn transfer_result(&self) -> Result<(), u32> {
//         self.0.query_named_key("transfer_result".to_string())
//     }

//     pub fn package_hash_result(&self) -> ContractPackageHash {
//         self.0.query_named_key("package_hash".to_string())
//     }

//     pub fn transfer_from_result(&self) -> Result<(), u32> {
//         self.0.query_named_key("transfer_from_result".to_string())
//     }
//     pub fn allowance_res(&self) -> U256 {
//         self.0.query_named_key("allowance".to_string())
//     }

//     pub fn increase_allowance_res(&self) -> Result<(), u32> {
//         self.0
//             .query_named_key("increase_allowance_result".to_string())
//     }
//     pub fn decrease_allowance_res(&self) -> Result<(), u32> {
//         self.0
//             .query_named_key("decrease_allowance_result".to_string())
//     }

//     pub fn meta(&self) -> Meta {
//         self.0.query_named_key(String::from("meta"))
//     }
// }

// pub fn key_to_str(key: &Key) -> String {
//     match key {
//         Key::Account(account) => account.to_string(),
//         Key::Hash(package) => hex::encode(package),
//         _ => panic!("Unexpected key type"),
//     }
// }

// pub fn keys_to_str(key_a: &Key, key_b: &Key) -> String {
//     let mut hasher = VarBlake2b::new(32).unwrap();
//     hasher.update(key_a.to_bytes().unwrap());
//     hasher.update(key_b.to_bytes().unwrap());
//     let mut ret = [0u8; 32];
//     hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
//     hex::encode(ret)
// }

// pub fn key_and_value_to_str<T: CLTyped + ToBytes>(key: &Key, value: &T) -> String {
//     let mut hasher = VarBlake2b::new(32).unwrap();
//     hasher.update(key.to_bytes().unwrap());
//     hasher.update(value.to_bytes().unwrap());
//     let mut ret = [0u8; 32];
//     hasher.finalize_variable(|hash| ret.clone_from_slice(hash));
//     hex::encode(ret)
// }
