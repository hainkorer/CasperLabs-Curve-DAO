//! @title Compound's InterestRateModel Interface
//! Provides default implementation as an interface for InterestRateModel contract
//! Must initialize before use with default implementation, if implemented on a contract for non-default implementation

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};
/// @notice Indicator that this is an InterestRateModel contract (for inspection)
pub use compound_protocol_utils::{entrypoints, keys, runtime_arguments};
use contract_utils::set_key;

/// Provides default implementation as an interface for InterestRateModel contract
/// Must initialize() before use with default implementation, if implemented on a contract for non-default implementation
pub trait InterestRateModel {
    /// @notice Initialized an Indicator that this is an InterestRateModel contract (for inspection)
    fn initialize(&self) {
        set_key(keys::IS_INTEREST_RATE_MODEL, true);
    }

    /// @notice Calculates the current borrow interest rate per block
    /// @param cash The total amount of cash the market has
    /// @param borrows The total amount of borrows the market has outstanding
    /// @param reserves The total amount of reserves the market has
    /// @param package_hash The optional ContractPackageHash of an InterestRateModel contract
    /// @return The borrow rate per block (as a percentage, and scaled by 1e18)
    ///
    fn get_borrow_rate(&self, _cash: U256, _borrows: U256, _reserves: U256) -> U256;

    /// @notice Calculates the current supply interest rate per block
    /// @param cash The total amount of cash the market has
    /// @param borrows The total amount of borrows the market has outstanding
    /// @param reserves The total amount of reserves the market has
    /// @param reserveFactorMantissa The current reserve factor the market has
    /// @return The supply rate per block (as a percentage, and scaled by 1e18)
    ///
    fn get_supply_rate(
        &self,
        _cash: U256,
        _borrows: U256,
        _reserves: U256,
        _reserve_factor_mantissa: U256,
    ) -> U256;
}

pub struct InterestRateModelInterface(ContractPackageHash);
impl InterestRateModel for InterestRateModelInterface {
    /// @notice Initialized an Indicator that this is an InterestRateModel contract (for inspection)
    /// In case of trait utilized as contract interface, it is a stub only.
    fn initialize(&self) {}

    /// @notice Calculates the current borrow interest rate per block
    /// @param cash The total amount of cash the market has
    /// @param borrows The total amount of borrows the market has outstanding
    /// @param reserves The total amount of reserves the market has
    /// @param package_hash The optional ContractPackageHash of an InterestRateModel contract
    /// @return The borrow rate per block (as a percentage, and scaled by 1e18)
    ///
    fn get_borrow_rate(&self, _cash: U256, _borrows: U256, _reserves: U256) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            entrypoints::GET_BORROW_RATE,
            runtime_args! {
                runtime_arguments::CASH=>_cash,
                runtime_arguments::BORROWS=>_borrows,
                runtime_arguments::RESERVES=>_reserves
            },
        )
    }

    /// @notice Calculates the current supply interest rate per block
    /// @param cash The total amount of cash the market has
    /// @param borrows The total amount of borrows the market has outstanding
    /// @param reserves The total amount of reserves the market has
    /// @param reserveFactorMantissa The current reserve factor the market has
    /// @return The supply rate per block (as a percentage, and scaled by 1e18)
    ///
    fn get_supply_rate(
        &self,
        _cash: U256,
        _borrows: U256,
        _reserves: U256,
        _reserve_factor_mantissa: U256,
    ) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            entrypoints::GET_SUPPLY_RATE,
            runtime_args! {
                runtime_arguments::CASH=>_cash,
                runtime_arguments::BORROWS=>_borrows,
                runtime_arguments::RESERVES=>_reserves,
                runtime_arguments::RESERVE_FACTOR_MANTISSA => _reserve_factor_mantissa
            },
        )
    }
}
