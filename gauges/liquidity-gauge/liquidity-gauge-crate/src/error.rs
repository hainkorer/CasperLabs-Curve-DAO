use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    /// 65,538 for (Reward Only Gauge OverFlow1)
    LiquidityGaugeV3OverFlow1 = 0,
    /// 65,539 for (Reward Only Gauge OverFlow2)
    LiquidityGaugeV3OverFlow2 = 1,
    /// 65,540 for (Reward Only Gauge OverFlow3)
    LiquidityGaugeV3OverFlow3 = 2,
    /// 65,541 for (Reward Only Gauge OverFlow4)
    LiquidityGaugeV3OverFlow4 = 3,
    /// 65,541 for (Reward Only Gauge OverFlow5)
    LiquidityGaugeV3OverFlow5 = 4,
    /// 65,541 for (Reward Only Gauge OverFlow6)
    LiquidityGaugeV3OverFlow6 = 5,
    /// 65,541 for (Reward Only Gauge OverFlow7)
    LiquidityGaugeV3OverFlow7 = 6,
    /// 65,542 for (Reward Only Gauge UnderFlow1)
    LiquidityGaugeUnderFlow1 = 7,
    /// 65,543 for (Reward Only Gauge UnderFlow2)
    LiquidityGaugeUnderFlow2 = 8,
    /// 65,544 for (Reward Only Gauge UnderFlow3)
    LiquidityGaugeUnderFlow3 = 9,
    /// 65,545 for (Reward Only Gauge UnderFlow4)
    LiquidityGaugeUnderFlow4 = 10,
    /// 65,546 for (Reward Only Gauge UnderFlow5)
    LiquidityGaugeUnderFlow5 = 12,
    /// 65,546 for (Reward Only Gauge UnderFlow6)
    LiquidityGaugeUnderFlow6 = 13,
    /// 65,546 for (Reward Only Gauge UnderFlow7)
    LiquidityGaugeUnderFlow7 = 14,
    /// 65,546 for (Reward Only Gauge UnderFlow8)
    LiquidityGaugeUnderFlow8 = 15,
    /// 65,546 for (Reward Only Gauge UnderFlow9)
    LiquidityGaugeUnderFlow9 = 16,
    /// 65,540 for (Reward Only Gauge Only Admin1)
    LiquidityGaugeOnlyAdmin1 = 17,
    /// 65,540 for (Reward Only Gauge Only Admin2)
    LiquidityGaugeOnlyAdmin2 = 18,
    /// 65,540 for (Reward Only Gauge Only Future Admin)
    LiquidityGaugeOnlyFutureAdmin = 19,
    /// 65,540 for (Reward Only Gauge Cannot Redirect When Claiming For Another User)
    LiquidityGaugeCannotRedirectWhenClaimingForAnotherUser = 20,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    LiquidityGaugeValueIsZero1 = 21,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    LiquidityGaugeValueIsZero2 = 22,
    /// 65,540 for (Reward Only Gauge Reward Token Is Zero)
    LiquidityGaugeRewardTokenIsZeroAddress = 23,
    /// 65,540 for (Reward Only Gauge Cannot Modify Existing Reward Token)
    LiquidityGaugeCannotModifyExistingRewardToken = 24,
    /// 65,540 for (Reward Only Gauge Receiver Is Zero Address)
    LiquidityGaugeLocked1 = 25,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
