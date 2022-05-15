//! @title Compound's InterestRateModel Interface
//! Provides default implementation as an interface for InterestRateModel contract
//! Must initialize before use with default implementation, if implemented on a contract for non-default implementation

use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs, U256};
/// @notice Indicator that this is an InterestRateModel contract (for inspection)
use contract_utils::set_key;

pub mod constants {
    /// @notice Indicator that this is an InterestRateModel contract (for inspection)
    /// @type bool
    pub const IS_INTEREST_RATE_MODEL: &str = "is_interest_rate_model";

    pub const RUNTIME_ARG_CASH: &str = "cash";
    pub const RUNTIME_ARG_BORROWS: &str = "borrows";
    pub const RUNTIME_ARG_RESERVES: &str = "reserves";
    pub const RUNTIME_ARG_RESERVE_FACTOR_MANTISSA: &str = "reserve_factor_mantissa";

    pub const ENTRYPOINT_GET_BORROW_RATE: &str = "get_borrow_rate";
    pub const ENTRYPOINT_GET_SUPPLY_RATE: &str = "get_supply_rate";
}
/// Provides default implementation as an interface for InterestRateModel contract
/// Must initialize() before use with default implementation, if implemented on a contract for non-default implementation
pub trait InterestRateModel {
    /// @notice Initialized an Indicator that this is an InterestRateModel contract (for inspection)
    fn initialize(&self) {
        set_key(constants::IS_INTEREST_RATE_MODEL, true);
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
            constants::ENTRYPOINT_GET_BORROW_RATE,
            runtime_args! {
                constants::RUNTIME_ARG_CASH=>_cash,
                constants::RUNTIME_ARG_BORROWS=>_borrows,
                constants::RUNTIME_ARG_RESERVES=>_reserves
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
            constants::ENTRYPOINT_GET_SUPPLY_RATE,
            runtime_args! {
                constants::RUNTIME_ARG_CASH=>_cash,
                constants::RUNTIME_ARG_BORROWS=>_borrows,
                constants::RUNTIME_ARG_RESERVES=>_reserves,
                constants::RUNTIME_ARG_RESERVE_FACTOR_MANTISSA => _reserve_factor_mantissa
            },
        )
    }
}
