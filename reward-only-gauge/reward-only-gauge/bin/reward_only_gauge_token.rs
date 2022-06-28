#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use alloc::{boxed::Box, collections::BTreeSet, format, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractHash, ContractPackageHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use casperlabs_contract_utils::{ContractContext, OnChainContractStorage};
use reward_only_gauge_crate::REWARDONLYGAUGE;

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl REWARDONLYGAUGE<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        admin: Key,
        lp_token: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
        lock: u64,
    ) {
        REWARDONLYGAUGE::init(
            self,
            admin,
            lp_token,
            Key::from(contract_hash),
            package_hash,
            lock,
        );
    }
}

#[no_mangle]
fn constructor() {
    let admin: Key = runtime::get_named_arg::<Key>("admin");
    let lp_token: Key = runtime::get_named_arg("lp_token");
    let contract_hash: ContractHash = runtime::get_named_arg("contract_hash");
    let package_hash: ContractPackageHash = runtime::get_named_arg("package_hash");
    let lock: u64 = runtime::get_named_arg("lock");

    Token::default().constructor(admin, lp_token, contract_hash, package_hash, lock);
}

#[no_mangle]
fn package_hash() {
    let ret: ContractPackageHash = Token::default().get_package_hash();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn admin() {
    let ret: Key = Token::default().admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn lp_token() {
    let ret: Key = Token::default().lp_token();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn future_admin() {
    let ret: Key = Token::default().future_admin();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn balance_of() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().balance_of(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_balances() {
    let owner: Key = runtime::get_named_arg("owner");
    let ret: U256 = Token::default().reward_balances(owner);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn rewards_receiver() {
    let claimant: Key = runtime::get_named_arg("claimant");
    let reward_receiver: Key = Token::default().rewards_receiver(claimant);
    runtime::ret(CLValue::from_t(reward_receiver).unwrap_or_revert());
}
#[no_mangle]
fn reward_integral() {
    let reward_token: Key = runtime::get_named_arg("reward_token");
    let ret: U256 = Token::default().reward_integral(reward_token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_tokens() {
    let index: U256 = runtime::get_named_arg("index");
    let ret: Key = Token::default().reward_tokens(index);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn claim_sig() {
    let ret: Bytes = Token::default().claim_sig();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn allowance() {
    let owner: Key = runtime::get_named_arg("owner");
    let spender: Key = runtime::get_named_arg("spender");
    let ret: U256 = Token::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}
#[no_mangle]
fn reward_integral_for() {
    let reward_token: Key = runtime::get_named_arg("reward_token");
    let claiming_address: Key = runtime::get_named_arg("claiming_address");
    let integral: U256 = Token::default().reward_integral_for(reward_token, claiming_address);
    runtime::ret(CLValue::from_t(integral).unwrap_or_revert());
}
// #[no_mangle]
// fn claim_data() {
//     let user: Key = runtime::get_named_arg("user");
//     let claiming_address: Key = runtime::get_named_arg("claiming_address");
//     let ret: ClaimDataStruct = Token::default().claim_data(user, claiming_address);
//     runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
// }

#[no_mangle]
fn total_supply() {
    let ret: U256 = Token::default().total_supply();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn symbol() {
    let ret: String = Token::default().symbol();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn name() {
    let ret: String = Token::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Get the number of decimals for this token
/// @dev Implemented as a view method to reduce gas costs
/// @return uint256 decimal places
/// """
#[no_mangle]
fn decimals() {
    let ret: u8 = Token::default().decimals();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Accept a pending ownership transfer
/// """

#[no_mangle]
fn accept_transfer_ownership() {
    Token::default().accept_transfer_ownership();
}

/// """
/// @notice Transfer ownership of GaugeController to `addr`
/// @param addr Address to have ownership transferred to
/// """

#[no_mangle]
fn commit_transfer_ownership() {
    let addr: Key = runtime::get_named_arg("addr");
    Token::default().commit_transfer_ownership(addr);
}

/// """
/// @notice Transfer token for a specified address
/// @dev Transferring claims pending reward tokens for the sender and receiver
/// @param _to The address to transfer to.
/// @param _value The amount to be transferred.
/// """

#[no_mangle]
fn transfer() {
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = Token::default().transfer(recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Transfer tokens from one address to another.
/// @dev Transferring claims pending reward tokens for the sender and receiver
/// @param _from address The address which you want to send tokens from
/// @param _to address The address which you want to transfer to
/// @param _value uint256 the amount of tokens to be transferred
/// """
#[no_mangle]
fn transfer_from() {
    let owner: Key = runtime::get_named_arg("owner");
    let recipient: Key = runtime::get_named_arg("recipient");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = Token::default().transfer_from(owner, recipient, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Approve the passed address to transfer the specified amount of
///         tokens on behalf of msg.sender
/// @dev Beware that changing an allowance via this method brings the risk
///         that someone may use both the old and new allowance by unfortunate
///         transaction ordering. This may be mitigated with the use of
///         {incraseAllowance} and {decreaseAllowance}.
///         https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
/// @param spender The address which will transfer the funds
/// @param _value The amount of tokens that may be transferred
/// @return bool success
/// """
#[no_mangle]
fn approve() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    Token::default().approve(spender, amount);
}

/// """
/// @notice Increase the allowance granted to `spender` by the caller
/// @dev This is alternative to {approve} that can be used as a mitigation for
///      the potential race condition
/// @param spender The address which will transfer the funds
/// @param amount The amount of to increase the allowance
/// @return bool success
/// """
#[no_mangle]
fn increase_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");
    let ret: Result<(), u32> = Token::default().increase_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Decrease the allowance granted to `spender` by the caller
/// @dev This is alternative to {approve} that can be used as a mitigation for
///      the potential race condition
/// @param spender The address which will transfer the funds
/// @param amount The amount of to decrease the allowance
/// @return bool success
/// """
#[no_mangle]
fn decrease_allowance() {
    let spender: Key = runtime::get_named_arg("spender");
    let amount: U256 = runtime::get_named_arg("amount");

    let ret: Result<(), u32> = Token::default().decrease_allowance(spender, amount);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Address of the reward contract providing non-CRV incentives for this gauge
/// @dev Returns `ZERO_ADDRESS` if there is no reward contract active
/// """
#[no_mangle]
fn reward_contract() {
    let ret = Token::default().reward_contract();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Epoch timestamp of the last call to claim from `reward_contract`
/// @dev Rewards are claimed at most once per hour in order to reduce gas costs
/// """
#[no_mangle]
fn last_claim() {
    let ret = Token::default().last_claim();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Get the number of already-claimed reward tokens for a user
/// @param _addr Account to get reward amount for
/// @param _token Token to get reward amount for
/// @return uint256 Total amount of `_token` already claimed by `_addr`
/// """
#[no_mangle]
fn claimed_reward() {
    let _addr: Key = runtime::get_named_arg("_addr");
    let _token: Key = runtime::get_named_arg("_token");

    let ret = Token::default().claimed_reward(_addr, _token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Get the number of claimable reward tokens for a user
/// @dev This call does not consider pending claimable amount in `reward_contract`.
///      Off-chain callers should instead use `claimable_rewards_write` as a
///      view method.
/// @param _addr Account to get reward amount for
/// @param _token Token to get reward amount for
/// @return uint256 Claimable reward token amount
/// """

#[no_mangle]
fn claimable_reward() {
    let _addr: Key = runtime::get_named_arg("_addr");
    let _token: Key = runtime::get_named_arg("_token");

    let ret: U256 = Token::default().claimable_reward(_addr, _token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Set the default reward receiver for the caller.
/// @dev When set to ZERO_ADDRESS, rewards are sent to the caller
/// @param _receiver Receiver address for any rewards claimed via `claim_rewards`
/// """
#[no_mangle]
fn set_rewards_receiver() {
    let _receiver: Key = runtime::get_named_arg("_receiver");
    Token::default().set_rewards_receiver(_receiver);
}

/// """
/// @notice Get the number of claimable reward tokens for a user
/// @dev This function should be manually changed to "view" in the ABI
///      Calling it via a transaction will claim available reward tokens
/// @param _addr Account to get reward amount for
/// @param _token Token to get reward amount for
/// @return uint256 Claimable reward token amount
/// """
#[no_mangle]
fn claimable_reward_write() {
    let _addr: Key = runtime::get_named_arg("_addr");
    let _token: Key = runtime::get_named_arg("_token");

    let ret: U256 = Token::default().claimable_reward_write(_addr, _token);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

/// """
/// @notice Claim available reward tokens for `_addr`
/// @param _addr Address to claim for
/// @param _receiver Address to transfer rewards to - if set to
///                  ZERO_ADDRESS, uses the default reward receiver
///                  for the caller
/// """
#[no_mangle]
fn claim_rewards() {
    let _addr: Option<Key> = runtime::get_named_arg("_addr");
    let _receiver: Option<Key> = runtime::get_named_arg("_token");

    Token::default().claim_rewards(_addr, _receiver);
}

/// """
/// @notice Set the active reward contract
/// @dev A reward contract cannot be set while this contract has no deposits
/// @param _reward_contract Reward contract address. Set to ZERO_ADDRESS to
///                         disable staking.
/// @param _claim_sig Four byte selectors for staking, withdrawing and claiming,
///             left padded with zero bytes. If the reward contract can
///             be claimed from but does not require staking, the staking
///             and withdraw selectors should be set to 0x00
/// @param _reward_tokens List of claimable reward tokens. New reward tokens
///                     may be added but they cannot be removed. When calling
///                     this function to unset or modify a reward contract,
///                     this array must begin with the already-set reward
///                     token addresses.
/// """
#[no_mangle]
fn set_rewards() {
    let _reward_contract: Key = runtime::get_named_arg("_reward_contract");
    let _claim_sig: Bytes = runtime::get_named_arg("_claim_sig");
    let _reward_tokens: Vec<String> = runtime::get_named_arg("_reward_tokens");

    Token::default().set_rewards(_reward_contract, _claim_sig, _reward_tokens);
}

/// """
/// @notice Withdraw `_value` LP tokens
/// @dev Withdrawing also claims pending reward tokens
/// @param _value Number of tokens to withdraw
/// """
#[no_mangle]
fn withdraw() {
    let _value: U256 = runtime::get_named_arg("_value");
    let _claim_rewards: Option<bool> = runtime::get_named_arg("_claim_rewards");

    Token::default().withdraw(_value, _claim_rewards);
}

/// """
/// @notice Deposit `_value` LP tokens
/// @dev Depositting also claims pending reward tokens
/// @param _value Number of tokens to deposit
/// @param _addr Address to deposit for
/// """
#[no_mangle]
fn deposit() {
    let _value: U256 = runtime::get_named_arg("_value");
    let _addr: Option<Key> = runtime::get_named_arg("_addr");
    let _claim_rewards: Option<bool> = runtime::get_named_arg("_claim_rewards");

    Token::default().deposit(_value, _addr, _claim_rewards);
}
#[no_mangle]
fn call() {
    // Contract name must be same for all new versions of the contracts
    let contract_name: String = runtime::get_named_arg("contract_name");

    // If this is the first deployment
    if !runtime::has_key(&format!("{}_package_hash", contract_name)) {
        // Build new package with initial a first version of the contract.
        let (package_hash, access_token) = storage::create_contract_package_at_hash();
        let (contract_hash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());
        // Read arguments for the constructor call.
        let admin: Key = runtime::get_named_arg("admin");
        let lp_token: Key = runtime::get_named_arg("lp_token");
        let lock: u64 = 0;
        // Prepare constructor args
        let constructor_args = runtime_args! {
            "admin" => admin,
            "lp_token" => lp_token,
            "contract_hash" => contract_hash,
            "package_hash"=> package_hash,
            "lock"=>lock

        };

        // Add the constructor group to the package hash with a single URef.
        let constructor_access: URef =
            storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
                .unwrap_or_revert()
                .pop()
                .unwrap_or_revert();

        // Call the constructor entry point
        let _: () =
            runtime::call_versioned_contract(package_hash, None, "constructor", constructor_args);

        // Remove all URefs from the constructor group, so no one can call it for the second time.
        let mut urefs = BTreeSet::new();
        urefs.insert(constructor_access);
        storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
            .unwrap_or_revert();

        // Store contract in the account's named keys.
        runtime::put_key(
            &format!("{}_package_hash", contract_name),
            package_hash.into(),
        );
        runtime::put_key(
            &format!("{}_package_hash_wrapped", contract_name),
            storage::new_uref(package_hash).into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
        runtime::put_key(
            &format!("{}_package_access_token", contract_name),
            access_token.into(),
        );
    } else {
        // this is a contract upgrade

        let package_hash: ContractPackageHash =
            runtime::get_key(&format!("{}_package_hash", contract_name))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();

        let (contract_hash, _): (ContractHash, _) =
            storage::add_contract_version(package_hash, get_entry_points(), Default::default());

        // update contract hash
        runtime::put_key(
            &format!("{}_contract_hash", contract_name),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{}_contract_hash_wrapped", contract_name),
            storage::new_uref(contract_hash).into(),
        );
    }
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("admin", Key::cl_type()),
            Parameter::new("lp_token", Key::cl_type()),
            Parameter::new("contract_hash", ContractHash::cl_type()),
            Parameter::new("package_hash", ContractPackageHash::cl_type()),
            Parameter::new("lock", u64::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "package_hash",
        vec![],
        ContractPackageHash::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "lp_token",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "future_admin",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "balance_of",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_balances",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "rewards_receiver",
        vec![Parameter::new("owner", Key::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_integral",
        vec![Parameter::new("owner", Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_tokens",
        vec![Parameter::new("index", U256::cl_type())],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_sig",
        vec![],
        Bytes::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "allowance",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("spender", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_integral_for",
        vec![
            Parameter::new("reward_token", Key::cl_type()),
            Parameter::new("claiming_address", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    // entry_points.add_entry_point(EntryPoint::new(
    //     "claim_data",
    //     vec![
    //         Parameter::new("user", Key::cl_type()),
    //         Parameter::new("claiming_address", Key::cl_type()),
    //     ],
    //     ClaimDataSturct::cl_type(),
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract,
    // ));

    // entry_points.add_entry_point(EntryPoint::new(
    //     "reward_data",
    //     vec![],
    //     U256::cl_type(),
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract,
    // ));

    entry_points.add_entry_point(EntryPoint::new(
        "total_supply",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "symbol",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decimals",
        vec![],
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "commit_transfer_ownership",
        vec![Parameter::new("addr", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "accept_transfer_ownership",
        vec![],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "decrease_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "increase_allowance",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer",
        vec![
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_from",
        vec![
            Parameter::new("owner", Key::cl_type()),
            Parameter::new("recipient", Key::cl_type()),
            Parameter::new("amount", U256::cl_type()),
        ],
        CLType::Result {
            ok: Box::new(CLType::Unit),
            err: Box::new(CLType::U32),
        },
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "reward_contract",
        vec![],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "last_claim",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimed_reward",
        vec![
            Parameter::new("_addr", Key::cl_type()),
            Parameter::new("_token", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_reward",
        vec![
            Parameter::new("_addr", Key::cl_type()),
            Parameter::new("_token", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "set_rewards_receiver",
        vec![Parameter::new("_receiver", Key::cl_type())],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claimable_reward_write",
        vec![
            Parameter::new("_addr", Key::cl_type()),
            Parameter::new("_token", Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "claim_rewards",
        vec![
            Parameter::new("_addr", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("_receiver", CLType::Option(Box::new(CLType::Key))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "deposit",
        vec![
            Parameter::new("_value", U256::cl_type()),
            Parameter::new("_addr", CLType::Option(Box::new(CLType::Key))),
            Parameter::new("_claim_rewards", CLType::Option(Box::new(bool::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("_value", U256::cl_type()),
            Parameter::new("_claim_rewards", CLType::Option(Box::new(bool::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "set_rewards",
        vec![
            Parameter::new("_reward_contract", Key::cl_type()),
            Parameter::new("_claim_sig", Bytes::cl_type()),
            Parameter::new("_reward_tokens", CLType::List(Box::new(String::cl_type()))),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
