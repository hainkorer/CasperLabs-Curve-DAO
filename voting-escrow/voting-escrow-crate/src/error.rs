use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    InvalidDecimals = 0,
    IsLocked,
    NotAdmin,
    ZeroAddress,
    SmartContractDepositorsNotAllowed,
    NeedNonZeroValue,
    NoExistingLockFound,
    CannotAddToExpiredLockWithdraw,
    WithdrawOldTokensFirst,
    CanOnlyLockUntilTimeInTheFuture,
    VotingLockCanBe4YearsMax,
    LockExpired,
    NothingIsLocked,
    CanOnlyIncreaseLockDuration,
    TheLockDidntExpire,
    InvalidBlockNumber,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
