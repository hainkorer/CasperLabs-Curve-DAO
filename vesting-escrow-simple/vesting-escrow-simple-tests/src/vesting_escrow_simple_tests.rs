// use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U256};
// use casperlabs_test_env::{TestContract, TestEnv};

// use crate::vesting_escrow_simple_instance::VESTINGESCROWSIMPLEInstance;
// pub const TEN_E_NINE: u128 = 1000000000;
// fn deploy_erc20(env: &TestEnv, owner: AccountHash) -> TestContract {
//     TestContract::new(
//         &env,
//         "erc20-token.wasm",
//         "erc2020",
//         owner,
//         runtime_args! {
//             "name" => "ERC",
//             "symbol" => "ERC20",
//             "decimals" => 9_u8,
//             "initial_supply" => U256::from(TEN_E_NINE*1000)
//         },
//         0,
//     )
// }
// fn deploy() -> (
//     TestEnv,
//     AccountHash,
//     VESTINGESCROWSIMPLEInstance,
//     VESTINGESCROWSIMPLEInstance,
//     TestContract,
// ) {
//     let env = TestEnv::new();
//     let owner = env.next_user();
//     let erc20 = deploy_erc20(&env, owner);

//     let contract = VESTINGESCROWSIMPLEInstance::new(
//         &env,
//         "VESTINGESCROWSIMPLE",
//         owner,
//         Key::from_formatted_str(
//             "hash-0000000000000000000000000000000000000000000000000000000000000001".into(),
//         )
//         .unwrap(),
//         Key::Hash(erc20.package_hash()),
//         Key::from_formatted_str(
//             "hash-0000000000000000000000000000000000000000000000000000000000000000"
//         )
//         .unwrap(),
//         1000000000.into(),
//         1000.into(),
//         5000.into(),
//         true,
//     );
//     let proxy = VESTINGESCROWSIMPLEInstance::proxy(
//         &env,
//         "VESTINGESCROWSIMPLEPROXY",
//         owner,
//         Key::Hash(contract.contract_hash()),
//     );

//     (
//         env,
//         owner,
//         VESTINGESCROWSIMPLEInstance::instance(contract),
//         VESTINGESCROWSIMPLEInstance::instance(proxy),
//         erc20,
//     )
// }

// #[test]
// fn test_deploy() {
//     let (_env, _owner, _contract, __, _) = deploy();
// }

// #[test]
// fn toggle_disable() {
//     let (_env, owner, contract, _, _) = deploy();
//     let recipient: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000"
//     )
//     .unwrap();

//     contract.toggle_disable(owner, recipient);
// }
// #[test]
// fn disable_can_disable() {
//     let (_env, owner, contract, _, _) = deploy();
//     contract.disable_can_disable(owner);
// }
// #[test]
// fn vested_of() {
//     let (_env, owner, _, proxy, _) = deploy();
//     let recipient: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000"
//     )
//     .unwrap();
//     proxy.vested_of(owner, recipient);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 1000000000.into());
// }
// #[test]
// fn vested_supply() {
//     let (_env, owner, _, proxy, _) = deploy();
//     proxy.vested_supply(owner);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 1000000000.into());
// }
// #[test]
// fn locked_supply() {
//     let (_env, owner, _, proxy, _) = deploy();
//     proxy.locked_supply(owner);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 1000000000.into());
// }
// #[test]
// fn balance_of() {
//     let (_env, owner, _, proxy, _) = deploy();
//     let recipient: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000"
//     )
//     .unwrap();
//     proxy.balance_of(owner, recipient);
//     let res: U256 = proxy.result();
//     assert_eq!(res, 1000000000.into());
// }
// #[test]
// fn commit_transfer_ownership() {
//     let (_env, owner, contract, _proxy, _) = deploy();
//     let addr: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000001".into(),
//     )
//     .unwrap();
//     contract.commit_transfer_ownership(owner, addr);
// }
// #[test]
// fn apply_transfer_ownership() {
//     let (_env, owner, contract, _proxy, _) = deploy();
//     let addr: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000001".into(),
//     )
//     .unwrap();
//     contract.commit_transfer_ownership(owner, addr);
//     contract.apply_transfer_ownership(owner);
// }
// #[test]
// fn claim() {
//     let (_env, owner, contract, _, erc20) = deploy();
//     let addr: Key = Key::from_formatted_str(
//         "hash-0000000000000000000000000000000000000000000000000000000000000000"
//     )
//     .unwrap();
//     let to: Key = contract.package_hash().into();
//     let amount: U256 = U256::from(TEN_E_NINE * 100);

//     erc20.call_contract(
//         owner,
//         "mint",
//         runtime_args! {"to" => to , "amount" => amount},
//         0,
//     );
//     contract.claim(owner, addr);
// }
