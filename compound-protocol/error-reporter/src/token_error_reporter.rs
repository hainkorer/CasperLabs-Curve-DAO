use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    NoError,
    Unauthorized,
    BadInput,
    ComptrollerRejection,
    ComptrollerCalculationError,
    InterestRateModelError,
    InvalidAccountPair,
    InvalidCloseAmountRequested,
    InvalidCollateralFactor,
    MathError,
    MarketNotFresh,
    MarketNotListed,
    TokenInsufficientAllowance,
    TokenInsufficientBalance,
    TokenInsufficientCash,
    TokenTransferInFailed,
    TokenTransferOutFailed,
}
impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

/*
* Note: FailureInfo (but not Error) is kept in alphabetical order
*       This is because FailureInfo grows significantly faster, and
*       the order of Error has some meaning, while the order of FailureInfo
*       is entirely arbitrary.
*/
#[repr(u16)]
pub enum FailureInfo {
    AcceptAdminPendingAdminCheck,
    AccrueInterestAccumulatedInterestCalculationFailed,
    AccrueInterestBorrowRateCalculationFailed,
    AccrueInterestNewBorrowIndexCalculationFailed,
    AccrueInterestNewTotalBorrowsCalculationFailed,
    AccrueInterestNewTotalReservesCalculationFailed,
    AccrueInterestSimpleInterestFactorCalculationFailed,
    BorrowAccumulatedBalanceCalculationFailed,
    BorrowAccrueInterestFailed,
    BorrowCashNotAvailable,
    BorrowFreshnessCheck,
    BorrowNewTotalBalanceCalculationFailed,
    BorrowNewAccountBorrowBalanceCalculationFailed,
    BorrowMarketNotListed,
    BorrowComptrollerRejection,
    LiquidateAccrueBorrowInterestFailed,
    LiquidateAccrueCollateralInterestFailed,
    LiquidateCollateralFreshnessCheck,
    LiquidateComptrollerRejection,
    LiquidateComptrollerCalculateAmountSeizeFailed,
    LiquidateCloseAmountIsUintMax,
    LiquidateCloseAmountIsZero,
    LiquidateFreshnessCheck,
    LiquidateLiquidatorIsBorrower,
    LiquidateRepayBorrowFreshFailed,
    LiquidateSeizeBalanceIncrementFailed,
    LiquidateSeizeBalanceDecrementFailed,
    LiquidateSeizeComptrollerRejection,
    LiquidateSeizeLiquidatorIsBorrower,
    LiquidateSeizeTooMuch,
    MintAccrueInterestFailed,
    MintComptrollerRejection,
    MintExchangeCalculationFailed,
    MintExchangeRateReadFailed,
    MintFreshnessCheck,
    MintNewAccountBalanceCalculationFailed,
    MintNewTotalSupplyCalculationFailed,
    MintTransferInFailed,
    MintTransferInNotPossible,
    RedeemAccrueInterestFailed,
    RedeemComptrollerRejection,
    RedeemExchangeTokensCalculationFailed,
    RedeemExchangeAmountCalculationFailed,
    RedeemExchangeRateReadFailed,
    RedeemFreshnessCheck,
    RedeemNewAccountBalanceCalculationFailed,
    RedeemNewTotalSupplyCalculationFailed,
    RedeemTransferOutNotPossible,
    ReduceReservesAccrueInterestFailed,
    ReduceReservesAdminCheck,
    ReduceReservesCashNotAvailable,
    ReduceReservesFreshCheck,
    ReduceReservesValidation,
    RepayBehalfAccrueInterestFailed,
    RepayBorrowAccrueInterestFailed,
    RepayBorrowAccumulatedBalanceCalculationFailed,
    RepayBorrowComptrollerRejection,
    RepayBorrowFreshnessCheck,
    RepayBorrowNewAccountBorrowBalanceCalculationFailed,
    RepayBorrowNewTotalBalanceCalculationFailed,
    RepayBorrowTransferInNotPossible,
    SetCollateralFactorOwnerCheck,
    SetCollateralFactorValidation,
    SetComptrollerOwnerCheck,
    SetInterestRateModelAccrueInterestFailed,
    SetInterestRateModelFreshCheck,
    SetInterestRateModelOwnerCheck,
    SetMaxAssetsOwnerCheck,
    SetOracleMarketNotListed,
    SetPendingAdminOwnerCheck,
    SetReserveFactorAccrueInterestFailed,
    SetReserveFactorAdminCheck,
    SetReserveFactorFreshCheck,
    SetReserveFactorBoundsCheck,
    TransferComptrollerRejection,
    TransferNotAllowed,
    TransferNotEnough,
    TransferTooMuch,
    AddReservesAccrueInterestFailed,
    AddReservesFreshCheck,
    AddReservesTransferInNotPossible,
}
impl From<FailureInfo> for ApiError {
    fn from(error: FailureInfo) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait TokenErrorReporter {
    // /**
    //   * @dev `error` corresponds to enum Error; `info` corresponds to enum FailureInfo, and `detail` is an arbitrary
    //   * contract-specific code that enables us to report opaque error codes from upgradeable contracts.
    //   **/
    // event Failure(uint error, uint info, uint detail);

    // /**
    //   * @dev use this when reporting a known error from the money market or a non-upgradeable collaborator
    //   */
    // function fail(Error err, FailureInfo info) internal returns (uint) {
    //     emit Failure(uint(err), uint(info), 0);

    //     return uint(err);
    // }

    // /**
    //   * @dev use this when reporting an opaque error from an upgradeable collaborator contract
    //   */
    // function failOpaque(Error err, FailureInfo info, uint opaqueError) internal returns (uint) {
    //     emit Failure(uint(err), uint(info), opaqueError);

    //     return uint(err);
    // }
}
