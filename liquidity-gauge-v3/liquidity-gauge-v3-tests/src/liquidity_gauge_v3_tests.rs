use crate::liquidity_gauge_v3_instance::LIQUIDITYGUAGEV3INSTANCEInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};
//Const
pub const TEN_E_NINE: u128 = 1000000000;
const NAME: &str = "LiquidityGaugeV3";
//ERC20
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "rewarded_token",
        owner,
        runtime_args! {
            "name" => "rewarded_token",
            "symbol" => "ERA",
            "decimals" => 9_u8,
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
            "decimal" => 9_u8,
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

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);
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

    let liquidity_gauge_v3_instance = LIQUIDITYGUAGEV3INSTANCEInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(erc20.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
    );
    // For Minting Purpose
    let to = Key::Hash(liquidity_gauge_v3_instance.package_hash());
    let amount: U256 = U256::from(TEN_E_NINE * 100000000000000000000);
    let amount_1: U256 = U256::from(TEN_E_NINE * 100);
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

    erc20_crv.call_contract(
        owner,
        "set_minter",
        runtime_args! {"_minter" => Key::Account(owner)},
        0,
    );
    erc20_crv.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to , "value" => amount_1},
        2000000000000000000,
    );

    let _name: String = "type".to_string();
    gauge_controller.call_contract(owner, "add_type", runtime_args! {"_name" => _name }, 0);
    let addr: Key = Key::Account(owner);
    let gauge_type: U128 = 0.into();
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr,
            "gauge_type" => gauge_type,
            "weight"=>None::<U256>
        },
        0,
    );
    let _name_1: String = "type1".to_string();
    gauge_controller.call_contract(owner, "add_type", runtime_args! {"_name" => _name_1 }, 0);
    let addr1: Key = Key::Hash(liquidity_gauge_v3_instance.package_hash());
    let gauge_type_1: U128 = 1.into();
    gauge_controller.call_contract(
        owner,
        "add_gauge",
        runtime_args! {
            "addr" => addr1,
            "gauge_type" => gauge_type_1,
            "weight"=>None::<U256>
        },
        0,
    );
    (env, owner, liquidity_gauge_v3_instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_commit_transfer_ownership() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr: Key = Key::Account(owner);
    contract.commit_transfer_ownership(owner, addr);
}
#[test]
fn test_accept_transfer_ownership() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let addr: Key = Key::Account(owner);
    contract.commit_transfer_ownership(owner, addr);
    contract.accept_transfer_ownership(owner);
}
#[test]
fn test_set_killed() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let is_killed: bool = true;
    contract.set_killed(owner, is_killed);
}
#[test]
fn test_increase_allowance() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let amount: U256 = 50000000.into();
    contract.increase_allowance(owner, spender, amount);
}
#[test]
fn test_decrease_allowance() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let approve_amount: U256 = 500000.into();
    contract.approve(owner, spender, approve_amount);
    let amount: U256 = 100000.into();
    contract.decrease_allowance(owner, spender, amount);
}
#[test]
fn test_approve() {
    let (_, owner, contract) = deploy();
    let contract = LIQUIDITYGUAGEV3INSTANCEInstance::instance(contract);
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000010000000000000000000000000000000000020000",
    )
    .unwrap();
    let approve_amount: U256 = 500000.into();
    contract.approve(owner, spender, approve_amount);
}
