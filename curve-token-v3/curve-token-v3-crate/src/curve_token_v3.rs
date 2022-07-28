use crate::{data::*, event::CurveTokenV3Event};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{
        runtime::{self},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, URef, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::*;

pub trait CURVETOKENV3<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &self,
        name: String,
        symbol: String,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        set_name(name);
        set_symbol(symbol);
        set_minter(self.get_caller());
        Balances::init();
        Allowances::init();
        set_total_supply(0.into());
        set_hash(contract_hash);
        set_package_hash(package_hash);
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: zero_address(),
            to: self.get_caller(),
            value: 0.into(),
        });
    }
    /// @notice Get the number of decimals for this token
    //// @return U256 decimal places
    fn decimals(&self) -> U256 {
        9.into()
    }
    /// @dev Transfer token for a specified address
    /// @param recipient The address to transfer to.
    /// @param amount The amount to be transferred.
    fn transfer(&self, recipient: Key, amount: U256) -> Result<(), u32> {
        Balances::instance().set(
            &self.get_caller(),
            Balances::instance()
                .get(&self.get_caller())
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::CurveTokenV3UnderFlow1),
        );
        Balances::instance().set(
            &recipient,
            Balances::instance()
                .get(&recipient)
                .checked_add(amount)
                .unwrap_or_revert_with(Error::CurveTokenV3OverFlow1),
        );
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: self.get_caller(),
            to: recipient,
            value: amount,
        });
        Ok(())
    }
    /// @dev Transfer tokens from one address to another.
    ///  @param  owner address The address which you want to send tokens from
    ///  @param recipient address The address which you want to transfer to
    ///  @param amount U256 the amount of tokens to be transferred
    fn transfer_from(&self, owner: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        Balances::instance().set(
            &owner,
            Balances::instance()
                .get(&owner)
                .checked_sub(amount)
                .unwrap_or_revert_with(Error::CurveTokenV3UnderFlow2),
        );
        Balances::instance().set(
            &recipient,
            Balances::instance()
                .get(&recipient)
                .checked_add(amount)
                .unwrap_or_revert_with(Error::CurveTokenV3OverFlow2),
        );
        let allowance: U256 = Allowances::instance().get(&owner, &self.get_caller());
        if allowance != U256::MAX {
            Allowances::instance().set(
                &owner,
                &self.get_caller(),
                allowance
                    .checked_sub(amount)
                    .unwrap_or_revert_with(Error::CurveTokenV3UnderFlow8),
            )
        }
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: owner,
            to: recipient,
            value: amount,
        });

        Ok(())
    }
    /// @notice Approve the passed address to transfer the specified amount of
    ///         tokens on behalf of self.get_caller
    /// @dev Beware that changing an allowance via this method brings the risk
    ///      that someone may use both the old and new allowance by unfortunate
    ///      transaction ordering. This may be mitigated with the use of
    ///      {increase_allowance} and {decrease_allowance}.
    /// @param spender The address which will transfer the funds
    /// @param amount The amount of tokens that may be transferred
    fn approve(&self, spender: Key, amount: U256) {
        Allowances::instance().set(&self.get_caller(), &spender, amount);
        self.curve_token_v3_emit(&CurveTokenV3Event::Approval {
            owner: self.get_caller(),
            spender,
            value: amount,
        });
    }
    /// @notice Increase the allowance granted to `spender` by the caller
    /// @dev This is alternative to {approve} that can be used as a mitigation for
    ///      the potential race condition
    /// @param spender The address which will transfer the funds
    /// @param added_value The amount of to increase the allowance
    /// @return ok success
    fn increase_allowance(&self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowances::instance()
            .get(&self.get_caller(), &spender)
            .checked_add(amount)
            .unwrap_or_revert_with(Error::CurveTokenV3OverFlow6);
        Allowances::instance().set(&self.get_caller(), &spender, allowance);
        self.curve_token_v3_emit(&CurveTokenV3Event::Approval {
            owner: self.get_caller(),
            spender,
            value: amount,
        });
        Ok(())
    }
    /// @notice Decrease the allowance granted to `spender` by the caller
    /// @dev This is alternative to {approve} that can be used as a mitigation for
    ///      the potential race condition
    /// @param spender The address which will transfer the funds
    /// @param amount The amount of to decrease the allowance
    /// @return ok success
    fn decrease_allowance(&self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowances::instance()
            .get(&self.get_caller(), &spender)
            .checked_sub(amount)
            .unwrap_or_revert_with(Error::CurveTokenV3UnderFlow7);
        Allowances::instance().set(&self.get_caller(), &spender, allowance);
        self.curve_token_v3_emit(&CurveTokenV3Event::Approval {
            owner: self.get_caller(),
            spender,
            value: amount,
        });
        Ok(())
    }
    /// @dev mint an amount of the token and assigns it to an account.
    ///      This encapsulates the modification of balances such that the
    ///      proper events are emitted.
    /// @param to The account that will receive the created tokens.
    /// @param amount The amount that will be created.
    fn mint(&self, to: Key, amount: U256) -> bool {
        if self.get_caller() != get_minter() {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed));
        }
        set_total_supply(
            get_total_supply()
                .checked_add(amount)
                .unwrap_or_revert_with(Error::CurveTokenV3OverFlow3),
        );
        Balances::instance().set(
            &to,
            Balances::instance()
                .get(&to)
                .checked_add(amount)
                .unwrap_or_revert_with(Error::CurveTokenV3OverFlow4),
        );
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: zero_address(),
            to,
            value: amount,
        });
        true
    }
    /// @dev Burn an amount of the token from a given account.
    /// @param to The account whose tokens will be burned.
    /// @param value The amount that will be burned.
    /// @return bool success
    fn burn_from(&self, to: Key, value: U256) -> bool {
        if self.get_caller() != get_minter() {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterAllowed2));
        }
        set_total_supply(
            get_total_supply()
                .checked_sub(value)
                .unwrap_or_revert_with(Error::CurveTokenV3UnderFlow5),
        );
        Balances::instance().set(
            &to,
            Balances::instance()
                .get(&to)
                .checked_sub(value)
                .unwrap_or_revert_with(Error::CurveTokenV3UnderFlow6),
        );
        self.curve_token_v3_emit(&CurveTokenV3Event::Transfer {
            from: to,
            to: zero_address(),
            value,
        });
        true
    }

    /// @dev set minter for a specified address
    /// @param minter The address to assign minter role.
    fn set_minter(&self, minter: Key) {
        if self.get_caller() != get_minter() {
            runtime::revert(ApiError::from(Error::CurveTokenV3OnlyMinterCanSet));
        }
        set_minter(minter);
    }
    /// @dev set name and symbol
    /// @param name.
    /// @param symbol.
    fn set_name(&self, name: String, symbol: String) {
        if self.get_caller()
            != runtime::call_versioned_contract(
                get_minter().into_hash().unwrap_or_revert().into(),
                None,
                "owner",
                runtime_args! {},
            )
        {
            runtime::revert(ApiError::from(Error::CurveTokenV3NotAuthorized));
        }
        set_name(name);
        set_symbol(symbol);
    }
    fn curve_token_v3_emit(&self, curve_token_v3_event: &CurveTokenV3Event) {
        let mut events = Vec::new();
        let package = get_package_hash();
        match curve_token_v3_event {
            CurveTokenV3Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_token_v3_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            CurveTokenV3Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_token_v3_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
