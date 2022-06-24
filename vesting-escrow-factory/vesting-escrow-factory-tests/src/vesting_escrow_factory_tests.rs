use casper_types::{account::AccountHash, Key, U256};
use test_env::{TestContract, TestEnv};

use crate::vesting_escrow_factory_instance::VESTINGESCROWFACTORYInstance;

const NAME: &str = "VESTINGESCROWFACTORY";

const TOKEN_NAME: &str = "ERC20";
const TOKEN_SYMBOL: &str = "ERC";
const DECIMALS: u8 = 8;
const INIT_TOTAL_SUPPLY: u64 = 0;

fn deploy() -> (
    TestEnv,
    VESTINGESCROWFACTORYInstance,
    TestContract,
    AccountHash,
    // VESTINGESCROWFACTORYInstance,
    // VESTINGESCROWFACTORYInstance,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token: TestContract = VESTINGESCROWFACTORYInstance::erc20(
        &env,
        owner,
        TOKEN_NAME,
        TOKEN_SYMBOL,
        DECIMALS,
        INIT_TOTAL_SUPPLY.into(),
    );
    let vesting_escrow_factory_instance: TestContract = VESTINGESCROWFACTORYInstance::new(
        &env,
        NAME,
        owner,
        Key::Hash(token.package_hash()),
        Key::from(owner),
        // Key::Hash(_token.package_hash()),
    );
    (
        env,
        VESTINGESCROWFACTORYInstance::instance(vesting_escrow_factory_instance),
        token,
        owner,
    )
}

#[test]
fn test_deploy() {
    let (env, vesting_escrow_factory_instance, target, owner) = deploy();
    let _user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
}

#[test]
fn test_commit_transfer_ownership() {
    let (env, vesting_escrow_factory_instance, target, owner) = deploy();
    let user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
    vesting_escrow_factory_instance.commit_transfer_ownership(owner, user);
    assert_eq!(vesting_escrow_factory_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_factory_instance.future_admin(), user.into());
}

#[test]
fn test_accept_transfer_ownership() {
    let (env, vesting_escrow_factory_instance, target, owner) = deploy();
    let user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
    vesting_escrow_factory_instance.commit_transfer_ownership(owner, user);
    assert_eq!(vesting_escrow_factory_instance.admin(), owner.into());
    assert_eq!(vesting_escrow_factory_instance.future_admin(), user.into());
    vesting_escrow_factory_instance.apply_transfer_ownership(owner);
    assert_eq!(vesting_escrow_factory_instance.admin(), user.into());
}

#[test]
fn test_deploy_vesting_contract() {
    let (env, vesting_escrow_factory_instance, target, owner) = deploy();
    let user = env.next_user();
    assert_eq!(vesting_escrow_factory_instance.admin(), Key::from(owner));
    assert_eq!(
        vesting_escrow_factory_instance.target(),
        Key::Hash(target.package_hash())
    );
    let _amount: U256 = 10.into();
    let _can_disable = false;
    let _vesting_duration: U256 = 20304001.into();
    let _vesting_start: U256 = 100000.into();
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
