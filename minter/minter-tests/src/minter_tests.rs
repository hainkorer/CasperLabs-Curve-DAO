use casper_types::{account::AccountHash, Key};
use test_env::{TestContract, TestEnv};

use crate::minter_instance::MINTERInstance;

const NAME: &str = "MINTER";
const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

fn deploy() -> (
    TestEnv,
    MINTERInstance,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let _token: TestContract = MINTERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );

    let voting_escrow = MINTERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(_token.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
    );
    let gauge_controller: TestContract = MINTERInstance::deploy_gauge_controller(
        &env,
        "gauge_controller",
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );

    let minter: TestContract = MINTERInstance::new(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(gauge_controller.package_hash()),
    );
    // let test_contract: TestContract =
    //     MINTERInstance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    // let test_contract2: TestContract =
    //     MINTERInstance::proxy2(&env, Key::Hash(token.contract_hash()), owner);
    (
        env,
        MINTERInstance::instance(minter),
        owner,
        _token,
        voting_escrow,
        gauge_controller, // MINTERInstance::instance(test_contract),
                          // MINTERInstance::instance(test_contract2),
    )
}

#[test]
fn test_deploy() {
    let (env, minter, _owner, token, _voting_escrow, gauge_controller) = deploy();
    let _user = env.next_user();
    assert_eq!(minter.token(), Key::Hash(token.package_hash()));
    assert_eq!(
        minter.controller(),
        Key::Hash(gauge_controller.package_hash())
    );
}

#[test]
fn test_minter_mint() {
    let (env, minter, owner, token, _voting_escrow, _gauge_controller) = deploy();
    let _user = env.next_user();

    // minter.mint(owner, Key::Hash(token.package_hash()));
}

#[test]
fn test_minter_mint_many() {
    let (_env, minter, owner, token, _voting_escrow, _gauge_controller) = deploy();
    let gauge_addrs: Vec<String> = vec![
        Key::Hash(token.package_hash()).to_formatted_string(),
        Key::Hash(token.package_hash()).to_formatted_string(),
        Key::Hash(token.package_hash()).to_formatted_string(),
        Key::Hash(token.package_hash()).to_formatted_string(),
        Key::Hash(token.package_hash()).to_formatted_string(),
    ];
    // minter.mint_many(owner, gauge_addrs);
}

#[test]
fn test_minter_mint_for() {
    let (_env, minter, owner, token, _voting_escrow, _gauge_controller) = deploy();

    // minter.mint_for(owner, Key::Hash(token.package_hash()), Key::from(owner));
}

#[test]
fn test_minter_toggle_approve_mint() {
    let (_env, minter, owner, token, _voting_escrow, _gauge_controller) = deploy();

    minter.toggle_approve_mint(owner, Key::Hash(token.package_hash()));
}
