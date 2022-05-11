use std::time::{SystemTime, UNIX_EPOCH};

use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::liquidity_gauge_instance::LIQUIDITYGAUGEInstance;

fn deploy() -> (TestEnv, TestContract, AccountHash /*, TestContract*/) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let liquidity_transformer = LIQUIDITYGAUGEInstance::new(
        &env,
        "LIQUIDITY_GAUGE",
        owner,
        Key::Account(owner),
        Key::Account(owner),
        Key::Account(owner),
    );
    // let proxy = LIQUIDITYTRANSFORMERInstance::proxy(
    //     &env,
    //     "proxy",
    //     owner,
    //     Key::Hash(liquidity_transformer.contract_hash()),
    // );

    (env, liquidity_transformer, owner /*, proxy*/)
}

#[test]
fn test_deploy() {
    // let (_, _, _) = deploy();
}
