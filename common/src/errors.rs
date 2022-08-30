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
    ///ERC20 ERRORS
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
    /// 65,546 for (ERC20 CRV Over flow1)
    Erc20CRVOverFlow1 = 10012,
    /// 65,546 for (ERC20 CRV Over flow2)
    Erc20CRVOverFlow2 = 10013,
    /// 65,546 for (ERC20 CRV Airthmetic Error1)
    Erc20CRVAirthmeticError1 = 10014,
    /// 65,546 for (ERC20 CRV Over flow3)
    Erc20CRVOverFlow3 = 10015,
    /// 65,546 for (ERC20 CRV Over flow4)
    Erc20CRVOverFlow4 = 10016,
    /// 65,546 for (ERC20 CRV Over flow5)
    Erc20CRVOverFlow5 = 10017,
    /// 65,546 for (ERC20 CRV Over flow6)
    Erc20CRVOverFlow6 = 10018,
    /// 65,546 for (ERC20 CRV Airthmetic Error2)
    Erc20CRVAirthmeticError2 = 10019,
    /// 65,546 for (ERC20 CRV Over flow7)
    Erc20CRVOverFlow7 = 10020,
    /// 65,546 for (ERC20 CRV Under flow1)
    Erc20CRVUnderFlow1 = 10021,
    /// 65,546 for (ERC20 CRV Airthmetic Error3)
    Erc20CRVAirthmeticError3 = 10022,
    /// 65,546 for (ERC20 CRV Over flow8)
    Erc20CRVOverFlow8 = 10023,
    /// 65,546 for (ERC20 CRV Under flow3)
    Erc20CRVUnderFlow3 = 10024,
    /// 65,546 for (ERC20 CRV Over flow9)
    Erc20CRVOverFlow9 = 10025,
    /// 65,546 for (ERC20 CRV Over flow10)
    Erc20CRVOverFlow10 = 10026,
    /// 65,546 for (ERC20 CRV Airthmetic Error4)
    Erc20CRVAirthmeticError4 = 10027,
    /// 65,546 for (ERC20 CRV Airthmetic Error5)
    Erc20CRVAirthmeticError5 = 10028,
    /// 65,546 for (ERC20 CRV Over flow11)
    Erc20CRVOverFlow11 = 10029,
    /// 65,546 for (ERC20 CRV Over flow12)
    Erc20CRVOverFlow12 = 10030,
    /// 65,546 for (ERC20 CRV Over flow13)
    Erc20CRVOverFlow13 = 10031,
    /// 65,546 for (ERC20 CRV Over flow14)
    Erc20CRVOverFlow14 = 10032,
    /// 65,546 for (ERC20 CRV Over flow15)
    Erc20CRVOverFlow15 = 10033,
    /// 65,546 for (ERC20 CRV Under flow4)
    Erc20CRVUnderFlow4 = 10034,
    /// 65,546 for (ERC20 CRV Over flow16)
    Erc20CRVOverFlow16 = 10035,
    /// 65,546 for (ERC20 CRV Over flow17)
    Erc20CRVOverFlow17 = 10036,
    /// 65,546 for (ERC20 CRV Over flow18)
    Erc20CRVOverFlow18 = 10037,
    /// 65,546 for (ERC20 CRV Over flow19)
    Erc20CRVOverFlow19 = 10038,
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
    /// 65,546 for (Reward Wrapper Division Error 1)
    RewardWrapperDivisionError1 = 10213,
    /// 65,546 for (Reward Wrapper Division Error 2)
    RewardWrapperDivisionError2 = 10214,
    /// 65,546 for (Reward Wrapper Division Error 3)
    RewardWrapperDivisionError3 = 10215,
    /// 65,546 for (Reward Wrapper Division Error 4)
    RewardWrapperDivisionError4 = 10216,
    /// 65,546 for (Reward Wrapper Division Error 5)
    RewardWrapperDivisionError5 = 10217,
    /// 65,546 for (Reward Wrapper Division Error 6)
    RewardWrapperDivisionError6 = 10218,
    /// 65,546 for (Reward Wrapper Division Error 7)
    RewardWrapperDivisionError7 = 10219,
    /// 65,546 for (Reward Wrapper Division Error 8)
    RewardWrapperDivisionError8 = 10220,
    /// 65,546 for (Reward Wrapper Addition Error 1)
    RewardWrapperAdditionError1 = 10221,
    /// 65,546 for (Reward Wrapper Addition Error 2)
    RewardWrapperAdditionError2 = 10222,
    /// 65,546 for (Reward Wrapper Addition Error 3)
    RewardWrapperAdditionError3 = 10223,
    /// 65,546 for (Reward Wrapper Addition Error 4)
    RewardWrapperAdditionError4 = 10224,
    /// 65,546 for (Reward Wrapper Addition Error 5)
    RewardWrapperAdditionError5 = 10225,
    /// 65,546 for (Reward Wrapper Addition Error 6)
    RewardWrapperAdditionError6 = 10226,
    /// 65,546 for (Reward Wrapper Addition Error 7)
    RewardWrapperAdditionError7 = 10227,
    /// 65,546 for (Reward Wrapper Addition Error 8)
    RewardWrapperAdditionError8 = 10228,
    /// 65,546 for (Reward Wrapper Addition Error 9)
    RewardWrapperAdditionError9 = 10229,
    /// 65,546 for (Reward Wrapper Addition Error 10)
    RewardWrapperAdditionError10 = 10230,
    /// 65,546 for (Reward Wrapper Addition Error 11)
    RewardWrapperAdditionError11 = 10231,
    /// 65,546 for (Reward Wrapper Addition Error 12)
    RewardWrapperAdditionError12 = 10232,
    /// 65,546 for (Reward Wrapper Subtraction Error 1)
    RewardWrapperSubtractionError1 = 10233,
    /// 65,546 for (Reward Wrapper Subtraction Error 2)
    RewardWrapperSubtractionError2 = 10234,
    /// 65,546 for (Reward Wrapper Subtraction Error 3)
    RewardWrapperSubtractionError3 = 10235,
    /// 65,546 for (Reward Wrapper Subtraction Error 4)
    RewardWrapperSubtractionError4 = 10236,
    /// 65,546 for (Reward Wrapper Subtraction Error 5)
    RewardWrapperSubtractionError5 = 10237,
    /// 65,546 for (Reward Wrapper Subtraction Error 6)
    RewardWrapperSubtractionError6 = 10238,
    /// 65,546 for (Reward Wrapper Subtraction Error 7)
    RewardWrapperSubtractionError7 = 10239,
    /// 65,546 for (Reward Wrapper Subtraction Error 8)
    RewardWrapperSubtractionError8 = 10240,
    /// 65,546 for (Reward Wrapper Subtraction Error 9)
    RewardWrapperSubtractionError9 = 10241,
    /// 65,546 for (Reward Wrapper Subtraction Error 10)
    RewardWrapperSubtractionError10 = 10242,
    /// 65,546 for (Reward Wrapper Subtraction Error 11)
    RewardWrapperSubtractionError11 = 10243,
    /// 65,546 for (Reward Wrapper Subtraction Error 12)
    RewardWrapperSubtractionError12 = 10244,
    /// 65,546 for (Reward Wrapper Multiply Error 1)
    RewardWrapperMultiplyError1 = 10245,
    /// 65,546 for (Reward Wrapper Multiply Error 2)
    RewardWrapperMultiplyError2 = 10246,
    /// 65,546 for (Reward Wrapper Multiply Error 3)
    RewardWrapperMultiplyError3 = 10247,
    /// 65,546 for (Reward Wrapper Multiply Error 4)
    RewardWrapperMultiplyError4 = 10248,
    /// 65,546 for (Reward Wrapper Multiply Error 5)
    RewardWrapperMultiplyError5 = 10249,
    /// 65,546 for (Reward Wrapper Multiply Error 6)
    RewardWrapperMultiplyError6 = 10250,
    /// 65,546 for (Reward Wrapper Multiply Error 7)
    RewardWrapperMultiplyError7 = 10251,
    /// 65,546 for (Reward Wrapper Multiply Error 8)
    RewardWrapperMultiplyError8 = 10252,
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
    ///Curve token v3 errors
    /// 65,538 for (Curve Token V3 Only Minter Can Set)
    CurveTokenV3OnlyMinterCanSet = 10501,
    /// 65,538 for (Curve Token V3 Under Fow1)
    CurveTokenV3UnderFlow1 = 10502,
    /// 65,538 for (Curve Token V3 Over Flow1)
    CurveTokenV3OverFlow1 = 10503,
    /// 65,538 for (Curve Token V3 Under Flow2)
    CurveTokenV3UnderFlow2 = 10504,
    /// 65,538 for (Curve Token V3 Over Flow2)
    CurveTokenV3OverFlow2 = 10505,
    /// 65,538 for (Curve Token V3 Reduce Spender AllowanceToZero )
    CurveTokenV3ReduceSpenderAllowanceToZero = 10506,
    /// 65,538 for (Curve Token V3 Only Minter Allowed )
    CurveTokenV3OnlyMinterAllowed = 10507,
    /// 65,538 for (Curve Token V3 Only Address not Allowed)
    CurveTokenV3ZeroAddressNotAllowed = 10508,
    /// 65,538 for (Curve Token V3  OverFlow3)
    CurveTokenV3OverFlow3 = 10509,
    /// 65,538 for (Curve Token V3  OverFlow4)
    CurveTokenV3OverFlow4 = 10510,
    /// 65,538 for (Curve Token V3  Zero address)
    CurveTokenV3ZeroAddress = 10511,
    /// 65,538 for (Curve Token V3  Under Flow5)
    CurveTokenV3UnderFlow5 = 10512,
    /// 65,538 for (Curve Token V3  Under Flow6)
    CurveTokenV3UnderFlow6 = 10513,
    /// 65,538 for (Curve Token V3  Only Minter Allowed1)
    CurveTokenV3OnlyMinterAllowed1 = 10514,
    /// 65,538 for (Curve Token V3  Only Minter Allowed2)
    CurveTokenV3OnlyMinterAllowed2 = 10515,
    /// 65,538 for (Curve Token V3  Over Flow6)
    CurveTokenV3OverFlow6 = 10516,
    /// 65,538 for (Curve Token V3 Under Flow7)
    CurveTokenV3UnderFlow7 = 10517,
    /// 65,538 for (Curve Token V3 Not Authorized)
    CurveTokenV3NotAuthorized = 10518,
    /// 65,538 for (Curve Token V3 Under Flow8)
    CurveTokenV3UnderFlow8 = 10519,
    /// 65,538 for (Curve Token V3 Under Flow9)
    CurveTokenV3UnderFlow9 = 10520,
    // FeeDistributor
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
    /// 65,539 for (Fee Distributor Division1)
    FeeDistributorDivisionError1 = 10615,
    /// 65,540 for (Fee Distributor Division2)
    FeeDistributorDivisionError2 = 10616,
    /// 65,541 for (Fee Distributor Division3)
    FeeDistributorDivisionError3 = 10617,
    /// 65,542 for (Fee Distributor Division4)
    FeeDistributorDivisionError4 = 10618,
    /// 65,543 for (Fee Distributor Division5)
    FeeDistributorDivisionError5 = 10619,
    /// 65,544 for (Fee Distributor Division6)
    FeeDistributorDivisionError6 = 10620,
    /// 65,545 for (Fee Distributor Division7)
    FeeDistributorDivisionError7 = 10621,
    /// 65,546 for (Fee Distributor Division8)
    FeeDistributorDivisionError8 = 10622,
    /// 65,547 for (Fee Distributor Division9)
    FeeDistributorDivisionError9 = 10623,
    /// 65,548 for (Fee Distributor Division10)
    FeeDistributorDivisionError10 = 10624,
    /// 65,549 for (Fee Distributor Division11)
    FeeDistributorDivisionError11 = 10625,
    /// 65,554 for (Fee Distributor Subtraction1)
    FeeDistributorSubtractionError1 = 10626,
    /// 65,555 for (Fee Distributor Subtraction2)
    FeeDistributorSubtractionError2 = 10627,
    /// 65,556 for (Fee Distributor Subtraction3)
    FeeDistributorSubtractionError3 = 10628,
    /// 65,557 for (Fee Distributor Subtraction4)
    FeeDistributorSubtractionError4 = 10629,
    /// 65,558 for (Fee Distributor Subtraction5)
    FeeDistributorSubtractionError5 = 10630,
    /// 65,559 for (Fee Distributor Subtraction6)
    FeeDistributorSubtractionError6 = 10631,
    /// 65,560 for (Fee Distributor Subtraction7)
    FeeDistributorSubtractionError7 = 10632,
    /// 65,561 for (Fee Distributor Subtraction8)
    FeeDistributorSubtractionError8 = 10633,
    /// 65,562 for (Fee Distributor Subtraction9)
    FeeDistributorSubtractionError9 = 10634,
    /// 65,563 for (Fee Distributor Subtraction10)
    FeeDistributorSubtractionError10 = 10635,
    /// 65,564 for (Fee Distributor Subtraction11)
    FeeDistributorSubtractionError11 = 10636,
    /// 65,565 for (Fee Distributor Subtraction12)
    FeeDistributorSubtractionError12 = 10637,
    /// 65,566 for (Fee Distributor Subtraction13)
    FeeDistributorSubtractionError13 = 10638,
    /// 65,567 for (Fee Distributor Subtraction14)
    FeeDistributorSubtractionError14 = 10639,
    /// 65,568 for (Fee Distributor Subtraction15)
    FeeDistributorSubtractionError15 = 10640,
    /// 65,569 for (Fee Distributor Subtraction16)
    FeeDistributorSubtractionError16 = 10641,
    /// 65,570 for (Fee Distributor Subtraction17)
    FeeDistributorSubtractionError17 = 10642,
    /// 65,574 for (Fee Distributor Addition1)
    FeeDistributorAdditionError1 = 10643,
    /// 65,575 for (Fee Distributor Addition2)
    FeeDistributorAdditionError2 = 10644,
    /// 65,576 for (Fee Distributor Addition3)
    FeeDistributorAdditionError3 = 10645,
    /// 65,577 for (Fee Distributor Addition4)
    FeeDistributorAdditionError4 = 10646,
    /// 65,578 for (Fee Distributor Addition5)
    FeeDistributorAdditionError5 = 10647,
    /// 65,579 for (Fee Distributor Addition6)
    FeeDistributorAdditionError6 = 10648,
    /// 65,570 for (Fee Distributor Addition7)
    FeeDistributorAdditionError7 = 10649,
    /// 65,571 for (Fee Distributor Addition8)
    FeeDistributorAdditionError8 = 10650,
    /// 65,572 for (Fee Distributor Addition9)
    FeeDistributorAdditionError9 = 10651,
    /// 65,573 for (Fee Distributor Addstion10)
    FeeDistributorAdditionError10 = 10652,
    /// 65,574 for (Fee Distributor Addition11)
    FeeDistributorAdditionError11 = 10653,
    /// 65,575 for (Fee Distributor Addition12)
    FeeDistributorAdditionError12 = 10654,
    /// 65,576 for (Fee Distributor Addition13)
    FeeDistributorAdditionError13 = 10655,
    /// 65,577 for (Fee Distributor Addition14)
    FeeDistributorAdditionError14 = 10656,
    /// 65,578 for (Fee Distributor Addition15)
    FeeDistributorAdditionError15 = 10657,
    /// 65,579 for (Fee Distributor Addition16)
    FeeDistributorAdditionError16 = 10658,
    /// 65,580 for (Fee Distributor Addition17)
    FeeDistributorAdditionError17 = 10659,
    /// 65,581 for (Fee Distributor Addition18)
    FeeDistributorAdditionError18 = 10660,
    /// 65,582 for (Fee Distributor Addition19)
    FeeDistributorAdditionError19 = 10661,
    /// 65,584 for (Fee Distributor Multiplication1)
    FeeDistributorMultiplicationError1 = 10662,
    /// 65,585 for (Fee Distributor Multiplication2)
    FeeDistributorMultiplicationError2 = 10663,
    /// 65,586 for (Fee Distributor Multiplication3)
    FeeDistributorMultiplicationError3 = 10664,
    /// 65,587 for (Fee Distributor Multiplication4)
    FeeDistributorMultiplicationError4 = 10665,
    /// 65,588 for (Fee Distributor Multiplication5)
    FeeDistributorMultiplicationError5 = 10666,
    /// 65,589 for (Fee Distributor Multiplication6)
    FeeDistributorMultiplicationError6 = 10667,
    /// 65,590 for (Fee Distributor Multiplication7)
    FeeDistributorMultiplicationError7 = 10668,
    /// 65,591 for (Fee Distributor Multiplication8)
    FeeDistributorMultiplicationError8 = 10669,
    /// 65,592 for (Fee Distributor Multiplication9)
    FeeDistributorMultiplicationError9 = 10670,
    /// 65,593 for (Fee Distributor Multiplication10)
    FeeDistributorMultiplicationError10 = 10671,
    /// 65,594 for (Fee Distributor Multiplication11)
    FeeDistributorMultiplicationError11 = 10672,
    // Gauge Controller
    /// 76,236 for (Gauge Controller Address Zero1)
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
    /// 76,305 for (Gauge Controller UnderFlow23)
    GaugeControllerUnderFlow24 = 10768,
    /// 76,306 for (Gauge Controller Multiply1)
    GaugeControllerMultiply1 = 10769,
    /// 76,306 for (Gauge Controller Multiply2)
    GaugeControllerMultiply2 = 10770,
    /// 76,306 for (Gauge Controller Multiply3)
    GaugeControllerMultiply3 = 10771,
    /// 76,306 for (Gauge Controller Multiply4)
    GaugeControllerMultiply4 = 10772,
    /// 76,306 for (Gauge Controller Multiply5)
    GaugeControllerMultiply5 = 10773,
    /// 76,306 for (Gauge Controller Multiply6)
    GaugeControllerMultiply6 = 10774,
    /// 76,306 for (Gauge Controller Multiply7)
    GaugeControllerMultiply7 = 10775,
    /// 76,306 for (Gauge Controller Multiply8)
    GaugeControllerMultiply8 = 10776,
    /// 76,306 for (Gauge Controller Multiply9)
    GaugeControllerMultiply9 = 10777,
    /// 76,306 for (Gauge Controller Multiply10)
    GaugeControllerMultiply10 = 10778,
    /// 76,306 for (Gauge Controller Multiply11)
    GaugeControllerMultiply11 = 10779,
    /// 76,306 for (Gauge Controller Multiply12)
    GaugeControllerMultiply12 = 10780,
    /// 76,306 for (Gauge Controller Multiply13)
    GaugeControllerMultiply13 = 10781,
    /// 76,306 for (Gauge Controller Multiply14)
    GaugeControllerMultiply14 = 10782,
    /// 76,306 for (Gauge Controller Multiply15)
    GaugeControllerMultiply15 = 10783,
    /// 76,306 for (Gauge Controller Multiply16)
    GaugeControllerMultiply16 = 10784,
    /// 76,306 for (Gauge Controller Multiply17)
    GaugeControllerMultiply17 = 10785,
    /// 76,306 for (Gauge Controller Multiply18)
    GaugeControllerMultiply18 = 10786,
    /// 76,306 for (Gauge Controller Multiply19)
    GaugeControllerMultiply19 = 10787,
    /// 76,306 for (Gauge Controller Divide1)
    GaugeControllerDivide1 = 10788,
    /// 76,306 for (Gauge Controller Divide2)
    GaugeControllerDivide2 = 10789,
    /// 76,306 for (Gauge Controller Divide3)
    GaugeControllerDivide3 = 10790,
    /// 76,306 for (Gauge Controller Divide4)
    GaugeControllerDivide4 = 10791,
    /// 76,306 for (Gauge Controller Divide5)
    GaugeControllerDivide5 = 10792,
    /// 76,306 for (Gauge Controller Divide6)
    GaugeControllerDivide6 = 10793,
    /// 76,306 for (Gauge Controller Divide7)
    GaugeControllerDivide7 = 10794,
    /// 76,306 for (Gauge Controller Divide8)
    GaugeControllerDivide8 = 10795,

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
    ///Vesting Escrow simple errors
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
    /// 65,540 for (Vesting Escrow Simple Is Locked1)
    VestingEscrowSimpleLocked1 = 11208,
    /// 65,540 for (Vesting Escrow Simple Is Locked2)
    VestingEscrowSimpleLocked2 = 11209,
    /// 65,540 for (Vesting Escrow Simple Airthmetic Error1)
    VestingEscrowSimpleAirthmeticError1 = 11210,
    /// 65,540 for (Vesting Escrow Simple Airthmetic Error2)
    VestingEscrowSimpleAirthmeticError2 = 11211,
    /// 65,540 for (Vesting Escrow Simple Under flow1)
    VestingEscrowSimpleUnderFlow1 = 11212,
    /// 65,540 for (Vesting Escrow Simple Under flow2)
    VestingEscrowSimpleUnderFlow2 = 11213,
    /// 65,540 for (Vesting Escrow Simple Under flow3)
    VestingEscrowSimpleUnderFlow3 = 11214,
    /// 65,540 for (Vesting Escrow Simple Under flow4)
    VestingEscrowSimpleUnderFlow4 = 11215,

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
    /// 65,541 for (Voting Escrow Division Error 1)
    VotingEscrowDivisionError1 = 11323,
    /// 65,542 for (Voting Escrow Division Error 2)
    VotingEscrowDivisionError2 = 11324,
    /// 65,543 for (Voting Escrow Division Error 3)
    VotingEscrowDivisionError3 = 11325,
    /// 65,544 for (Voting Escrow Division Error 4)
    VotingEscrowDivisionError4 = 11326,
    /// 65,545 for (Voting Escrow Division Error 5)
    VotingEscrowDivisionError5 = 11327,
    /// 65,546 for (Voting Escrow DivSsion Error 6)
    VotingEscrowDivisionError6 = 11328,
    /// 65,547 for (Voting Escrow Division Error 7)
    VotingEscrowDivisionError7 = 11329,
    /// 65,548 for (Voting Escrow Division Error 8)
    VotingEscrowDivisionError8 = 11330,
    /// 65,549 for (Voting Escrow Division Error 9)
    VotingEscrowDivisionError9 = 11331,
    /// 65,557 for (Voting Escrow Subtraction Error 1)
    VotingEscrowSubtractionError1 = 11332,
    /// 65,558 for (Voting Escrow Subtraction Error 2)
    VotingEscrowSubtractionError2 = 11333,
    /// 65,559 for (Voting Escrow Subtraction Error 3)
    VotingEscrowSubtractionError3 = 11334,
    /// 65,560 for (Voting Escrow Subtraction Error 4)
    VotingEscrowSubtractionError4 = 11335,
    /// 65,561 for (Voting Escrow Subtraction Error 5)
    VotingEscrowSubtractionError5 = 11336,
    /// 65,562 for (Voting Escrow Subtraction Error 6)
    VotingEscrowSubtractionError6 = 11337,
    /// 65,563 for (Voting Escrow Subtraction Error 7)
    VotingEscrowSubtractionError7 = 11338,
    /// 65,564 for (Voting Escrow Subtraction Error 8)
    VotingEscrowSubtractionError8 = 11339,
    /// 65,565 for (Voting Escrow Subtraction Error 9)
    VotingEscrowSubtractionError9 = 11340,
    /// 65,566 for (Voting Escrow Subtraction Error 10)
    VotingEscrowSubtractionError10 = 11341,
    /// 65,567 for (Voting Escrow Subtraction Error 11)
    VotingEscrowSubtractionError11 = 11342,
    /// 65,568 for (Voting Escrow Subtraction Error 12)
    VotingEscrowSubtractionError12 = 11343,
    /// 65,569 for (Voting Escrow Subtraction Error 13)
    VotingEscrowSubtractionError13 = 11344,
    /// 65,570 for (Voting Escrow Subtraction Error 14)
    VotingEscrowSubtractionError14 = 11345,
    /// 65,571 for (Voting Escrow Subtraction Error 15)
    VotingEscrowSubtractionError15 = 11346,
    /// 65,572 for (Voting Escrow Subtraction Error 16)
    VotingEscrowSubtractionError16 = 11347,
    /// 65,573 for (Voting Escrow Subtraction Error 17)
    VotingEscrowSubtractionError17 = 11348,
    /// 65,574 for (Voting Escrow Subtraction Error 18)
    VotingEscrowSubtractionError18 = 11349,
    /// 65,575 for (Voting Escrow Subtraction Error 19)
    VotingEscrowSubtractionError19 = 11350,
    /// 65,576 for (Voting Escrow Subtraction Error 20)
    VotingEscrowSubtractionError20 = 11351,
    /// 65,577 for (Voting Escrow Subtraction Error 21)
    VotingEscrowSubtractionError21 = 11352,
    /// 65,578 for (Voting Escrow Subtraction Error 22)
    VotingEscrowSubtractionError22 = 11353,
    /// 65,579 for (Voting Escrow Subtraction Error 23)
    VotingEscrowSubtractionError23 = 11354,
    /// 65,580 for (Voting Escrow Subtraction Error 24)
    VotingEscrowSubtractionError24 = 11355,
    /// 65,581 for (Voting Escrow Subtraction Error 25)
    VotingEscrowSubtractionError25 = 11356,
    /// 65,582 for (Voting Escrow Subtraction Error 26)
    VotingEscrowSubtractionError26 = 11357,
    /// 65,583 for (Voting Escrow Subtraction Error 27)
    VotingEscrowSubtractionError27 = 11358,
    /// 65,584 for (Voting Escrow Subtraction Error 28)
    VotingEscrowSubtractionError28 = 11359,
    /// 65,585 for (Voting Escrow Multiplication Error 1)
    VotingEscrowMultiplicationError1 = 11360,
    /// 65,586 for (Voting Escrow Multiplication Error 2)
    VotingEscrowMultiplicationError2 = 11361,
    /// 65,587 for (Voting Escrow Multiplication Error 3)
    VotingEscrowMultiplicationError3 = 11362,
    /// 65,588 for (Voting Escrow Multiplication Error 4)
    VotingEscrowMultiplicationError4 = 11363,
    /// 65,589 for (Voting Escrow Multiplication Error 5)
    VotingEscrowMultiplicationError5 = 11364,
    /// 65,590 for (Voting Escrow Multiplication Error 6)
    VotingEscrowMultiplicationError6 = 11365,
    /// 65,591 for (Voting Escrow Multiplication Error 7)
    VotingEscrowMultiplicationError7 = 11366,
    /// 65,592 for (Voting Escrow Multiplication Error 8)
    VotingEscrowMultiplicationError8 = 11367,
    /// 65,593 for (Voting Escrow Multiplication Error 9)
    VotingEscrowMultiplicationError9 = 11368,
    /// 65,594 for (Voting Escrow Multiplication Error 10)
    VotingEscrowMultiplicationError10 = 11369,
    /// 65,595 for (Voting Escrow Multiplication Error 11)
    VotingEscrowMultiplicationError11 = 11370,
    /// 65,596 for (Voting Escrow Multiplication Error 12)
    VotingEscrowMultiplicationError12 = 11371,
    /// 65,597 for (Voting Escrow Multiplication Error 13)
    VotingEscrowMultiplicationError13 = 11372,
    /// 65,598 for (Voting Escrow Multiplication Error 14)
    VotingEscrowMultiplicationError14 = 11373,
    /// 65,599 for (Voting Escrow Addition Error 1)
    VotingEscrowAdditionError1 = 11374,
    /// 65,600 for (Voting Escrow Addition Error 2)
    VotingEscrowAdditionError2 = 11375,
    /// 65,601 for (Voting Escrow Addition Error 3)
    VotingEscrowAdditionError3 = 11376,
    /// 65,602 for (Voting Escrow Addition Error 4)
    VotingEscrowAdditionError4 = 11377,
    /// 65,603 for (Voting Escrow Addition Error 5)
    VotingEscrowAdditionError5 = 11378,
    /// 65,604 for (Voting Escrow Addition Error 6)
    VotingEscrowAdditionError6 = 11379,
    /// 65,605 for (Voting Escrow Addition Error 7)
    VotingEscrowAdditionError7 = 11380,
    /// 65,606 for (Voting Escrow Addition Error 8)
    VotingEscrowAdditionError8 = 11381,
    /// 65,607 for (Voting Escrow Addition Error 9)
    VotingEscrowAdditionError9 = 11382,
    /// 65,608 for (Voting Escrow Addition Error 10)
    VotingEscrowAdditionError10 = 11383,
    /// 65,609 for (Voting Escrow Addition Error 11)
    VotingEscrowAdditionError11 = 11384,
    /// 65,610 for (Voting Escrow Addition Error 12)
    VotingEscrowAdditionError12 = 11385,
    /// 65,611 for (Voting Escrow Addition Error 13)
    VotingEscrowAdditionError13 = 11386,
    /// 65,612 for (Voting Escrow Addition Error 14)
    VotingEscrowAdditionError14 = 11387,
    /// 65,613 for (Voting Escrow Addition Error 15)
    VotingEscrowAdditionError15 = 11388,
    /// 65,614 for (Voting Escrow Addition Error 16)
    VotingEscrowAdditionError16 = 11389,
    /// 65,615 for (Voting Escrow Addition Error 17)
    VotingEscrowAdditionError17 = 11390,
    /// 65,616 for (Voting Escrow Addition Error 18)
    VotingEscrowAdditionError18 = 11391,
    /// 65,617 for (Voting Escrow Addition Error 19)
    VotingEscrowAdditionError19 = 11392,
    /// 65,618 for (Voting Escrow Addition Error 20)
    VotingEscrowAdditionError20 = 11393,
    /// 65,619 for (Voting Escrow Addition Error 21)
    VotingEscrowAdditionError21 = 11394,
    /// 65,620 for (Voting Escrow Addition Error 22)
    VotingEscrowAdditionError22 = 11395,
    /// 65,621 for (Voting Escrow Addition Error 23)
    VotingEscrowAdditionError23 = 11496,
    /// 65,622 for (Voting Escrow Addition Error 24)
    VotingEscrowAdditionError24 = 11397,
    /// 65,623 for (Voting Escrow Addition Error 25)
    VotingEscrowAdditionError25 = 11398,
    /// 65,624 for (Voting Escrow Addition Error 26)
    VotingEscrowAdditionError26 = 11399,
    /// 65,625 for (Voting Escrow Week Multiplication Error 1)
    VotingEscrowWeekMultiplicationError1 = 11400,
    /// 65,626 for (Voting Escrow Week Division Error 1)
    VotingEscrowWeekDivisionError1 = 11401,
    /// 65,627 for (Voting Escrow Week Multiplication Error 2)
    VotingEscrowWeekMultiplicationError2 = 11402,
    /// 65,628 for (Voting Escrow Week Division Error 2)
    VotingEscrowWeekDivisionError2 = 11403,
    /// 65,637 for (Voting Escrow Invalid Block Timestamp 1)
    VotingEscrowInvalidBlockTimestamp1 = 11404,
    /// 65,638 for (Voting Escrow Invalid Block Timestamp 2)
    VotingEscrowInvalidBlockTimestamp2 = 11405,

    /// 65,718 for (Liquidity Gauge Wrapper Unauthorized)
    GaugeWrapperUnauthorized = 11501,
    /// 65,719 for (Liquidity Gauge Wrapper Is Killed)
    GaugeWrapperIsKilled1 = 11502,
    /// 65,720 for (Liquidity Gauge Wrapper Not Approved)
    GaugeWrapperNotApproved = 11503,
    /// 65,721 for (Liquidity Gauge Wrapper IsLocked1)
    GaugeWrapperIsLocked1 = 11504,
    /// 65,722 for (Liquidity Gauge Wrapper IsLocked2)
    GaugeWrapperIsLocked2 = 11505,
    /// 65,723 for (Liquidity Gauge Wrapper Is Locked3)
    GaugeWrapperIsLocked3 = 11506,
    /// 65,724 for (Liquidity Gauge Wrapper Admin Only1)
    GaugeWrapperAdminOnly1 = 11507,
    /// 65,725 for (Liquidity Gauge Wrapper Admin Only2)
    GaugeWrapperAdminOnly2 = 11508,
    /// 65,726 for (Liquidity Gauge Wrapper Admin Only3)
    GaugeWrapperAdminOnly3 = 11509,
    /// 65,727 for (Liquidity Gauge Wrapper Admin Not Set)
    GaugeWrapperAdminNotSet = 11510,
    /// 65,728 for (Liquidity Gauge Wrapper Zero Address)
    GaugeWrapperZeroAddress = 11511,
    /// 65,729 for (Liquidity Gauge Wrapper Is Killed)
    GaugeWrapperIsKilled2 = 11512,
    /// 65,730 for (Liquidity Gauge Wrapper Addition Error 1)
    GaugeWrapperAdditionError1 = 11513,
    /// 65,731 for (Liquidity Gauge Wrapper Addition Error 2)
    GaugeWrapperAdditionError2 = 11514,
    /// 65,732 for (Liquidity Gauge Wrapper Addition Error 3)
    GaugeWrapperAdditionError3 = 11515,
    /// 65,733 for (Liquidity Gauge Wrapper Addition Error 4)
    GaugeWrapperAdditionError4 = 11516,
    /// 65,734 for (Liquidity Gauge Wrapper Addition Error 5)
    GaugeWrapperAdditionError5 = 11517,
    /// 65,735 for (Liquidity Gauge Wrapper Addition Error 6)
    GaugeWrapperAdditionError6 = 11518,
    /// 65,736 for (Liquidity Gauge Wrapper Addition Error 7)
    GaugeWrapperAdditionError7 = 11519,
    /// 65,737 for (Liquidity Gauge Wrapper Addition Error 8)
    GaugeWrapperAdditionError8 = 11520,
    /// 65,738 for (Liquidity Gauge Wrapper Subtraction Error 1)
    GaugeWrapperSubtractionError1 = 11521,
    /// 65,739 for (Liquidity Gauge Wrapper Subtraction Error 2)
    GaugeWrapperSubtractionError2 = 11522,
    /// 65,740 for (Liquidity Gauge Wrapper Subtraction Error 3)
    GaugeWrapperSubtractionError3 = 11523,
    /// 65,741 for (Liquidity Gauge Wrapper Subtraction Error 4)
    GaugeWrapperSubtractionError4 = 11524,
    /// 65,742 for (Liquidity Gauge Wrapper Subtraction Error 5)
    GaugeWrapperSubtractionError5 = 11525,
    /// 65,743 for (Liquidity Gauge Wrapper Subtraction Error 6)
    GaugeWrapperSubtractionError6 = 11526,
    /// 65,744 for (Liquidity Gauge Wrapper Subtraction Error 7)
    GaugeWrapperSubtractionError7 = 11527,
    /// 65,745 for (Liquidity Gauge Wrapper Subtraction Error 8)
    GaugeWrapperSubtractionError8 = 11528,
    /// 65,746 for (Liquidity Gauge Wrapper Division Error 1)
    GaugeWrapperDivisionError1 = 11529,
    /// 65,747 for (Liquidity Gauge Wrapper Division Error 2)
    GaugeWrapperDivisionError2 = 11530,
    /// 65,748 for (Liquidity Gauge Wrapper Division Error 3)
    GaugeWrapperDivisionError3 = 11531,
    /// 65,749 for (Liquidity Gauge Wrapper Division Error 4)
    GaugeWrapperDivisionError4 = 11532,
    /// 65,750 for (Liquidity Gauge Wrapper Multiply Error 1)
    GaugeWrapperMultiplyError1 = 11533,
    /// 65,751 for (Liquidity Gauge Wrapper Multiply Error 2)
    GaugeWrapperMultiplyError2 = 11534,
    /// 65,752 for (Liquidity Gauge Wrapper Multiply Error 3)
    GaugeWrapperMultiplyError3 = 11535,
    /// 65,753 for (Liquidity Gauge Wrapper Multiply Error 4)
    GaugeWrapperMultiplyError4 = 11536,

    /// 65,546 for (Ownable: caller is not the owner)
    OwnableNotOwner = 11698,
    /// 65,540 for (Ownable: new owner is the zero address)
    OwnableNewOwnerAddressZero = 11699,

    /// 65,540 for (IRewardDistributionRecipient: Caller is not reward distribution)
    NotRewardDistribution = 11701,
    //65,540 for (CurveRewards:Cannot stake 0)
    CurveRewardsCannotStake = 11801,
    //65,540 for (CurveRewards:Cannot withdraw 0)
    CurveRewardsCannotWithdraw = 11802,
    //65,540 for (CurveRewards:Addition Error 1)
    CurveRewardsAdditionError1 = 11803,
    //65,540 for (CurveRewards:Addition Error 2)
    CurveRewardsAdditionError2 = 11804,
    //65,540 for (CurveRewards:Addition Error 3)
    CurveRewardsAdditionError3 = 11805,
    //65,540 for (CurveRewards:Addition Error 4)
    CurveRewardsAdditionError4 = 11806,
    //65,540 for (CurveRewards:Division Error 1)
    CurveRewardsDivisionError1 = 11807,
    //65,540 for (CurveRewards:Division Error 2)
    CurveRewardsDivisionError2 = 11808,
    //65,540 for (CurveRewards:Division Error 3)
    CurveRewardsDivisionError3 = 11809,
    //65,540 for (CurveRewards:Division Error 4)
    CurveRewardsDivisionError4 = 11810,
    //65,540 for (CurveRewards:Subtraction Error 1)
    CurveRewardsSubtractionError1 = 11811,
    //65,540 for (CurveRewards:Subtraction Error 2)
    CurveRewardsSubtractionError2 = 11812,
    //65,540 for (CurveRewards:Subtraction Error 3)
    CurveRewardsSubtractionError3 = 11813,
    //65,540 for (CurveRewards:Multiply Error 1)
    CurveRewardsMultiplyError1 = 11814,
    //65,540 for (CurveRewards:Multiply Error 2)
    CurveRewardsMultiplyError2 = 11815,
    //65,540 for (CurveRewards:Multiply Error 3)
    CurveRewardsMultiplyError3 = 11816,
    //65,540 for (CurveRewards:Multiply Error 4)
    CurveRewardsMultiplyError4 = 11817,
    //LIQUIDITY GUAGE V3
    /// 65,538 for (Liquidity guage v3 OverFlow1)
    LiquidityGaugeOverFlow1 = 11900,
    /// 65,539 for (Liquidity guage v3 OverFlow2)
    LiquidityGaugeOverFlow2 = 11901,
    /// 65,540 for (Liquidity guage v3 OverFlow3)
    LiquidityGaugeOverFlow3 = 11902,
    /// 65,541 for (Liquidity guage v3 OverFlow4)
    LiquidityGaugeOverFlow4 = 11903,
    /// 65,541 for (Liquidity guage v3 OverFlow5)
    LiquidityGaugeOverFlow5 = 11904,
    /// 65,541 for (Liquidity guage v3 OverFlow6)
    LiquidityGaugeOverFlow6 = 11905,
    /// 65,541 for (Liquidity guage v3 OverFlow7)
    LiquidityGaugeOverFlow7 = 11906,
    /// 65,542 for (Liquidity guage v3 UnderFlow1)
    LiquidityGaugeUnderFlow1 = 11907,
    /// 65,543 for (Liquidity guage v3 UnderFlow2)
    LiquidityGaugeUnderFlow2 = 11908,
    /// 65,544 for (Liquidity guage v3 UnderFlow3)
    LiquidityGaugeUnderFlow3 = 11909,
    /// 65,545 for (Liquidity guage v3 UnderFlow4)
    LiquidityGaugeUnderFlow4 = 11910,
    /// 65,546 for (Liquidity guage v3UnderFlow5)
    LiquidityGaugeUnderFlow5 = 11912,
    /// 65,546 for (Liquidity guage v3 UnderFlow6)
    LiquidityGaugeUnderFlow6 = 11913,
    /// 65,546 for (Liquidity guage v3UnderFlow7)
    LiquidityGaugeUnderFlow7 = 11914,
    /// 65,546 for (Liquidity guage v3UnderFlow8)
    LiquidityGaugeUnderFlow8 = 11915,
    /// 65,546 for (Liquidity guage v3 UnderFlow9)
    LiquidityGaugeUnderFlow9 = 11916,
    /// 65,540 for (Liquidity guage v3 Only Admin1)
    LiquidityGaugeOnlyAdmin1 = 11917,
    /// 65,540 for (Liquidity guage v3 Only Admin2)
    LiquidityGaugeOnlyAdmin2 = 11918,
    /// 65,540 for (Liquidity guage v3 Only Future Admin)
    LiquidityGaugeOnlyFutureAdmin = 11919,
    /// 65,540 for (Liquidity guage v3 Cannot Redirect When Claiming For Another User)
    LiquidityGaugeCannotRedirectWhenClaimingForAnotherUser = 11920,
    /// 65,540 for (Liquidity guage v3 Value Is Zero)
    LiquidityGaugeValueIsZero1 = 11921,
    /// 65,540 for (Liquidity guage v3 Value Is Zero)
    LiquidityGaugeValueIsZero2 = 11922,
    /// 65,540 for (Liquidity guage v3 Reward Token Is Zero)
    LiquidityGaugeRewardTokenIsZeroAddress = 11923,
    /// 65,540 for (Liquidity guage v3 Cannot Modify Existing Reward Token)
    LiquidityGaugeCannotModifyExistingRewardToken = 11924,
    /// 65,540 for (Liquidity guage locked 1)
    LiquidityGaugeLocked1 = 11925,
    /// 65,540 for (Liquidity guage v3 unauthorized)
    LiquidityGuageV3Unauthorized = 11926,
    /// 65,540 for (Liquidity guage v3 Underflow 10)
    LiquidityGaugeUnderFlow10 = 11927,
    /// 65,540 for (Liquidity guage v3 Underflow 11)
    LiquidityGaugeUnderFlow11 = 11928,
    /// 65,540 for (Liquidity guage v3 Kick not allowed)
    LiquidityGuageV3KickNotAllowed = 11929,
    /// 65,540 for (Liquidity guage v3 TokenIsZeroAddress)
    LiquidityGaugeTokenIsZeroAddress = 11930,
    /// 65,540 for (Liquidity guage v3 zero total supply)
    LiquidityGaugeZeroTotalSupply = 11931,
    /// 65,540 for (Liquidity guage v3 failed deposit)
    LiquidityGaugeFailedToDeposit = 11932,
    /// 65,540 for (Liquidity guage v3 failed to withdraw)
    LiquidityGaugeFailedToWithdraw = 11933,
    /// 65,540 for (Liquidity guage v3 cannot modify existing reward token)
    LiquidityGaugeCannotModifyExistingRewardtoken = 11934,
    /// 65,540 for (Liquidity guage v3 Kick not needed)
    LiquidityGuageV3KickNotNeeded = 11935,
    /// 65,540 for (Liquidity guage locked 2)
    LiquidityGaugeLocked2 = 11936,
    /// 65,540 for (Liquidity guage v3 locked 3)
    LiquidityGaugeLocked3 = 11937,
    /// 65,540 for (Liquidity guage v3 locked 4)
    LiquidityGaugeLocked4 = 11938,
    /// 65,540 for (Liquidity guage v3 locked 5)
    LiquidityGaugeLocked5 = 11939,
    /// 65,540 for (Liquidity guage v3 locked 6)
    LiquidityGaugeLocked6 = 11940,
    /// 65,540 for (Liquidity guage v3 locked 7)
    LiquidityGaugeLocked7 = 11941,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 1)
    LiquidityGaugeArithmeticError1 = 11942,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 2)
    LiquidityGaugeArithmeticError2 = 11943,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 3)
    LiquidityGaugeArithmeticError3 = 11944,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 4)
    LiquidityGaugeArithmeticError4 = 11945,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 5)
    LiquidityGaugeArithmeticError5 = 11946,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 6)
    LiquidityGaugeArithmeticError6 = 11947,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 7)
    LiquidityGaugeArithmeticError7 = 11948,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 8)
    LiquidityGaugeArithmeticError8 = 11949,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 9)
    LiquidityGaugeArithmeticError9 = 11950,
    /// 65,540 for (Liquidity guage locked 10)
    LiquidityGaugeArithmeticError10 = 11951,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 11)
    LiquidityGaugeArithmeticError11 = 11952,
    /// 65,540 for (Liquidity guage v3 Arithmetic error 12)
    LiquidityGaugeArithmeticError12 = 11953,

    /// 65,540 for (Lp Token Wrapper Addition Error 1)
    LpTokenWrapperAdditionError1 = 12001,
    /// 65,540 for (Lp Token Wrapper Addition Error 2)
    LpTokenWrapperAdditionError2 = 12002,
    /// 65,540 for (Lp Token Wrapper Subtraction Error 1)
    LpTokenWrapperSubtractionError1 = 12003,
    /// 65,540 for (Lp Token Wrapper Subtraction Error 2)
    LpTokenWrapperSubtractionError2 = 12004,

    Abort = 150,
    SmartContractDepositorsNotAllowed = 151,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
