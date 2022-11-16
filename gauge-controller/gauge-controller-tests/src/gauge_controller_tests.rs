use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U128, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::gauge_controller_instance::GAUGECONLTROLLERInstance;
use common::keys::*;

const NAME: &str = "GAUGECONLTROLLER";
const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;
pub const VOTING_ESCROW_WEEK: U256 = U256([604800000, 0, 0, 0]); // all future times are rounded by week

fn deploy() -> (
    TestEnv,
    GAUGECONLTROLLERInstance,
    AccountHash,
    TestContract,
    TestContract,
    u64,
    TestContract
) {
    let block_time = GAUGECONLTROLLERInstance::now();
    let env = TestEnv::new();
    let owner = env.next_user();

    let _token: TestContract = GAUGECONLTROLLERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        block_time
    );

    let voting_escrow = GAUGECONLTROLLERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(_token.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
        block_time
    );
    let gauge_controller: TestContract = GAUGECONLTROLLERInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(voting_escrow.package_hash()),
        block_time
    );
    let minter:TestContract = GAUGECONLTROLLERInstance::minter(
        &env,
        "Minter",
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(gauge_controller.package_hash()),
        block_time
    );
    let liquidity_gauge = GAUGECONLTROLLERInstance::deploy_liquidity_gauge(
        &env,
        "Liquidity Gauge 1",
        owner,
        Key::Hash(_token.package_hash()),
        Key::Hash(minter.package_hash()),
        Key::Account(owner),
        block_time
    );
    (
        env,
        GAUGECONLTROLLERInstance::instance(gauge_controller),
        owner,
        _token,
        voting_escrow,
        block_time,
        liquidity_gauge
    )
}

fn deploy_fail() -> (
    TestEnv,
    GAUGECONLTROLLERInstance,
    AccountHash,
    TestContract,
    TestContract,
    u64
) {
    let block_time = GAUGECONLTROLLERInstance::now();
    let env = TestEnv::new();
    let owner = env.next_user();

    let _token: TestContract = GAUGECONLTROLLERInstance::deploy_erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
        block_time
    );

    let voting_escrow = GAUGECONLTROLLERInstance::deploy_voting_escrow(
        &env,
        "Voting Escrow",
        owner,
        Key::Hash(_token.package_hash()),
        "VotingEscrow".into(),
        "VE".into(),
        "1".into(),
        block_time
    );
    let gauge_controller: TestContract = GAUGECONLTROLLERInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(_token.package_hash()),
        Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
        block_time
    );
    (
        env,
        GAUGECONLTROLLERInstance::instance(gauge_controller),
        owner,
        _token,
        voting_escrow,
        block_time
    )
}

// #[test]
// fn test_deploy() {
//     let (env, gauge_controller, owner, token, voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(1668038400000 as u128));
// }

// #[test]
// #[should_panic]
// fn test_deploy_with_address_zero() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy_fail();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(1668038400000 as u128));
// }

// #[test]
// fn test_gauge_controller_commit_transfer_ownership() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(1668038400000 as u128));
//     gauge_controller.commit_transfer_ownership(_owner, _user,blocktime);
//     assert_eq!(gauge_controller.future_admin(), Key::from(_user));
// }
// #[test]
// #[should_panic]
// fn test_gauge_controller_commit_transfer_ownership_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(1668038400000 as u128));
//     gauge_controller.commit_transfer_ownership(_user, _user,blocktime);
//     assert_eq!(gauge_controller.future_admin(), Key::from(_user));
// }

// #[test]
// fn test_gauge_controller_apply_transfer_ownership() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(1668038400000 as u128));
//     gauge_controller.commit_transfer_ownership(_owner, _user,blocktime);
//     assert_eq!(gauge_controller.future_admin(), Key::from(_user));
//     gauge_controller.apply_transfer_ownership(_owner,blocktime);
//     assert_eq!(gauge_controller.admin(), Key::from(_user));
// }
// #[test]
// #[should_panic]
// fn test_gauge_controller_apply_transfer_ownership_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
       //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(1668038400000 as u128));
//     gauge_controller.commit_transfer_ownership(_owner, _user,blocktime);
//     assert_eq!(gauge_controller.future_admin(), Key::from(_user));
//     gauge_controller.apply_transfer_ownership(_user,blocktime);
//     assert_eq!(gauge_controller.admin(), Key::from(_user));
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_apply_transfer_ownership_without_commiting_transfer_ownership() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     assert_eq!(gauge_controller.time_total(), U256::from(0));
//     gauge_controller.apply_transfer_ownership(_owner,blocktime);
//     assert_eq!(gauge_controller.admin(), Key::from(_user));
// }

