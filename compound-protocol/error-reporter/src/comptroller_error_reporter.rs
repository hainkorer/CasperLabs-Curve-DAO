use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    NoError,
    Unauthorized,
    ComptrollerMismatch,
    InsufficientShortfall,
    InsufficientLiquidity,
    InvalidCloseFactor,
    InvalidCollateralFactor,
    InvalidLiquidationIncentive,
    MarketNotEnteredNoLongerPossible,
    MarketNotListed,
    MarketAlreadyListed,
    MathError,
    NonzeroBorrowBalance,
    PriceError,
    Rejection,
    SnapshotError,
    TooManyAssets,
    TooMuchRepay,
}
impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[repr(u16)]
pub enum FailureInfo {
    AcceptAdminPendingAdminCheck,
    AcceptPendingImplementationAddressCheck,
    ExitMarketBalanceOwed,
    ExitMarketRejection,
    SetCloseFactorOwnerCheck,
    SetCloseFactorValidation,
    SetCollateralFactorOwnerCheck,
    SetCollateralFactorNoExists,
    SetCollateralFactorValidation,
    SetCollateralFactorWithoutPrice,
    SetImplementationOwnerCheck,
    SetLiquidationIncentiveOwnerCheck,
    SetLiquidationIncentiveValidation,
    SetMaxAssetsOwnerCheck,
    SetPendingAdminOwnerCheck,
    SetPendingImplementationOwnerCheck,
    SetPriceOracleOwnerCheck,
    SupportMarketExists,
    SupportMarketOwnerCheck,
    SetPauseGuardianOwnerCheck,
}
impl From<FailureInfo> for ApiError {
    fn from(error: FailureInfo) -> ApiError {
        ApiError::User(error as u16)
    }
}
pub trait ComptrollerErrorReporter {
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
