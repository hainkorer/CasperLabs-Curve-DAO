use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    /// 65,536 for (ERC20 EXPIRED)
    UniswapV2CoreERC20EXPIRED = 0,
    /// 65,537 for (ERC20 Signature Verification Failed)
    UniswapV2CoreERC20SignatureVerificationFailed = 1,
    /// 65,538 for (ERC20 OverFlow1)
    UniswapV2CoreERC20OverFlow1 = 2,
    /// 65,539 for (ERC20 OverFlow2)
    UniswapV2CoreERC20OverFlow2 = 3,
    /// 65,540 for (ERC20 OverFlow3)
    UniswapV2CoreERC20OverFlow3 = 4,
    /// 65,541 for (ERC20 OverFlow4)
    UniswapV2CoreERC20OverFlow4 = 5,
    /// 65,542 for (ERC20 UnderFlow1)
    UniswapV2CoreERC20UnderFlow1 = 6,
    /// 65,543 for (ERC20 UnderFlow2)
    UniswapV2CoreERC20UnderFlow2 = 7,
    /// 65,544 for (ERC20 UnderFlow3)
    UniswapV2CoreERC20UnderFlow3 = 8,
    /// 65,545 for (ERC20 UnderFlow4)
    UniswapV2CoreERC20UnderFlow4 = 9,
    /// 65,546 for (ERC20 UnderFlow5)
    UniswapV2CoreERC20UnderFlow5 = 10,
    /// 65,546 for (ERC20 CRV Invalid Minter)
    Erc20CRVInvalidMinter = 10001,
    /// 65,546 for (ERC20 CRV Only Minter Allowed1)
    Erc20CRVOnlyMinterAllowed1 = 10002,
    /// 65,546 for (ERC20 CRV Only Minter Allowed2)
    Erc20CRVOnlyMinterAllowed2 = 10003,
    /// 65,546 for (ERC20 CRV Admin Only)
    Erc20CRVAdminOnly = 10004,
    /// 65,546 for (ERC20 CRV Too Soon)
    Erc20CRVTooSoon = 10005,
    /// 65,546 for (ERC20 CRV Zero Address)
    Erc20CRVZeroAddress = 10006,
    /// 65,546 for (ERC20 CRV Minter Only)
    Erc20CRVMinterOnly = 10007,
    /// 65,546 for (ERC20 CRV Exceeds Allowable Mint)
    Erc20CRVExceedsAllowableMint = 10008,
    /// 65,546 for (ERC20 CRV Start Greater Than End)
    Erc20CRVStartGreaterThanEnd = 10009,
    /// 65,546 for (ERC20 CRV Too Far In Future)
    Erc20CRVTooFarInFuture = 10010,
    /// 65,546 for (ERC20 CRV Curr Rate Less Than Init Rate)
    Erc20CRVCurrRateLessThanInitRate = 10011,
    /// 65,546 for (Liquidity Gauge Reward Unauthorized)
    LiquidityGaugeRewardUnauthorized = 10101,
    /// 65,546 for (Liquidity Gauge Reward Kick Not Allowed1)
    LiquidityGaugeRewardKickNotAllowed1 = 10102,
    /// 65,546 for (Liquidity Gauge Reward  Kick Not Needed1))
    LiquidityGaugeRewardKickNotNeeded1 = 10103,
    /// 65,546 for (Liquidity Gauge Reward Kick Not Needed2)
    LiquidityGaugeRewardKickNotNeeded2 = 10104,
    /// 65,546 for (Liquidity Gauge Reward Not Approved)
    LiquidityGaugeRewardNotApproved = 10105,
    /// 65,546 for (Liquidity Gauge Reward Is Locked1)
    LiquidityGaugeRewardIsLocked1 = 10106,
    /// 65,546 for (Liquidity Gauge Reward Is Locked2)
    LiquidityGaugeRewardIsLocked2 = 10107,
    /// 65,546 for (Liquidity Gauge Reward Is Locked3)
    LiquidityGaugeRewardIsLocked3 = 10108,
    /// 65,546 for (Liquidity Gauge Reward Admin Only1)
    LiquidityGaugeRewardAdminOnly1 = 10109,
    /// 65,546 for (Liquidity Gauge Reward Admin Only2)
    LiquidityGaugeRewardAdminOnly2 = 10110,
    /// 65,546 for (Liquidity Gauge Reward Admin Only3)
    LiquidityGaugeRewardAdminOnly3 = 10111,
    /// 65,546 for (Liquidity Gauge Reward Admin Only4)
    LiquidityGaugeRewardAdminOnly4 = 10112,
    /// 65,546 for (Liquidity Gauge Reward Admin Not Set)
    LiquidityGaugeRewardAdminNotSet = 10113,
    /// 65,546 for (Liquidity Gauge Reward Zero Address1)
    LiquidityGaugeRewardZeroAddress1 = 10114,
    /// 65,546 for (Liquidity Gauge Reward Zero Address2)
    LiquidityGaugeRewardZeroAddress2 = 10115,
    /// 65,546 for (Liquidity Gauge Reward Zero Address3)
    LiquidityGaugeRewardZeroAddress3 = 10116,
    /// 65,546 for (Reward Wrapper Unauthorized)
    RewardWrapperUnauthorized = 10201,
    /// 65,546 for (Reward Wrapper Not Approved)
    RewardWrapperNotApproved = 10202,
    /// 65,546 for (Reward Wrapper IsLocked1)
    RewardWrapperIsLocked1 = 10203,
    /// 65,546 for (Reward Wrapper IsLocked2)
    RewardWrapperIsLocked2 = 10204,
    /// 65,546 for (Reward Wrapper Is Locked3)
    RewardWrapperIsLocked3 = 10205,
    /// 65,546 for (Reward Wrapper Admin Only1)
    RewardWrapperAdminOnly1 = 10206,
    /// 65,546 for (Reward Wrapper Admin Only2)
    RewardWrapperAdminOnly2 = 10207,
    /// 65,546 for (Reward Wrapper Admin Only3)
    RewardWrapperAdminOnly3 = 10208,
    /// 65,546 for (Reward Wrapper Admin Not Set)
    RewardWrapperAdminNotSet = 10209,
    /// 65,546 for (Reward Wrapper Zero Address)
    RewardWrapperZeroAddress = 10210,
    /// 65,546 for (Reward Wrapper IsKilled1)
    RewardWrapperIsKilled1 = 10211,
    /// 65,546 for (Reward Wrapper Is Killed2)
    RewardWrapperIsKilled2 = 10212,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV1InvalidMinter = 10301,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV1OnlyMinterAllowed1 = 10302,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV1OnlyMinterAllowed2 = 10303,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV1OnlyMinterAllowed3 = 10304,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV1ZeroAddressNotAllowed = 10305,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV2InvalidMinter = 10406,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV2OnlyMinterAllowed1 = 10407,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV2OnlyMinterAllowed2 = 10408,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV2ZeroAddressNotAllowed = 10409,
    /// 65,538 for (Curve Token V1 Invalid Minter)
    CurveTokenV3InvalidMinter = 10501,
    /// 65,538 for (Curve Token V3 Only Minter Allowed1)
    CurveTokenV3OnlyMinterAllowed1 = 10502,
    /// 65,538 for (Curve Token V3 Only Minter Allowed2)
    CurveTokenV3OnlyMinterAllowed2 = 10503,
    /// 65,538 for (Curve Token V3 Only Minter Allowed3)
    CurveTokenV3OnlyMinterAllowed3 = 10504,
    /// 65,538 for (Fee Distributor Invalid Token Checkpoint Update)
    FeeDistributorInvalidTokenCheckpointUpdate = 10601,
    /// 65,538 for (Fee Distributor Killed1)
    FeeDistributorKilled1 = 10602,
    /// 65,538 for (Fee Distributor Killed2)
    FeeDistributorKilled2 = 10603,
    /// 65,538 for (Fee Distributor Killed3)
    FeeDistributorKilled3 = 10604,
    /// 65,538 for (Fee Distributor Is Locked1)
    FeeDistributorIsLocked1 = 10605,
    /// 65,538 for (Fee Distributor Is Locked2)
    FeeDistributorIsLocked2 = 10606,
    /// 65,538 for (Fee Distributor Invalid Coin1)
    FeeDistributorInvalidCoin1 = 10607,
    /// 65,538 for (Fee Distributor Invalid Coin2)
    FeeDistributorInvalidCoin2 = 10608,
    /// 65,538 for (Fee Distributor Access Denied)
    FeeDistributorAccessDenied = 10609,
    /// 65,538 for (Fee Distributor Zero Future Admin)
    FeeDistributorZeroFutureAdmin = 10610,
    /// 65,538 for (Fee Distributor Invalid Admin1)
    FeeDistributorInvalidAdmin1 = 10611,
    /// 65,538 for (Fee Distributor Invalid Admin2)
    FeeDistributorInvalidAdmin2 = 10612,
    /// 65,538 for (Fee Distributor Invalid Admin3)
    FeeDistributorInvalidAdmin3 = 10613,
    /// 65,538 for (Fee Distributor Invalid Admin4)
    FeeDistributorInvalidAdmin4 = 10614,
    /// 65,538 for (Gauge Controller Address Zero1)
    GaugeControllerAddressZero1 = 10701,
    /// 76,237 for (Gauge Controller Address Zero2)
    GaugeControllerAddressZero2 = 10702,
    /// 76,238 for (Gauge Controller Only Admin1)
    GaugeControllerOnlyAdmin1 = 10703,
    /// 76,239 for (Gauge Controller Only Admin2)
    GaugeControllerOnlyAdmin2 = 10704,
    /// 76,241 for (Gauge Controller Admin Not Set)
    GaugeControllerAdminNotSet = 10705,
    /// 76,242 for (Gauge Controller Gauge Type Is Zero)
    GaugeControllerGaugeTypeIsZero = 10706,
    /// 76,243 for (Gauge Controller Not Admin1)
    GaugeControllerNotAdmin1 = 10707,
    /// 76,244 for (Gauge Controller Not Admin2)
    GaugeControllerNotAdmin2 = 10708,
    /// 65,545 for (Gauge Controller Not Admin3)
    GaugeControllerNotAdmin3 = 10709,
    /// 76,246 for (Gauge Controller Not Admin3)
    GaugeControllerNotAdmin4 = 10710,
    /// 76,247 for (Gauge Controller cannot add same gauge twice)
    GaugeControllerCannotAddSameGaugeTwice = 10711,
    /// 76,248 for (Gauge Controller gauge type is greater than equal to zero and less than n_gauge_types)
    GaugeControllerGaugeType1 = 10712,
    /// 76,249 for (Gauge Controller Your token lock expires too soon)
    GaugeControllerTokenLockExpiresTooSoon = 10713,
    /// 76,250 for (Gauge Controller You used all your voting power)
    GaugeControllerUsedAllYourVotingPower = 10714,
    /// 76,251 for (Gauge Controller You Cannot vote so often)
    GaugeControllerCannotVoteSoOften = 10715,
    /// 76,252 for (Gauge Controller Gauge not added)
    GaugeControllerGaugeNotAdded = 10716,
    /// 76,253 for (Gauge Controller Used too much power)
    GaugeControllerUsedTooMuchPower = 10717,
    /// 76,254 for (Gauge Controller OverFlow1)
    GaugeControllerOverFlow1 = 10718,
    /// 76,255 for (Gauge Controller OverFlow2)
    GaugeControllerOverFlow2 = 10719,
    /// 76,256 for (Gauge Controller OverFlow3)
    GaugeControllerOverFlow3 = 10720,
    /// 76,257 for (Gauge Controller OverFlow4)
    GaugeControllerOverFlow4 = 10721,
    /// 76,258 for (Gauge Controller OverFlow5)
    GaugeControllerOverFlow5 = 10722,
    /// 76,259 for (Gauge Controller OverFlow6)
    GaugeControllerOverFlow6 = 10723,
    /// 76,260 for (Gauge Controller OverFlow7)
    GaugeControllerOverFlow7 = 10724,
    /// 76,261 for (Gauge Controller OverFlow8)
    GaugeControllerOverFlow8 = 10725,
    /// 76,262 for (Gauge Controller OverFlow9)
    GaugeControllerOverFlow9 = 10726,
    /// 76,263 for (Gauge Controller OverFlow10)
    GaugeControllerOverFlow10 = 10727,
    /// 76,264 for (Gauge Controller OverFlow11)
    GaugeControllerOverFlow11 = 10728,
    /// 76,265 for (Gauge Controller OverFlow12)
    GaugeControllerOverFlow12 = 10729,
    /// 76,266 for (Gauge Controller OverFlow13)
    GaugeControllerOverFlow13 = 10730,
    /// 76,267 for (Gauge Controller OverFlow14)
    GaugeControllerOverFlow14 = 10731,
    /// 76,268 for (Gauge Controller OverFlow15)
    GaugeControllerOverFlow15 = 10732,
    /// 76,269 for (Gauge Controller OverFlow16)
    GaugeControllerOverFlow16 = 10733,
    /// 76,270 for (Gauge Controller OverFlow17)
    GaugeControllerOverFlow17 = 10734,
    /// 76,271 for (Gauge Controller OverFlow18)
    GaugeControllerOverFlow18 = 10735,
    /// 76,272 for (Gauge Controller OverFlow19)
    GaugeControllerOverFlow19 = 10736,
    /// 76,273 for (Gauge Controller OverFlow20)
    GaugeControllerOverFlow20 = 10737,
    /// 76,274 for (Gauge Controller OverFlow21)
    GaugeControllerOverFlow21 = 10738,
    /// 76,275 for (Gauge Controller OverFlow22)
    GaugeControllerOverFlow22 = 10739,
    /// 76,276 for (Gauge Controller OverFlow23)
    GaugeControllerOverFlow23 = 10740,
    /// 76,278 for (Gauge Controller OverFlow24)
    GaugeControllerOverFlow24 = 10741,
    /// 76,279 for (Gauge Controller OverFlow25)
    GaugeControllerOverFlow25 = 10742,
    /// 76,280 for (Gauge Controller OverFlow26)
    GaugeControllerOverFlow26 = 10743,
    /// 76,281 for (Gauge Controller OverFlow27)
    GaugeControllerOverFlow27 = 10744,
    /// 76,282 for (Gauge Controller UnderFlow1)
    GaugeControllerUnderFlow1 = 10745,
    /// 76,283 for (Gauge Controller UnderFlow2)
    GaugeControllerUnderFlow2 = 10746,
    /// 76,284 for (Gauge Controller UnderFlow3)
    GaugeControllerUnderFlow3 = 10747,
    /// 76,285 for (Gauge Controller UnderFlow4)
    GaugeControllerUnderFlow4 = 10748,
    /// 76,286 for (Gauge Controller UnderFlow5)
    GaugeControllerUnderFlow5 = 10749,
    /// 76,287 for (Gauge Controller UnderFlow6)
    GaugeControllerUnderFlow6 = 10750,
    /// 76,288 for (Gauge Controller UnderFlow7)
    GaugeControllerUnderFlow7 = 10751,
    /// 76,289 for (Gauge Controller UnderFlow8)
    GaugeControllerUnderFlow8 = 10752,
    /// 76,290 for (Gauge Controller UnderFlow9)
    GaugeControllerUnderFlow9 = 10753,
    /// 76,291 for (Gauge Controller UnderFlow10)
    GaugeControllerUnderFlow10 = 10754,
    /// 76,292 for (Gauge Controller UnderFlow11)
    GaugeControllerUnderFlow11 = 10755,
    /// 76,293 for (Gauge Controller UnderFlow12)
    GaugeControllerUnderFlow12 = 10756,
    /// 76,294 for (Gauge Controller UnderFlow13)
    GaugeControllerUnderFlow13 = 10757,
    /// 76,295 for (Gauge Controller UnderFlow14)
    GaugeControllerUnderFlow14 = 10758,
    /// 76,296 for (Gauge Controller UnderFlow15)
    GaugeControllerUnderFlow15 = 10759,
    /// 76,297 for (Gauge Controller UnderFlow16)
    GaugeControllerUnderFlow16 = 10760,
    /// 76,298 for (Gauge Controller UnderFlow17)
    GaugeControllerUnderFlow17 = 10761,
    /// 76,299 for (Gauge Controller UnderFlow18)
    GaugeControllerUnderFlow18 = 10762,
    /// 76,300 for (Gauge Controller UnderFlow19)
    GaugeControllerUnderFlow19 = 10763,
    /// 76,301 for (Gauge Controller UnderFlow20)
    GaugeControllerUnderFlow20 = 10764,
    /// 76,302 for (Gauge Controller UnderFlow21)
    GaugeControllerUnderFlow21 = 10765,
    /// 76,303 for (Gauge Controller UnderFlow22)
    GaugeControllerUnderFlow22 = 10766,
    /// 76,304 for (Gauge Controller UnderFlow23)
    GaugeControllerUnderFlow23 = 10767,
    /// 65,536 for (Minter Gauge Is Not Added)
    MinterIsNotAdded = 10801,
    /// 65,537 for (Minter Gauge Locked)
    MinterLocked1 = 10802,
    /// 65,537 for (Minter Gauge Locked)
    MinterLocked2 = 10803,
    /// 65,537 for (Minter Gauge Locked)
    MinterLocked3 = 10804,
    /// 65,538 for (Reward Only Gauge OverFlow1)
    RewardOnlyGaugeOverFlow1 = 10901,
    /// 65,539 for (Reward Only Gauge OverFlow2)
    RewardOnlyGaugeOverFlow2 = 10902,
    /// 65,540 for (Reward Only Gauge OverFlow3)
    RewardOnlyGaugeOverFlow3 = 10903,
    /// 65,541 for (Reward Only Gauge OverFlow4)
    RewardOnlyGaugeOverFlow4 = 10904,
    /// 65,541 for (Reward Only Gauge OverFlow5)
    RewardOnlyGaugeOverFlow5 = 10905,
    /// 65,541 for (Reward Only Gauge OverFlow6)
    RewardOnlyGaugeOverFlow6 = 10906,
    /// 65,541 for (Reward Only Gauge OverFlow7)
    RewardOnlyGaugeOverFlow7 = 10907,
    /// 65,542 for (Reward Only Gauge UnderFlow1)
    RewardOnlyGaugeUnderFlow1 = 10908,
    /// 65,543 for (Reward Only Gauge UnderFlow2)
    RewardOnlyGaugeUnderFlow2 = 10909,
    /// 65,544 for (Reward Only Gauge UnderFlow3)
    RewardOnlyGaugeUnderFlow3 = 10910,
    /// 65,545 for (Reward Only Gauge UnderFlow4)
    RewardOnlyGaugeUnderFlow4 = 10911,
    /// 65,546 for (Reward Only Gauge UnderFlow5)
    RewardOnlyGaugeUnderFlow5 = 10912,
    /// 65,546 for (Reward Only Gauge UnderFlow6)
    RewardOnlyGaugeUnderFlow6 = 10913,
    /// 65,546 for (Reward Only Gauge UnderFlow7)
    RewardOnlyGaugeUnderFlow7 = 10914,
    /// 65,546 for (Reward Only Gauge UnderFlow8)
    RewardOnlyGaugeUnderFlow8 = 10915,
    /// 65,546 for (Reward Only Gauge UnderFlow9)
    RewardOnlyGaugeUnderFlow9 = 10916,
    /// 65,540 for (Reward Only Gauge Only Admin1)
    RewardOnlyGaugeOnlyAdmin1 = 10917,
    /// 65,540 for (Reward Only Gauge Only Admin2)
    RewardOnlyGaugeOnlyAdmin2 = 10918,
    /// 65,540 for (Reward Only Gauge Only Future Admin)
    RewardOnlyGaugeOnlyFutureAdmin = 10919,
    /// 65,540 for (Reward Only Gauge Cannot Redirect When Claiming For Another User)
    RewardOnlyGaugeCannotRedirectWhenClaimingForAnotherUser = 10920,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero1 = 10921,
    /// 65,540 for (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero2 = 10922,
    /// 65,540 for (Reward Only Gauge Reward Token Is Zero)
    RewardOnlyGaugeRewardTokenIsZeroAddress = 10923,
    /// 65,540 for (Reward Only Gauge Cannot Modify Existing Reward Token)
    RewardOnlyGaugeCannotModifyExistingRewardToken = 10924,
    /// 65,540 for (Reward Only Gauge Receiver Is Zero Address)
    RewardOnlyGaugeLocked1 = 10925,
    /// 65,538 for (Vesting Escrow OverFlow1)
    VestingEscrowOverFlow1 = 11001,
    /// 65,539 for (Vesting Escrow OverFlow2)
    VestingEscrowOverFlow2 = 11002,
    /// 65,540 for (Vesting Escrow OverFlow3)
    VestingEscrowOverFlow3 = 11003,
    /// 65,541 for (Vesting Escrow OverFlow4)
    VestingEscrowOverFlow4 = 11004,
    /// 65,541 for (Vesting Escrow OverFlow5)
    VestingEscrowOverFlow5 = 11005,
    /// 65,542 for (Vesting Escrow UnderFlow1)
    VestingEscrowUnderFlow1 = 11006,
    /// 65,543 for (Vesting Escrow UnderFlow2)
    VestingEscrowUnderFlow2 = 11007,
    /// 65,544 for (Vesting Escrow UnderFlow3)
    VestingEscrowUnderFlow3 = 11008,
    /// 65,545 for (Vesting Escrow UnderFlow4)
    VestingEscrowUnderFlow4 = 11009,
    /// 65,546 for (Vesting Escrow UnderFlow5)
    VestingEscrowUnderFlow5 = 11010,
    /// 65,546 for (Vesting Escrow UnderFlow6)
    VestingEscrowUnderFlow6 = 11011,
    /// 65,546 for (Vesting Escrow UnderFlow7)
    VestingEscrowUnderFlow7 = 11012,
    /// 65,546 for (Vesting Escrow UnderFlow8)
    VestingEscrowUnderFlow8 = 11013,
    /// 65,546 for (Vesting Escrow UnderFlow9)
    VestingEscrowUnderFlow9 = 11014,
    /// 65,546 for (Vesting Escrow UnderFlow10)
    VestingEscrowUnderFlow10 = 11015,
    /// 65,546 for (Vesting Escrow UnderFlow11)
    VestingEscrowUnderFlow11 = 11016,
    /// 65,546 for (Vesting Escrow UnderFlow12)
    VestingEscrowUnderFlow12 = 11017,
    /// 65,546 for (Vesting Escrow UnderFlow13)
    VestingEscrowUnderFlow13 = 11018,
    /// 65,546 for (Vesting Escrow Cannot Disable)
    VestingEscrowCannotDisable = 11019,
    /// 65,540 for (Vesting Escrow Only Admin1)
    VestingEscrowOnlyAdmin1 = 11020,
    /// 65,540 for (Vesting Escrow Only Admin2)
    VestingEscrowOnlyAdmin2 = 11021,
    /// 65,540 for (Vesting Escrow Only Admin3)
    VestingEscrowOnlyAdmin3 = 11022,
    /// 65,540 for (Vesting Escrow Only Admin4)
    VestingEscrowOnlyAdmin4 = 11023,
    /// 65,540 for (Vesting Escrow Only Admin5)
    VestingEscrowOnlyAdmin5 = 11024,
    /// 65,540 for (Vesting Escrow Only Admin6)
    VestingEscrowOnlyAdmin6 = 11025,
    /// 65,540 for (Vesting Escrow Only Admin7)
    VestingEscrowOnlyAdmin7 = 11026,
    /// 65,540 for (Vesting Escrow Admin Not Set)
    VestingEscrowAdminNotSet = 11027,
    /// 65,540 for (Vesting Escrow Locked)
    VestingEscrowLocked1 = 11028,
    /// 65,540 for (Vesting Escrow Locked)
    VestingEscrowLocked2 = 11029,
    /// 65,540 for (Vesting Escrow Fund Admin Disabled)
    VestingEscrowFundAdminsDisabled = 11030,
    /// 65,538 for (Vesting Escrow Factory OverFlow1)
    VestingEscrowFactoryOverFlow1 = 11101,
    /// 65,539 for (Vesting Escrow Factory OverFlow2)
    VestingEscrowFactoryOverFlow2 = 11102,
    /// 65,540 for (Vesting Escrow Factory Only Admin1)
    VestingEscrowFactoryOnlyAdmin1 = 11103,
    /// 65,540 for (Vesting Escrow Factory Only Admin2)
    VestingEscrowFactoryOnlyAdmin2 = 11104,
    /// 65,540 for (Vesting Escrow Factory Only Admin3)
    VestingEscrowFactoryOnlyAdmin3 = 11105,
    /// 65,540 for (Vesting Escrow Factory Duration Too Short)
    VestingEscrowFactoryDurationTooShort = 11106,
    /// 65,540 for (Vesting Escrow Factory Start Time Too Soon)
    VestingEscrowFactoryStartTimeTooSoon = 11107,
    /// 65,540 for (Vesting Escrow Factory Admin Not Set)
    VestingEscrowFactoryAdminNotSet = 11108,
    /// 65,540 for (Vesting Escrow Simple Initialize Once)
    VestingEscrowSimpleOnlyInitializeOnce = 11201,
    /// 65,540 for (Vesting Escrow Simple Admin Only1)
    VestingEscrowSimpleAdminOnly1 = 11202,
    /// 65,540 for (Vesting Escrow Simple Admin Only2)
    VestingEscrowSimpleAdminOnly2 = 11203,
    /// 65,540 for (Vesting Escrow Simple Admin Only3)
    VestingEscrowSimpleAdminOnly3 = 11204,
    /// 65,540 for (Vesting Escrow Simple Admin Only4)
    VestingEscrowSimpleAdminOnly4 = 11205,
    /// 65,540 for (Vesting Escrow Simple Cannot Disable)
    VestingEscrowSimpleCannotDisable = 11206,
    /// 65,540 for (Vesting Escrow Simple Admin Not Set)
    VestingEscrowSimpleAdminNotSet = 11207,
    /// 65,540 for (Vesting Escrow Simple Is Locked)
    VestingEscrowSimpleIsLocked = 11208,
    /// 65,540 for (Voting Escrow Invalid Decimals)
    VotingEscrowInvalidDecimals = 11301,
    /// 65,540 for (Voting Escrow Admin Only)
    VotingEscrowAdminOnly = 11302,
    /// 65,540 for (Voting Escrow Zero Address)
    VotingEscrowZeroAddress = 11303,
    /// 65,540 for (Voting Escrow Is Locked1)
    VotingEscrowIsLocked1 = 11304,
    /// 65,540 for (Voting Escrow Is Locked2)
    VotingEscrowIsLocked2 = 11305,
    /// 65,540 for (Voting Escrow Is Locked3)
    VotingEscrowIsLocked3 = 11306,
    /// 65,540 for (Voting Escrow Is Locked4)
    VotingEscrowIsLocked4 = 11307,
    /// 65,540 for (Voting Escrow Need Non Zero Value1)
    VotingEscrowNeedNonZeroValue1 = 11308,
    /// 65,540 for (Voting Escrow Need Non Zero Value2)
    VotingEscrowNeedNonZeroValue2 = 11309,
    /// 65,540 for (Voting Escrow Need Non Zero Value3)
    VotingEscrowNeedNonZeroValue3 = 11310,
    /// 65,540 for (Voting Escrow No Existing Lock Found1)
    VotingEscrowNoExistingLockFound1 = 11311,
    /// 65,540 for (Voting Escrow No Existing Lock Found2)
    VotingEscrowNoExistingLockFound2 = 11312,
    /// 65,540 for (Voting Escrow Cannot Add To Expired Lock Withdraw1)
    VotingEscrowCannotAddToExpiredLockWithdraw1 = 11313,
    /// 65,540 for (Voting Escrow Cannot Add To Expired Lock Withdraw2)
    VotingEscrowCannotAddToExpiredLockWithdraw2 = 11314,
    /// 65,540 for (Voting Escrow Withdraw Old Tokens First)
    VotingEscrowWithdrawOldTokensFirst = 11315,
    /// 65,540 for (Voting Escrow Can Only Lock Until Time In The Future)
    VotingEscrowCanOnlyLockUntilTimeInTheFuture = 11316,
    /// 65,540 for (Voting Escrow Voting Lock Can Be 4 Years Max1)
    VotingEscrowVotingLockCanBe4YearsMax1 = 11317,
    /// 65,540 for (Voting Escrow Voting Lock Can Be 4 Years Max2)
    VotingEscrowVotingLockCanBe4YearsMax2 = 11318,
    /// 65,540 for (Voting Escrow Lock Expired)
    VotingEscrowLockExpired = 11319,
    /// 65,540 for (Voting Escrow Is Locked)
    VotingEscrowNothingIsLocked = 11320,
    /// 65,540 for (Voting Escrow Can Only Increase Lock Duration)
    VotingEscrowCanOnlyIncreaseLockDuration = 11321,
    /// 65,540 for (Voting Escrow The Lock Didnt Expire)
    VotingEscrowTheLockDidntExpire = 11322,
    /// 65,540 for (Voting Escrow Invalid Block Number)
    VotingEscrowInvalidBlockNumber = 11323,
    Abort = 150,
    SmartContractDepositorsNotAllowed = 151,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
