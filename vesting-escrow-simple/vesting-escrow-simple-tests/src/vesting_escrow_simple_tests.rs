use casper_types::{
    account::AccountHash, runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512,
};
use test_env::{TestContract, TestEnv};

use crate::vesting_escrow_simple_instance::VESTINGESCROWSIMPLEInstance;
pub const TEN_E_NINE: u128 = 1000000000;
fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
    TestContract::new(
        &env,
        "erc20-token.wasm",
        "erc2020",
        owner,
        runtime_args! {
            "name" => "ERC",
            "symbol" => "ERC20",
            "decimals" => 9 as u8,
            "initial_supply" => U256::from(TEN_E_NINE*1000)
        },
        0,
    )
}
fn deploy() -> (
    TestEnv,
    AccountHash,
    TestContract,
    TestContract,
    TestContract,
) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let erc20 = deploy_erc20(&env, owner);

    let contract = VESTINGESCROWSIMPLEInstance::new(
        &env,
        "VESTINGESCROWSIMPLE",
        owner,
        Key::Hash(erc20.package_hash()),
    );
    let proxy = VESTINGESCROWSIMPLEInstance::proxy(
        &env,
        "VESTINGESCROWSIMPLEPROXY",
        owner,
        Key::Hash(contract.contract_hash()),
    );
    let key: ContractPackageHash =
        contract.query_named_key("self_contract_package_hash".to_string());
    let to: Key = Key::from(key);
    let amount: U256 = U256::from(TEN_E_NINE * 100);

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

    (env, owner, contract, proxy, erc20)
}

#[test]
fn test_deploy() {
    let (env, owner, contract, __, _) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    // assert_eq!(contract.start_time(), 0.into());
    // assert_eq!(contract.end_time(), 0.into());
}

#[test]
fn toggle_disable() {
    let (env, owner, contract, proxy, erc20) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let recipient: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let recipient: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000001".into(),
    )
    .unwrap();
    let to: Key = proxy.package_hash().into();
    let amount: U256 = 1000000000.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to, "amount" => amount},
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => spender , "amount" => amount},
        0,
    );
    proxy.initialize(
        owner,
        proxy.package_hash().into(),
        Key::Hash(erc20.package_hash()),
        recipient,
        amount,
        0.into(),
        5.into(),
        true,
    );
    // let _recipient_arg: Key = Key::Account(owner);
    proxy.toggle_disable(owner, recipient);
}
//#[test]
fn disable_can_disable() {
    let (env, owner, contract, proxy, erc20) = deploy();
    let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
    let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
    let recipient: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap();
    let spender: Key = Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000001".into(),
    )
    .unwrap();
    let to: Key = proxy.package_hash().into();
    let amount: U256 = 1000000000.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {"to" => to, "amount" => amount},
        0,
    );
    erc20.call_contract(
        owner,
        "approve",
        runtime_args! {"spender" => spender , "amount" => amount},
        0,
    );
    proxy.initialize(
        owner,
        proxy.package_hash().into(),
        Key::Hash(erc20.package_hash()),
        recipient,
        amount,
        0.into(),
        5.into(),
        true,
    );
    contract.disable_can_disable(owner);
}
// //#[test]
// fn vested_of() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
//     let recipient_arg: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
//     )
//     .unwrap();
//     proxy.vested_of(owner, recipient_arg);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 100.into());
// }
// //#[test]
// fn vested_supply() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
//     proxy.vested_supply(owner);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 0.into());
// }
// //#[test]
// fn locked_supply() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
//     proxy.locked_supply(owner);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 100.into());
// }
// //#[test]
// fn balance_of_vest() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
//     let recipient_arg: Key = Key::Account(owner);
//     proxy.balance_of_vest(owner, recipient_arg);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 0.into());
// }
// //#[test]
// fn commit_transfer_ownership() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
//     let addr: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
//     )
//     .unwrap();
//     proxy.commit_transfer_ownership(owner, addr);
//     let res: bool = proxy.result();
//     assert_eq!(res, true);
// }
// //#[test]
// fn apply_transfer_ownership() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let proxy = VESTINGESCROWSIMPLEInstance::contract_instance(proxy);
//     proxy.apply_transfer_ownership(owner);
//     let res: bool = proxy.result();
//     assert_eq!(res, true);
// }
// //#[test]
// fn claim() {
//     let (env, owner, contract, proxy) = deploy();
//     let contract = VESTINGESCROWSIMPLEInstance::contract_instance(contract);
//     let addr_arg: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
//     )
//     .unwrap();

//     contract.claim(owner, addr_arg);
// }
