use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, URef, U256,
};
use test_env::{TestContract, TestEnv};

pub struct LIQUIDITYGAUGEInstance(TestContract);

impl LIQUIDITYGAUGEInstance {
    pub fn instance(liquidity_gauge: TestContract) -> LIQUIDITYGAUGEInstance {
        LIQUIDITYGAUGEInstance(liquidity_gauge)
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
            "liquidity-gauge.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter" => minter,
                "admin" => admin,
            },
        )
    }

    // pub fn proxy(
    //     env: &TestEnv,
    //     contract_name: &str,
    //     sender: AccountHash,
    //     liquidity_transformer: Key,
    // ) -> TestContract {
    //     TestContract::new(
    //         env,
    //         "proxy-liquidity-transformer.wasm",
    //         contract_name,
    //         sender,
    //         runtime_args! {
    //             "liquidity_transformer" => liquidity_transformer,
    //         },
    //     )
    // }
}