// #[test]
// fn test_gauge_controller_checkpoint() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(0));
//     gauge_controller.checkpoint(_owner,blocktime);
// }
// #[test]
// fn test_gauge_controller_checkpoint_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     //This line is commented because it may vary due to blocktime
//     //assert_eq!(gauge_controller.time_total(), U256::from(0));
//     gauge_controller.checkpoint(_user,blocktime);
// }
// #[test]
// fn test_gauge_controller_checkpoint_gauge() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//    // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     gauge_controller.checkpoint_gauge(_owner, _user,blocktime);
// }
// #[test]
// fn test_gauge_controller_checkpoint_gauge_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//    //assert_eq!(gauge_controller.time_total(), U256::from(0));
//     gauge_controller.checkpoint_gauge(_user, _user,blocktime);
// }

// #[test]
// fn test_gauge_controller_change_type_weight() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//      // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight,blocktime);
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_change_type_weight_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//      // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_user, type_id, weight,blocktime);
// }

// #[test]
// fn test_gauge_controller_add_type() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//      // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
// }

// #[test]
// fn test_gauge_controller_add_gauge() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//        // This line is commented because it may vary due to blocktime
//       // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     let _weight: U256 = 1.into();
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_add_gauge_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//      // This line is commented because it may vary due to blocktime
//       // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_user, _user, gauge_type, None,blocktime);
// }

// #[test]
// fn test_gauge_controller_add_gauge_multiple_time() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,_) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, None,blocktime);
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_add_gauge_multiple_time_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,liquidity_gauge) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_user, _user1, gauge_type, None,blocktime);
// }

// #[test]
// fn test_gauge_controller_change_gauge_weight() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,liquidity_gauge) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
//     let weight: U256 = 2.into();
//     gauge_controller.change_gauge_weight(_owner, _user, weight,blocktime);
// }

// #[test]
// fn test_gauge_controller_change_gauge_weight_multiple_time() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,liquidity_gauge) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
//     let weight: U256 = 2.into();
//     gauge_controller.change_gauge_weight(_owner, _user, weight,blocktime);
//     let weight: U256 = 3.into();
//     gauge_controller.change_gauge_weight(_owner, _user, weight,blocktime);
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_change_gauge_weight_by_user() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,liquidity_gauge) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
//     let weight: U256 = 2.into();
//     gauge_controller.change_gauge_weight(_user, _user, weight,blocktime);
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_change_gauge_weight_without_adding_type() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,liquidity_gauge) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let weight: U256 = 2.into();
//     gauge_controller.change_gauge_weight(_owner, _user, weight,blocktime);
// }


// #[test]
// #[should_panic]
// fn test_gauge_controller_change_gauge_weight_without_adding_gauge() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime,liquidity_gauge) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let weight: U256 = 2.into();
//     gauge_controller.change_gauge_weight(_owner, _user, weight,blocktime);
// }

#[test]
fn test_gauge_controller_vote_for_gauge_weights() {
    let (env, gauge_controller, owner, token, voting_escrow,blocktime,liquidity_gauge) = deploy();

    let value: U256 = (10000000000 as u128).into();
    let unlock_time: U256 =
        VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + blocktime;
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        blocktime,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(voting_escrow.package_hash()),
            "amount" => value + value
        },
        blocktime,
    );
    voting_escrow.call_contract(
        owner,
        "create_lock",
        runtime_args! {
            "value" => value,
            "unlock_time" => unlock_time
        },
        blocktime,
    );
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(owner));
    // This line is commented because it may vary due to blocktime
    // assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(owner, name, None,blocktime);
    let gauge_type: (bool, U128) = (false, 0.into());
    gauge_controller.add_gauge(owner, _user, gauge_type, None,blocktime);
    let weight: U256 = 0.into();
    gauge_controller.vote_for_gauge_weights(owner, _user, weight,blocktime);
}

// #[test]
// #[should_panic]
// fn test_gauge_controller_vote_for_gauge_weights_by_user() {
//     let (env, gauge_controller, owner, token, voting_escrow,blocktime,liquidity_gauge) = deploy();

