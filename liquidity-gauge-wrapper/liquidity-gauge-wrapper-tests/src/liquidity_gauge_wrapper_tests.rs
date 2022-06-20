use crate::liquidity_gauge_wrapper_instance::LIQUIDITYGAUGEWRAPPERInstance;
use blake2::digest::consts::U128;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGuageWrapper";
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
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        0,
    )
}
// CRV
fn deploy_erc20_crv(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20_crv.wasm",
        "erc20-crv",
        sender,
        runtime_args! {
            "name" => "CRV",
            "symbol" => "ERC20CRV",
            "decimal" => 9 as u8,
            "supply" => U256::from(TEN_E_NINE * 10000000000000000)
        },
        200000000000,
    )
}
// Voting Escrow
fn deploy_voting_escrow(
    env: &TestEnv,
    sender: AccountHash,
    token_addr: Key,
    name: String,
    symbol: String,
    version: String,
) -> TestContract {
    TestContract::new(
        env,
        "voting-escrow.wasm",
        "voting-escrow",
        sender,
        runtime_args! {
            "token_addr" => token_addr,
            "name" => name,
            "symbol" => symbol,
            "version" => version,
        },
        0,
    )
}
//gauge_controller
fn deploy_gauge_controller(
    env: &TestEnv,
    sender: AccountHash,
    token: Key,
    voting_escrow: Key,
) -> TestContract {
    TestContract::new(
        env,
        "gauge-controller-token.wasm",
        "gauge-controller",
        sender,
        runtime_args! {
            "token" => token,
            "voting_escrow" => voting_escrow,
        },
        0,
    )
}
//Reward
fn deploy_reward(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "reward token",
        owner,
        runtime_args! {
            "name" => "reward token",
            "symbol" => "RT",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE * 100000000000000000000)
        },
        0,
    )
}
//Minter
fn deploy_minter(env: &TestEnv, sender: AccountHash, controller: Key, token: Key) -> TestContract {
    TestContract::new(
        env,
        "minter-token.wasm",
        "minter",
        sender,
        runtime_args! {
            "controller" => controller,
            "token" => token,
        },
        0,
    )
}
// Liquidity Gauge V3
fn deploy_liquidity_gauge_v3(env: &TestEnv, sender: AccountHash,lp_addr:Key, minter: Key, admin: Key) -> TestContract {
    TestContract::new(
        env,
        "liquidity-gauge-v3.wasm",
        "liquidity-gauge-v3",
        sender,
        runtime_args! {
            "lp_addr" => lp_addr,
            "minter" => minter,
            "admin" => admin,
        },
        0,
    )
}
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
    //let reward = deploy_reward(&env, owner);
    let erc20_crv = deploy_erc20_crv(&env, owner);
    let voting_escrow = deploy_voting_escrow(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        "Voting Escrow".into(),
        "VT".into(),
        "1".into(),
    );
    let gauge_controller = deploy_gauge_controller(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );
    let minter = deploy_minter(
        &env,
        owner,
        Key::Hash(gauge_controller.package_hash()),
        Key::Hash(erc20_crv.package_hash()),
    );
     let deploy_liquidity_gauge_v3 = deploy_liquidity_gauge_v3(
        &env,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
    );
    let liquidity_gauge_wrapper_instance = LIQUIDITYGAUGEWRAPPERInstance::new(
        &env,
        NAME,
        owner,
        "Gauge Wrapper".to_string(),
        "LGW".to_string(),
        Key::Hash(deploy_liquidity_gauge_v3.package_hash()),
        Key::Account(owner),
    );
    // For Minting Purpose
    let to = Key::Hash(deploy_liquidity_gauge_v3.package_hash());
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    let amount_1: U256 = U256::from(TEN_E_NINE * 100);
    // deploy_liquidity_gauge_reward.call_contract(
    //     owner,
    //     "set_approve_deposit",
    //     runtime_args! {"addr" => Key::Hash(deploy_liquidity_gauge_reward.package_hash()) , "can_deposit" => true},
    //     0,
    // );
    // let blocktime: u64 = (TEN_E_NINE * 10000000000) as u64;
    // erc20.call_contract(
    //     owner,
    //     "mint",
    //     runtime_args! {"to" => to , "amount" => amount},
    //     0,
    // );
    // erc20.call_contract(
    //     owner,
    //     "approve",
    //     runtime_args! {"spender" => to , "amount" => amount},
    //     0,
    // );
    // reward.call_contract(
    //     owner,
    //     "mint",
    //     runtime_args! {"to" => to , "amount" => amount},
    //     0,
    // );
    // erc20_crv.call_contract(
    //     owner,
    //     "set_minter",
    //     runtime_args! {"_minter" => Key::Account(owner)},
    //     0,
    // );
    // erc20_crv.call_contract(
    //     owner,
    //     "mint",
    //     runtime_args! {"to" => to , "value" => amount_1},
    //     2000000000000000000,
    // );

    // let _name: String = "type".to_string();
    // gauge_controller.call_contract(owner, "add_type", runtime_args! {"_name" => _name }, 0);
    // let addr: Key = Key::Account(owner);
    // let gauge_type: U128 = 0.into();
    // gauge_controller.call_contract(
    //     owner,
    //     "add_gauge",
    //     runtime_args! {
    //         "addr" => addr,
    //         "gauge_type" => gauge_type,
    //         "weight"=>None::<U256>
    //     },
    //     0,
    // );
    // let _name_1: String = "type1".to_string();
    // gauge_controller.call_contract(owner, "add_type", runtime_args! {"_name" => _name_1 }, 0);
    // let addr1: Key = Key::Hash(deploy_liquidity_gauge_reward.package_hash());
    // let gauge_type_1: U128 = 1.into();
    // gauge_controller.call_contract(
    //     owner,
    //     "add_gauge",
    //     runtime_args! {
    //         "addr" => addr1,
    //         "gauge_type" => gauge_type_1,
    //         "weight"=>None::<U256>
    //     },
    //     0,
    // );
    (env, owner, liquidity_gauge_wrapper_instance)
}
#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
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
