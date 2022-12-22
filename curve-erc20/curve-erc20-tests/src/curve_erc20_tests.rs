use crate::curve_erc20_instance::*;
use casper_types::{account::AccountHash, runtime_args, RuntimeArgs, U256};
use casperlabs_test_env::{now, TestContract, TestEnv};
use curve_casper_erc20_crate::Address;

fn deploy() -> (TestEnv, AccountHash, TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let token = TestContract::new(
        &env,
        "curve-erc20.wasm",
        "curve-erc20",
        owner,
        runtime_args! {
            "name" => NAME,
            "symbol" => SYMBOL,
            "decimals" => DECIMALS,
            "initial_supply" => INITIAL_SUPPLY,
        },
        now(),
    );
    (env, owner, token)
}

#[test]
fn test_deploy() {
    let (_env, _owner, token) = deploy();
    assert_eq!(NAME, token.query_named_key::<String>("name".into()));
    assert_eq!(SYMBOL, token.query_named_key::<String>("symbol".into()));
    assert_eq!(DECIMALS, token.query_named_key::<u8>("decimals".into()));
    assert_eq!(
        INITIAL_SUPPLY,
        token.query_named_key::<U256>("total_supply".into())
    );
}

#[test]
fn test_set_name_symbol() {
    let (_, owner, token) = deploy();
    assert_eq!(NAME, token.query_named_key::<String>("name".into()));
    assert_eq!(SYMBOL, token.query_named_key::<String>("symbol".into()));
    const NEW_NAME: &str = "new-curve-erc20";
    const NEW_SYMBOL: &str = "NEW-CURVE";
    token.call_contract(
        owner,
        "set_name",
        runtime_args! {
            "name" => NEW_NAME
        },
        now(),
    );
    token.call_contract(
        owner,
        "set_symbol",
        runtime_args! {
            "symbol" => NEW_SYMBOL
        },
        now(),
    );
    assert_eq!(NEW_NAME, token.query_named_key::<String>("name".into()));
    assert_eq!(NEW_SYMBOL, token.query_named_key::<String>("symbol".into()));
}

#[test]
fn test_erc20_mint_burn() {
    let (_, owner, erc20) = deploy();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(owner),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
    erc20.call_contract(
        owner,
        "burn",
        runtime_args! {
            "from" => Address::Account(owner),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
}
#[test]
fn test_erc20_transfer() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        to,
        "transfer",
        runtime_args! {
            "recipient" => Address::Account(owner),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, 0.into());
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
}

#[test]
fn test_erc20_approve_transfer_from() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let tmp_user = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        to,
        "approve",
        runtime_args! {
            "spender" => Address::Account(tmp_user),
            "amount" => amount,
        },
        now(),
    );
    erc20.call_contract(
        tmp_user,
        "transfer_from",
        runtime_args! {
            "owner" => Address::Account(to),
            "recipient" => Address::Account(owner),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, 0.into());
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, amount);
}

#[test]
fn test_erc20_increase_decrease_allowance() {
    let (env, owner, erc20) = deploy();
    let to = env.next_user();
    let amount: U256 = 123_000_000_000u64.into();
    erc20.call_contract(
        owner,
        "mint",
        runtime_args! {
            "to" => Address::Account(to),
            "amount" => amount
        },
        now(),
    );
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(to)));
    assert_eq!(ret, amount);
    let ret: U256 = erc20.query(BALANCES, address_to_str(&Address::Account(owner)));
    assert_eq!(ret, 0.into());
    erc20.call_contract(
        owner,
        "increase_allowance",
        runtime_args! {
            "spender" => Address::Account(to),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(
        ALLOWANCES,
        addresses_to_str(Address::Account(owner), Address::Account(to)),
    );
    assert_eq!(ret, amount);
    erc20.call_contract(
        owner,
        "decrease_allowance",
        runtime_args! {
            "spender" => Address::Account(to),
            "amount" => amount,
        },
        now(),
    );
    let ret: U256 = erc20.query(
        ALLOWANCES,
        addresses_to_str(Address::Account(owner), Address::Account(to)),
    );
    assert_eq!(ret, 0.into());
}