//     let value: U256 = 1000.into();
//     let unlock_time: U256 =
//         VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK;
//     token.call_contract(
//         owner,
//         "mint",
//         runtime_args! {
//             "to" => Key::Account(owner),
//             "amount" => value + value
//         },
//         0,
//     );
//     token.call_contract(
//         owner,
//         "approve",
//         runtime_args! {
//             "spender" => Key::Hash(voting_escrow.package_hash()),
//             "amount" => value + value
//         },
//         0,
//     );
//     voting_escrow.call_contract(
//         owner,
//         "create_lock",
//         runtime_args! {
//             "value" => value,
//             "unlock_time" => unlock_time
//         },
//         0,
//     );
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(owner, _user, gauge_type, None,blocktime);
//     let weight: U256 = 0.into();
//     gauge_controller.vote_for_gauge_weights(_user, _user, weight,blocktime);
// }

#[test]
fn test_gauge_controller_vote_for_gauge_weights_multiple_time() {
    let (env, gauge_controller, owner, token, voting_escrow,blocktime,liquidity_gauge) = deploy();

    let value: U256 = (10000000000 as u128).into();
    let unlock_time: U256 =
        VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + VOTING_ESCROW_WEEK + blocktime;
    token.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Key::Account(owner),
            "amount" => value + value
        },
        blocktime,
    );
    token.call_contract(
        owner,
        "approve",
        runtime_args! {
            "spender" => Key::Hash(voting_escrow.package_hash()),
            "amount" => value + value
        },
        blocktime,
    );
    voting_escrow.call_contract(
        owner,
        "create_lock",
        runtime_args! {
            "value" => value,
            "unlock_time" => unlock_time
        },
        blocktime,
    );
    let _user = env.next_user();
    let _user1 = env.next_user();
    assert_eq!(gauge_controller.token(), Key::Hash(token.package_hash()));
    assert_eq!(
        gauge_controller.voting_escrow(),
        Key::Hash(voting_escrow.package_hash())
    );
    assert_eq!(gauge_controller.admin(), Key::from(owner));
    // This line is commented because it may vary due to blocktime
    // assert_eq!(gauge_controller.time_total(), U256::from(0));
    let name: String = "type".to_string();
    gauge_controller.add_type(owner, name, None,blocktime);
    let gauge_type: (bool, U128) = (false, 0.into());
    gauge_controller.add_gauge(owner, _user, gauge_type, None,blocktime);
    let weight: U256 = 0.into();
    gauge_controller.vote_for_gauge_weights(owner, _user, weight,blocktime);
    let name: String = "type2".to_string();
    gauge_controller.add_type(owner, name, None,blocktime);
    let gauge_type: (bool, U128) = (false, 1.into());
    gauge_controller.add_gauge(owner, _user1, gauge_type, None,blocktime);
    let weight: U256 = 1.into();
    gauge_controller.vote_for_gauge_weights(owner, _user1, weight,blocktime);
}

// #[test]
// fn test_gauge_controller_gauge_types() {
//     let (env, gauge_controller, _owner, _token, _voting_escrow,blocktime) = deploy();
//     let _user = env.next_user();
//     let _user1 = env.next_user();
//     assert_eq!(gauge_controller.token(), Key::Hash(_token.package_hash()));
//     assert_eq!(
//         gauge_controller.voting_escrow(),
//         Key::Hash(_voting_escrow.package_hash())
//     );
//     assert_eq!(gauge_controller.admin(), Key::from(_owner));
//     // This line is commented because it may vary due to blocktime
//     // assert_eq!(gauge_controller.time_total(), U256::from(0));
//     let name: String = "type".to_string();
//     gauge_controller.add_type(_owner, name, None,blocktime);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None,blocktime);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_owner, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 0.into()));
// }

// #[test]
// fn test_gauge_controller_gauge_types_by_user() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_user, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 0.into()));
// }

// #[test]
// fn test_gauge_controller_gauge_types_by_user_multiple_times() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None);
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, None);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_user, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 0.into()));

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_user, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 1.into()));
// }

// #[test]
// fn test_gauge_controller_gauge_types_multiple_times() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, None);
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, None);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_user, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 0.into()));

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_user, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 1.into()));
// }

// #[test]
// #[should_panic]
// fn test_gauge_controller_gauge_types_without_adding_gauge_types() {
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
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_TYPES),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user)
//         },
//         0,
//     );

//     let ret: (bool, U128) = env.query_account_named_key(_user, &[GAUGE_TYPES.into()]);
//     assert_eq!(ret, (false, 0.into()));
// }

// #[test]
// fn test_gauge_controller_gauge_relative_weight() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(1000000.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_RELATIVE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GAUGE_RELATIVE_WEIGHT.into()]);
//     assert_eq!(ret, 0.into());
// }

