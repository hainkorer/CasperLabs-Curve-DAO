use casper_types::{account::AccountHash, Key, U128, U256};
use test_env::{TestContract, TestEnv};

use crate::gauge_controller_instance::GAUGECONLTROLLERInstance;

const NAME: &str = "GAUGECONLTROLLER";
const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

fn deploy() -> (
    TestEnv,
    GAUGECONLTROLLERInstance,
    AccountHash,
    TestContract,
    TestContract, // GAUGECONLTROLLERInstance,
                  // GAUGECONLTROLLERInstance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let _token: TestContract = GAUGECONLTROLLERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );

    let voting_escrow = GAUGECONLTROLLERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(_token.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
    );
    let gauge_controller: TestContract = GAUGECONLTROLLERInstance::new(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
    );
    // let test_contract: TestContract =
    //     GAUGECONLTROLLERInstance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    // let test_contract2: TestContract =
    //     GAUGECONLTROLLERInstance::proxy2(&env, Key::Hash(token.contract_hash()), owner);
    (
        env,
        GAUGECONLTROLLERInstance::instance(gauge_controller),
        owner,
        _token,
        voting_escrow, // GAUGECONLTROLLERInstance::instance(test_contract),
                       // GAUGECONLTROLLERInstance::instance(test_contract2),
    )
}

fn deploy_fail() -> (
    TestEnv,
    GAUGECONLTROLLERInstance,
    AccountHash,
    TestContract,
    TestContract, // GAUGECONLTROLLERInstance,
                  // GAUGECONLTROLLERInstance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let _token: TestContract = GAUGECONLTROLLERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );

    let voting_escrow = GAUGECONLTROLLERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(_token.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
    );
    let gauge_controller: TestContract = GAUGECONLTROLLERInstance::new(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
        )
        .unwrap(),
    );
    // let test_contract: TestContract =
    //     GAUGECONLTROLLERInstance::proxy(&env, Key::Hash(token.contract_hash()), owner);
    // let test_contract2: TestContract =
    //     GAUGECONLTROLLERInstance::proxy2(&env, Key::Hash(token.contract_hash()), owner);
    (
        env,
        GAUGECONLTROLLERInstance::instance(gauge_controller),
        owner,
        _token,
        voting_escrow, // GAUGECONLTROLLERInstance::instance(test_contract),
                       // GAUGECONLTROLLERInstance::instance(test_contract2),
    )
}

#[test]
fn test_deploy() {
    let (env, gauge_controller, owner, token, voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
}

#[test]
#[should_panic]
fn test_deploy_with_address_zero() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy_fail();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
}

#[test]
fn test_gauge_controller_commit_transfer_ownership() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.commit_transfer_ownership(_owner, _user);
    assert_eq!(gauge_controller.future_admin(), Key::from(_user));
}
#[test]
#[should_panic]
fn test_gauge_controller_commit_transfer_ownership_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.commit_transfer_ownership(_user, _user);
    assert_eq!(gauge_controller.future_admin(), Key::from(_user));
}

#[test]
fn test_gauge_controller_apply_transfer_ownership() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.commit_transfer_ownership(_owner, _user);
    assert_eq!(gauge_controller.future_admin(), Key::from(_user));
    gauge_controller.apply_transfer_ownership(_owner);
    assert_eq!(gauge_controller.admin(), Key::from(_user));
}
#[test]
#[should_panic]
fn test_gauge_controller_apply_transfer_ownership_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.commit_transfer_ownership(_owner, _user);
    assert_eq!(gauge_controller.future_admin(), Key::from(_user));
    gauge_controller.apply_transfer_ownership(_user);
    assert_eq!(gauge_controller.admin(), Key::from(_user));
}

#[test]
#[should_panic]
fn test_gauge_controller_apply_transfer_ownership_without_commiting_transfer_ownership() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.apply_transfer_ownership(_owner);
    assert_eq!(gauge_controller.admin(), Key::from(_user));
}

#[test]
// #[should_panic]
fn test_gauge_controller_checkpoint() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.checkpoint(_owner);
}
#[test]
fn test_gauge_controller_checkpoint_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.checkpoint(_user);
}
#[test]
fn test_gauge_controller_checkpoint_gauge() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.checkpoint_gauge(_owner, _user);
}
#[test]
fn test_gauge_controller_checkpoint_gauge_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    gauge_controller.checkpoint_gauge(_user, _user);
}

#[test]
fn test_gauge_controller_change_type_weight() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let type_id: U128 = 1.into();
    let weight: U256 = 2.into();
    gauge_controller.change_type_weight(_owner, type_id, weight);
}

#[test]
#[should_panic]
fn test_gauge_controller_change_type_weight_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let type_id: U128 = 1.into();
    let weight: U256 = 2.into();
    gauge_controller.change_type_weight(_user, type_id, weight);
}

#[test]
fn test_gauge_controller_add_type() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
}

#[test]
fn test_gauge_controller_add_gauge() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_owner, _user, gauge_type);
}

#[test]
#[should_panic]
fn test_gauge_controller_add_gauge_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_user, _user, gauge_type);
}

#[test]
fn test_gauge_controller_add_gauge_multiple_time() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_owner, _user, gauge_type);
    let name: String = "type2".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 1.into();
    gauge_controller.add_gauge(_owner, _user1, gauge_type);
}

#[test]
#[should_panic]
fn test_gauge_controller_add_gauge_multiple_time_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_owner, _user, gauge_type);
    let name: String = "type2".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 1.into();
    gauge_controller.add_gauge(_user, _user1, gauge_type);
}

#[test]
fn test_gauge_controller_change_gauge_weight() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_owner, _user, gauge_type);
    let weight: U256 = 2.into();
    gauge_controller.change_gauge_weight(_owner, _user, weight);
}

#[test]
fn test_gauge_controller_change_gauge_weight_multiple_time() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_owner, _user, gauge_type);
    let weight: U256 = 2.into();
    gauge_controller.change_gauge_weight(_owner, _user, weight);
    let weight: U256 = 3.into();
    gauge_controller.change_gauge_weight(_owner, _user, weight);
}

#[test]
#[should_panic]
fn test_gauge_controller_change_gauge_weight_by_user() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    let gauge_type: U128 = 0.into();
    gauge_controller.add_gauge(_owner, _user, gauge_type);
    let weight: U256 = 2.into();
    gauge_controller.change_gauge_weight(_user, _user, weight);
}

#[test]
#[should_panic]
fn test_gauge_controller_change_gauge_weight_without_adding_type() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    // let name: String = "type".to_string();
    // gauge_controller.add_type(_owner, name);
    // let gauge_type: U128 = 0.into();
    // gauge_controller.add_gauge(_owner, _user, gauge_type);
    let weight: U256 = 2.into();
    gauge_controller.change_gauge_weight(_owner, _user, weight);
}

#[test]
#[should_panic]
fn test_gauge_controller_change_gauge_weight_without_adding_gauge() {
    let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(_voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(_owner));
    assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(_owner, name);
    // let gauge_type: U128 = 0.into();
    // gauge_controller.add_gauge(_owner, _user, gauge_type);
    let weight: U256 = 2.into();
    gauge_controller.change_gauge_weight(_owner, _user, weight);
}


// #[test]
// fn test_gauge_controller_vote_for_gauge_weights() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name);
//     let gauge_type: U128 = 0.into();
//     gauge_controller.add_gauge(_owner, _user, gauge_type);
//     let weight: U256 = 0.into();
//     gauge_controller.vote_for_gauge_weights(_owner, _user, weight);
// }