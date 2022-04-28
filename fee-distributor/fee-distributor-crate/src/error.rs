use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    InvalidTokenCheckpointUpdate = 0,
    Killed,
    IsLocked,
    InvalidCoin,
    AccessDenied,
    ZeroFutureAdmin,
    InvalidAdmin,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
