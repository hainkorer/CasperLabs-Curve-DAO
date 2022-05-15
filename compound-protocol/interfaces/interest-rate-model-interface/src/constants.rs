/// @notice Indicator that this is an InterestRateModel contract (for inspection)
/// @type bool
pub const IS_INTEREST_RATE_MODEL: &str = "is_interest_rate_model";

pub const RUNTIME_ARG_CASH: &str = "cash";
pub const RUNTIME_ARG_BORROWS: &str = "borrows";
pub const RUNTIME_ARG_RESERVES: &str = "reserves";
pub const RUNTIME_ARG_RESERVE_FACTOR_MANTISSA: &str = "reserve_factor_mantissa";

pub const ENTRYPOINT_GET_BORROW_RATE: &str = "get_borrow_rate";
pub const ENTRYPOINT_GET_SUPPLY_RATE: &str = "get_supply_rate";
