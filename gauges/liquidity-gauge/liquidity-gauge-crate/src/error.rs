use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    Locked = 0,
    NotZeroAddress,
    InvalidCaller,
    KickNotAllowed,
    KickNotNeeded,
    NotApproved,
    AdminOnly,
    AdminNotSet,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
