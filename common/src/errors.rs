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
    AdminOnly = 10,
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
    Unauthorized = 24,
    KickNotAllowed = 25,
    KickNotNeeded = 26,
    NotApproved = 27,
    AdminNotSet = 28,
    InvalidMinter = 29,
    OnlyMinterAllowed = 30,
    // AdminOnly = 31,
    TooSoon = 32,
    // ZeroAddress = 33,
    MinterOnly = 34,
    ExceedsAllowableMint = 35,
    StartGreaterThanEnd = 36,
    TooFarInFuture = 37,
    CurrRateLessThanInitRate = 39,
    RewardWrapperUnauthorized = 40,
    RewardWrapperNotApproved = 41,
    RewardWrapperIsLocked = 42,
    RewardWrapperAdminOnly = 43,
    RewardWrapperAdminNotSet = 44,
    RewardWrapperZeroAddress = 45,
    RewardWrapperIsKilled = 46,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
