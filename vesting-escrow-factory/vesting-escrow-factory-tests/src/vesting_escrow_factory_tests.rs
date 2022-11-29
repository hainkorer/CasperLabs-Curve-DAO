use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::vesting_escrow_factory_instance::VESTINGESCROWFACTORYInstance;

const NAME: &str = "VESTINGESCROWFACTORY";

const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 9;
const INIT_TOTAL_SUPPLY: u64 = 0;
const MILLI_SECONDS_IN_DAY: u64 = 86_400_000;
pub const TEN_E_NINE: u128 = 1000000000;

fn deploy() -> (
    TestEnv,
    VESTINGESCROWFACTORYInstance,
    TestContract,
    AccountHash,
    u64,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let time_now: u64 = VESTINGESCROWFACTORYInstance::now();
    let token: TestContract = VESTINGESCROWFACTORYInstance::erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    let vesting_escrow_factory_instance: TestContract = VESTINGESCROWFACTORYInstance::new_deploy(
        &env,
        NAME,
        owner,
        Key::Hash(token.package_hash()),
        Key::from(owner),
    );
    (
        env,
        VESTINGESCROWFACTORYInstance::instance(vesting_escrow_factory_instance),
        token,
        owner,
        time_now,
    )
}

#[test]
fn test_deploy() {
    let (env, vesting_escrow_factory_instance, target, owner, _) = deploy();
    let _user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
}

#[test]
fn test_commit_transfer_ownership() {
    let (env, vesting_escrow_factory_instance, target, owner, time_now) = deploy();
    let user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
    vesting_escrow_factory_instance.commit_transfer_ownership(owner, time_now, user);
    assert_eq!(vesting_escrow_factory_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_factory_instance.future_admin(), user.into());
}

#[test]
fn test_accept_transfer_ownership() {
    let (env, vesting_escrow_factory_instance, target, owner, time_now) = deploy();
    let user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
    vesting_escrow_factory_instance.commit_transfer_ownership(owner, time_now, user);
    assert_eq!(vesting_escrow_factory_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_factory_instance.future_admin(), user.into());
    vesting_escrow_factory_instance.apply_transfer_ownership(owner, time_now);
    assert_eq!(vesting_escrow_factory_instance.admin(), user.into());
}

#[test]
fn test_deploy_vesting_contract() {
    let (env, vesting_escrow_factory_instance, target, owner, time_now) = deploy();
    let _user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
    let _amount: U256 = U256::from(10 * TEN_E_NINE);
    let _can_disable = false;
    let _vesting_duration: U256 = U256::from(MILLI_SECONDS_IN_DAY * 365);
    let _vesting_start: U256 = U256::from(time_now + MILLI_SECONDS_IN_DAY);

    target.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => Key::from(vesting_escrow_factory_instance.contract_package_hash()) , "amount" => _amount},
        time_now,
    );

    target.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => Key::from(vesting_escrow_factory_instance.contract_package_hash()) , "amount" => _amount},
        time_now,
    );

    vesting_escrow_factory_instance.deploy_vesting_contract(
        owner,
        Key::Hash(target.package_hash()),
        Key::from(owner),
        _amount,
        _can_disable,
        _vesting_duration,
        Some(_vesting_start),
    );
}