// #[test]
// fn test_gauge_controller_gauge_relative_weight_by_user() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(1000000.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_RELATIVE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_user, &[GAUGE_RELATIVE_WEIGHT.into()]);
//     assert_eq!(ret, 0.into());
// }

// #[test]
// fn test_gauge_controller_gauge_relative_weight_without_adding_gauge() {
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

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_RELATIVE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GAUGE_RELATIVE_WEIGHT.into()]);
//     assert_eq!(ret, 0.into());
// }

// #[test]
// fn test_gauge_controller_gauge_relative_weight_write() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(1000000.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_RELATIVE_WEIGHT_WRITE),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GAUGE_RELATIVE_WEIGHT_WRITE.into()]);
//     assert_eq!(ret, 0.into());
// }

// #[test]
// fn test_gauge_controller_gauge_relative_weight_write_by_user() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(1000000.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GAUGE_RELATIVE_WEIGHT_WRITE),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_user, &[GAUGE_RELATIVE_WEIGHT_WRITE.into()]);
//     assert_eq!(ret, 0.into());
// }

// #[test]
// fn test_gauge_controller_get_gauge_weight() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(1000000.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_GAUGE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_GAUGE_WEIGHT.into()]);
//     assert_eq!(ret, 1000000.into());
// }

// #[test]
// fn test_gauge_controller_get_gauge_weight_multiple_users() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_GAUGE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user1)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_GAUGE_WEIGHT.into()]);
//     assert_eq!(ret, 1000000.into());
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_GAUGE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "addr"=>Key::from(_user)
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_GAUGE_WEIGHT.into()]);
//     assert_eq!(ret, 500.into());
// }

// #[test]
// fn test_gauge_controller_get_type_weight() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);

//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user1,
//         runtime_args! {
//             "entrypoint" => String::from(GET_TYPE_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_user1, &[GET_TYPE_WEIGHT.into()]);
//     assert_eq!(ret, 2.into());
// }

// #[test]
// fn test_gauge_controller_get_total_weight() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_TOTAL_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_TOTAL_WEIGHT.into()]);
//     assert_eq!(ret, 2000000.into());
// }

// #[test]
// fn test_gauge_controller_get_total_weight_by_user() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GET_TOTAL_WEIGHT),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_user, &[GET_TOTAL_WEIGHT.into()]);
//     assert_eq!(ret, 2000000.into());
// }

// #[test]
// fn test_gauge_controller_get_weights_sum_per_type() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_WEIGHTS_SUM_PER_TYPE),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_WEIGHTS_SUM_PER_TYPE.into()]);
//     assert_eq!(ret, 1000000.into());
// }

// #[test]
// fn test_gauge_controller_get_weights_sum_per_type_by_user() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _user,
//         runtime_args! {
//             "entrypoint" => String::from(GET_WEIGHTS_SUM_PER_TYPE),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_user, &[GET_WEIGHTS_SUM_PER_TYPE.into()]);
//     assert_eq!(ret, 1000000.into());
// }

// #[test]
// fn test_gauge_controller_get_weights_sum_per_type_multiple_times() {
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
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 0.into());
//     gauge_controller.add_gauge(_owner, _user, gauge_type, Some(500.into()));
//     let name: String = "type2".to_string();
//     gauge_controller.add_type(_owner, name, None);
//     let gauge_type: (bool, U128) = (false, 1.into());
//     gauge_controller.add_gauge(_owner, _user1, gauge_type, Some(1000000.into()));
//     let type_id: (bool, U128) = (false, 1.into());
//     let weight: U256 = 2.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_WEIGHTS_SUM_PER_TYPE),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_WEIGHTS_SUM_PER_TYPE.into()]);
//     assert_eq!(ret, 1000000.into());
//     let type_id: (bool, U128) = (false, 0.into());
//     let weight: U256 = 3.into();
//     gauge_controller.change_type_weight(_owner, type_id, weight);
//     TestContract::new(
//         &env,
//         "gauge-controller-session-code.wasm",
//         "SessionCode",
//         _owner,
//         runtime_args! {
//             "entrypoint" => String::from(GET_WEIGHTS_SUM_PER_TYPE),
//             "package_hash" => Key::from(gauge_controller.contract_package_hash()),
//             "type_id"=>type_id
//         },
//         1000000000,
//     );

//     let ret: U256 = env.query_account_named_key(_owner, &[GET_WEIGHTS_SUM_PER_TYPE.into()]);
//     assert_eq!(ret, 500.into());
// }
