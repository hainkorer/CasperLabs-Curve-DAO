use crate::lp_token_wrapper_instance::LPTOKENWRAPPERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 18 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 10000000000000)
        },
        0,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::new(
        &env,
        "LPTOKENWRAPPER",
        owner,
        Key::Hash(erc20.package_hash()),
    );
    let lp_token_wrapper_package_hash = Key::Hash(lp_token_wrapper_instance.package_hash());
    // For Minting Purpose
    let to: Key = Key::from(lp_token_wrapper_package_hash);
    let amount: U256 = U256::from(TEN_E_NINE * 1000000000000);
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "amount" => amount},
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => to , "amount" => amount},
        0,
    );
    (env, owner, lp_token_wrapper_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn total_supply() {
    let (_, owner, instance) = deploy();
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 2);
    lp_token_wrapper_instance.stake(owner, amount);
    // proxy.total_supply(owner);
    // let v: U256 = proxy.result();
    // println!("{:?}", v);
}
#[test]
fn balance_of() {
    let (_, owner, instance) = deploy();
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 2);
    lp_token_wrapper_instance.stake(owner, amount);
    // proxy.balance_of(owner, Key::Account(owner));
    // let v: U256 = proxy.result();
    // println!("{:?}", v);
}
#[test]
fn stake() {
    let (_, owner, instance) = deploy();
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    lp_token_wrapper_instance.stake(owner, amount);
}
#[test]
fn withdraw() {
    let (_, owner, instance) = deploy();
    let lp_token_wrapper_instance = LPTOKENWRAPPERInstance::contract_instance(instance);
    let amount: U256 = U256::from(TEN_E_NINE * 20);
    lp_token_wrapper_instance.stake(owner, amount);
    let withdraw_amount: U256 = U256::from(TEN_E_NINE * 10);
    lp_token_wrapper_instance.withdraw(owner, withdraw_amount);
}
