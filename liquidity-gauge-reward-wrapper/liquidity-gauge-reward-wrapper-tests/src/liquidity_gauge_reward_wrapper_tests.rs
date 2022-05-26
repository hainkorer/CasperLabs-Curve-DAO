use crate::liquidity_gauge_reward_wrapper_instance::LIQUIDITYGAUGEREWARDWRAPPERInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGuageRewardWrapper";
//ERC20
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "rewarded_token",
        owner,
        runtime_args! {
            "name" => "rewarded_token",
            "symbol" => "ERA",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 10000000)
        },
        0,
    )
}

//Minter
fn deploy_minter(env: &TestEnv, owner: AccountHash) -> TestContract {
    let token = deploy_erc20(&env, owner);
    TestContract::new(
        &env,
        "minter-token.wasm",
        "minter",
        owner,
        runtime_args! {
            "token" => Key::Hash(token.package_hash()),
            "controller" => Key::Account(owner),
            "lock" => U256::from(0)
        },
        0,
    )
}
//Liquidity Guage Reward
// fn deploy_liquidity_gauge_reward(env: &TestEnv, owner: AccountHash) -> TestContract {
//     let rewarded_token = deploy_erc20(&env, owner);
//     let minter = deploy_minter(&env, owner);
//     TestContract::new(
//         &env,
//         "liquidity-gauge-reward.wasm",
//         "Liquidity Guage Reward",
//         owner,
//         runtime_args!{
//             "lp_addr" => lp_addr,
//             "minter" => Key::Hash(minter.package_hash()),
//             "reward_contract" => reward_contract,
//             "rewarded_token" => Key::Hash(rewarded_token.package_hash()),
//             "admin" => Key::Account(owner)
//         },
//         0
//     )
// }
// fn deploy() -> (
//     TestEnv,
//     AccountHash,
//     TestContract
// ) {
//     let env = TestEnv::new();
//     let owner = env.next_user();
//     let deploy_liquidity_gauge_reward = deploy_liquidity_gauge_reward(&env, owner);
//     let liquidity_gauge_reward_wrapper_instance = LIQUIDITYGAUGEREWARDWRAPPERInstance::new(
//         &env,
//         NAME,
//         owner,
//         "Reward Wrapper".to_string(),
//         "LGRW".to_string(),
//         Key::Hash(deploy_liquidity_gauge_reward.package_hash()),
//         Key::Account(owner),
//     );
//     (env,owner,liquidity_gauge_reward_wrapper_instance)
// }

#[test]
fn test_deploy() {
    //let (env, token, owner) = deploy();
    // let user = env.next_user();
    // assert_eq!(token.name(), NAME);
    // assert_eq!(token.symbol(), SYMBOL);
    // // assert_eq!(token.meta(), meta::contract_meta());
    // assert_eq!(
    //     token.total_supply(),
    //     (INIT_TOTAL_SUPPLY + INIT_TOTAL_SUPPLY).into()
    // );
    // assert_eq!(token.decimals(), DECIMALS);
    // assert_eq!(token.balance_of(owner), INIT_TOTAL_SUPPLY.into());
    // assert_eq!(token.balance_of(user), 0.into());
    // assert_eq!(token.allowance(owner, user), 0.into());
    // assert_eq!(token.allowance(user, owner), 0.into());
}
