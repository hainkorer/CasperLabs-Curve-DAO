use crate::fee_distributor_instance::FEEDISTRIBUTORInstance;
use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use fee_distributor_crate::data::*;
use test_env::{TestContract, TestEnv};

fn deploy_erc20(env: &TestEnv, sender: AccountHash) -> TestContract {
    TestContract::new(
        env,
        "erc20-token.wasm",
        "erc20",
        sender,
        runtime_args! {
            "initial_supply" => U256::from(0),
            "name" => "Token",
            "symbol" => "ERC20",
            "decimals" => 9 as u8
        },
        0,
    )
}

fn deploy_voting_escrow(env: &TestEnv, sender: AccountHash, erc20: &TestContract) -> TestContract {
    TestContract::new(
        env,
        "voting-escrow.wasm",
        "Voting Escrow",
        sender,
        runtime_args! {
            "token_addr" => Key::Hash(erc20.package_hash()),
            "name" => String::from("VotingEscrow"),
            "symbol" => String::from("VE"),
            "version" => String::from("1"),
        },
        0,
    )
}

fn deploy() -> (TestEnv, AccountHash, FEEDISTRIBUTORInstance, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let erc20 = deploy_erc20(&env, owner);
    let voting_escrow = deploy_voting_escrow(&env, owner, &erc20);

    let instance = FEEDISTRIBUTORInstance::new(
        &env,
        "Fee Distributor",
        owner,
        Key::Hash(voting_escrow.contract_hash()),
        0.into(),
        Key::Hash(erc20.contract_hash()),
        Key::Account(owner),
        Key::Account(owner),
    );

    (env, owner, instance, erc20)
}

#[test]
fn test_deploy() {
    let (_, _, _, _) = deploy();
}
