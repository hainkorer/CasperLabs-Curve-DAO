use crate::i_reward_distribution_recipient_instance::IREWARDDISTRIBUTIONRECIPIENTInstance;
use casper_types::{account::AccountHash, Key};
use casperlabs_test_env::{TestContract, TestEnv};
fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance =
        IREWARDDISTRIBUTIONRECIPIENTInstance::new(&env, "IREWARDDISTRIBUTIONRECIPIENT", owner);
    (env, owner, instance)
}

#[test]
fn test_deploy() {
    let (_, _, _) = deploy();
}
#[test]
fn test_set_reward_distribution() {
    let (_, owner, instance) = deploy();
    let i_reward_distribution_recipient_instance =
        IREWARDDISTRIBUTIONRECIPIENTInstance::contract_instance(instance);
    i_reward_distribution_recipient_instance.set_reward_distribution(owner, Key::Account(owner));
}
