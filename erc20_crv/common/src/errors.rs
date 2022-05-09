use casper_types::ApiError;
#[repr(u16)]
pub enum Error {
    InvalidMinter = 0,
    OnlyMinterAllowed = 1,
    AdminOnly = 2,
    TooSoon = 3,
    ZeroAddress = 4,
    MinterOnly = 5,
    ExceedsAllowableMint = 6,
    StartGreaterThanEnd = 7,
    TooFarInFuture = 8,
    CurrRateLessThanInitRate = 9,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
