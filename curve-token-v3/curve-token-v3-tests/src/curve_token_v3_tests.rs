use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::curve_token_v3_instance::CURVETOKENV3Instance;

const NAME: &str = "CRVTokenV3";
const SYMBOL: &str = "CRV3";
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_token_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => "Token",
            "symbol" => "TK",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000)
        },
        0,
    )
}
fn deploy_reward(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc20",
        owner,
        runtime_args! {
            "name" => "Reward",
            "symbol" => "RD",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 1000000000000000000000)
        },
        0,
    )
}
fn deploy_curve_rewards(
    env: &TestEnv,
    owner: AccountHash,
    token: Key,
    reward: Key,
) -> TestContract {
    TestContract::new(
        &env,
        "curve-rewards.wasm",
        "CURVEREWARDS",
        owner,
        runtime_args! {
            "token" => token,
            "reward" => reward,
        },
        0,
    )
}
fn deploy() -> (
    TestEnv,
    CURVETOKENV3Instance,
    AccountHash,
    CURVETOKENV3Instance,
    CURVETOKENV3Instance,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token_erc20 = deploy_token_erc20(&env, owner);
    let reward = deploy_reward(&env, owner);
    let curve_reward = deploy_curve_rewards(
        &env,
        owner,
        Key::Hash(token_erc20.package_hash()),
        Key::Hash(reward.package_hash()),
    );
    let token: TestContract =
        CURVETOKENV3Instance::new(&env, NAME, owner, NAME.to_string(), SYMBOL.to_string());
    let test_contract: TestContract =
        CURVETOKENV3Instance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    let test_contract2: TestContract =
        CURVETOKENV3Instance::proxy2(&env, Key::Hash(token.contract_hash()), owner);

    (
        env,
        CURVETOKENV3Instance::instance(token),
        owner,
        CURVETOKENV3Instance::instance(test_contract),
        CURVETOKENV3Instance::instance(test_contract2),
        curve_reward,
    )
}

#[test]
fn test_deploy() {
    let (_, _, _, _, _, _) = deploy();
}
#[test]
fn test_decimals() {
    let (_, _, owner, proxy, _, _) = deploy();
    proxy.decimals(owner);
    let res: U256 = proxy.result();
    assert_eq!(res, 9.into());
}
#[test]
fn test_set_minter() {
    let (_, token, owner, _, _, _) = deploy();
    let _minter_arg: Key = Key::Account(owner);
    token.set_minter(owner, _minter_arg);
}
#[test]
fn test_mint() {
    let (_, token, owner, proxy, _, _) = deploy();
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    proxy.mint(owner, _to_arg, _value_arg);
    let res: bool = proxy.result();
    assert_eq!(res, true);
}
#[test]
fn test_transfer() {
    let (_, token, owner, proxy, _, _) = deploy();
    let _to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.transfer(owner, _to_arg, _value_arg);
    let res: Result<(), u32> = proxy.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "Transfer Failed ERROR:{}", e),
    }
}
#[test]
fn test_transfer_from() {
    let (_, token, owner, proxy, proxy2, _) = deploy();
    let to_arg: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
   
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.approve(owner, proxy2.package_hash().into(), _value_arg);
    proxy2.transfer_from(owner, proxy.package_hash().into(), to_arg, _value_arg);
    let res: Result<(), u32> = proxy2.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "transfer_from Failed ERROR:{}", e),
    }
}
#[test]
fn test_approve() {
    let (_, token, owner, proxy, _, _) = deploy();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    let approve_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.approve(owner, spender, approve_amount);
}
#[test]
fn test_increase_allowance() {
    let (_, token, owner, proxy, _, _) = deploy();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    let approve_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);

    proxy.increase_allowance(owner, spender, approve_amount);
    let res: Result<(), u32> = proxy.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "increase_allowance Failed ERROR:{}", e),
    }
}
#[test]
fn test_decrease_allowance() {
    let (_, token, owner, proxy, _, _) = deploy();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000".into(),
    )
    .unwrap();
    token.set_minter(owner, proxy.package_hash().into());
    let _value_arg: U256 = 2000000000.into();
    let decrease_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), _value_arg);
    proxy.approve(owner, spender, _value_arg);

    proxy.decrease_allowance(owner, spender, decrease_amount);
    let res: Result<(), u32> = proxy.result();
    match res {
        Ok(()) => {}
        Err(e) => assert!(false, "decrease_allowance Failed ERROR:{}", e),
    }
}
#[test]
fn test_burn_from() {
    let (_, token, owner, proxy, _, _) = deploy();
    

    token.set_minter(owner, proxy.package_hash().into());
    let mint_amount: U256 = 2000000000.into();
    let burn_amount: U256 = 1000000000.into();
    proxy.mint(owner, proxy.package_hash().into(), mint_amount);
    proxy.burn_from(owner, proxy.package_hash().into(), burn_amount);
    let res: bool = proxy.result();
    assert_eq!(res, true);
}
#[test]
fn test_set_name() {
    let (_, token, owner, _, _, curve_reward) = deploy();
    let curve_rewards_package_hash = Key::Hash(curve_reward.package_hash());
    token.set_minter(owner, curve_rewards_package_hash);
    token.set_name(owner, "curve-token-v3".to_string(), "crvtok3".to_string());
}
