use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    Abort = 0,
    InvalidTokenCheckpointUpdate = 1,
    Killed = 2,
    IsLocked = 3,
    InvalidCoin = 4,
    AccessDenied = 5,
    ZeroFutureAdmin = 6,
    InvalidAdmin = 7,
    InvalidDecimals = 8,
    NotAdmin = 10,
    ZeroAddress = 11,
    SmartContractDepositorsNotAllowed = 12,
    NeedNonZeroValue = 13,
    NoExistingLockFound = 14,
    CannotAddToExpiredLockWithdraw = 15,
    WithdrawOldTokensFirst = 16,
    CanOnlyLockUntilTimeInTheFuture = 17,
    VotingLockCanBe4YearsMax = 18,
    LockExpired = 19,
    NothingIsLocked = 20,
    CanOnlyIncreaseLockDuration = 21,
    TheLockDidntExpire = 22,
    InvalidBlockNumber = 23,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
