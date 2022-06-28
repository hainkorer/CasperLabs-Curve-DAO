use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
use casperlabs_test_env::{TestContract, TestEnv};

use crate::gauge_proxy_instance::GAUGEPROXYInstance;

const NAME: &str = "GAUGEPROXY";

fn deploy() -> (TestEnv, GAUGEPROXYInstance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let gauge_proxy =
        GAUGEPROXYInstance::new(&env, NAME, owner, Key::from(owner), Key::from(owner));
    (env, gauge_proxy, owner)
}

#[test]
fn test_deploy() {
    let (env, gauge_proxy, owner) = deploy();
}

#[test]
fn test_commit_set_admins() {
    let (env, gauge_proxy, owner) = deploy();
    let o_admin: Key = Key::Account(env.next_user());
    let e_admin: Key = Key::Account(env.next_user());
    gauge_proxy.commit_set_admins(owner, o_admin, e_admin);
    let ret_o_admin: Key = gauge_proxy.key_value("future_ownership_admin".into());
    let ret_e_admin: Key = gauge_proxy.key_value("future_emergency_admin".into());
    assert_eq!(o_admin, ret_o_admin, "Future ownership admin not set");
    assert_eq!(e_admin, ret_e_admin, "Future emergency admin not set");
}

#[test]
fn test_accept_set_admins() {
    let (env, gauge_proxy, owner) = deploy();
    // Setting ownership admin
    let o_admin: Key = Key::Account(env.next_user());
    let e_admin: Key = Key::Account(env.next_user());
    gauge_proxy.commit_set_admins(owner, o_admin, e_admin);
    gauge_proxy.accept_set_admins(o_admin.into_account().unwrap());
    let ret_o_admin: Key = gauge_proxy.key_value("ownership_admin".into());
    let ret_e_admin: Key = gauge_proxy.key_value("emergency_admin".into());
    assert_eq!(o_admin, ret_o_admin, "Ownership admin not set");
    assert_eq!(e_admin, ret_e_admin, "Emergency admin not set");
}
