import { config } from "dotenv";
import * as utils from "../src/utils";
import {
  CLAccountHash,
  CLByteArray,
  CLValueBuilder,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";
import { install, reserveWise } from "../src/install";
config();

const {
  NODE_ADDRESS,
  MASTER_KEY_PAIR_PATH,
  BASE_PATH,
  // WASMS
  ERC20_WASM,
  CURVE_REWARDS_WASM,
  CURVE_TOKEN_V3_WASM,
  ERC20_CRV_WASM,
  FEE_DISTRIBUTOR_WASM,
  GAUGE_CONTROLLER_WASM,
  GAUGE_PROXY_WASM,
  I_REWARD_DISTRIBUTION_RECIPIENT_WASM,
  LIQUIDITY_GAUGE_V3_WASM,
  LIQUIDITY_GAUGE_REWARD_WASM,
  LIQUIDITY_GAUGE_REWARD_WRAPPER_WASM,
  LIQUIDITY_GAUGE_WRAPPER_WASM,
  LP_TOKEN_WRAPPER_WASM,
  MINTER_WASM,
  OWNABLE_WASM,
  REWARD_ONLY_GAUGE_WASM,
  VOTING_ESCROW_WASM,
  VESTING_ESCROW_SIMPLE_WASM,
  VESTING_ESCROW_WASM,
  VESTING_ESCROW_FACTORY_WASM,

  // PAYMENT
  PAYMENT_ERC20,
  PAYMENT_CURVE_REWARDS,
  PAYMENT_ERC20_CRV,
  PAYMENT_FEE_DISTRIBUTOR,
  PAYMENT_CURVE_TOKEN_V3,
  PAYMENT_GAUGE_CONTROLLER,
  PAYMENT_LIQUIDITY_GAUGE_V3,
  PAYMENT_LIQUIDITY_GAUGE_REWARD,
  PAYMENT_LIQUIDITY_GAUGE_REWARD_WRAPPER,
  PAYMENT_LIQUIDITY_GAUGE_WRAPPER,
  PAYMENT_I_REWARD_DISTRIBUTION_RECIPIENT,
  PAYMENT_OWNABLE,
  PAYMENT_REWARD_ONLY_GAUGE,
  PAYMENT_LP_TOKEN_WRAPPER,
  PAYMENT_MINTER,
  PAYMENT_GAUGE_PROXY,
  PAYMENT_VOTING_ESCROW,
  PAYMENT_VESTING_ESCROW,
  PAYMENT_VESTING_ESCROW_SIMPLE,
  PAYMENT_VESTING_ESCROW_FACTORY,
  // NAMES
  ERC20_token,
  ERC20_reward,
  CURVE_REWARDS,
  ERC20_CRV,
  FEE_DISTRIBUTOR,
  CURVE_TOKEN_V3,
  GAUGE_CONTROLLER,
  LIQUIDITY_GAUGE_V3,
  LIQUIDITY_GAUGE_REWARD,
  LIQUIDITY_GAUGE_REWARD_WRAPPER,
  LIQUIDITY_GAUGE_WRAPPER,
  REWARD_ONLY_GAUGE,
  I_REWARD_DISTRIBUTION_RECIPIENT,
  OWNABLE,
  LP_TOKEN_WRAPPER,
  MINTER,
  GAUGE_PROXY,
  VOTING_ESCROW,
  VESTING_ESCROW,
  VESTING_ESCROW_SIMPLE,
  VESTING_ESCROW_FACTORY,
  // PARAMS
  USER_ADDRESS,
  USER_ADDRESS_FRM_STR,
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const zip = require("zip-local");

const deployContract = async () => {
  // GET A DEPLOYMENT COUNT (VERSION)
  let version = utils.getDeploymentCount();
  let contractName, runtimeArgs, installHash;

  // --- ERC20 Token --- //

  console.log("Erc20 Token being deployed...");

  contractName = ERC20_token + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("Token"),
    symbol: CLValueBuilder.string("Erc"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(KEYS, PAYMENT_ERC20!, ERC20_WASM!, runtimeArgs);
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let tokenContractHash = await utils.getContractHash(contractName);
  let tokenPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "erc20TokenContractHash",
    tokenContractHash
  );
  utils.writeHashToFile(BASE_PATH + "erc20TokenPackageHash", tokenPackageHash);

  console.log("Token deployed and saved");

  // --- ERC20 Reward --- //

  console.log("ERC20 Reward being deployed...");

  contractName = ERC20_reward + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("Reward"),
    symbol: CLValueBuilder.string("Erc"),
    decimals: CLValueBuilder.u8("9"),
    initial_supply: CLValueBuilder.u256("0"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(KEYS, PAYMENT_ERC20!, ERC20_WASM!, runtimeArgs);
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let rewardContractHash = await utils.getContractHash(contractName);
  let rewardPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "erc20RewardContractHash",
    rewardContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "erc20RewardPackageHash",
    rewardPackageHash
  );

  console.log("Reward deployed and saved");

  // --- ERC20 CRV --- //

  console.log("ERC20 CRV being deployed...");

  contractName = ERC20_CRV + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("ERC20CRV"),
    symbol: CLValueBuilder.string("Crv"),
    decimals: CLValueBuilder.u8("9"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_ERC20_CRV!,
    ERC20_CRV_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let erc20CrvContractHash = await utils.getContractHash(contractName);
  let erc20CrvPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "erc20CrvContractHash",
    erc20CrvContractHash
  );
  utils.writeHashToFile(BASE_PATH + "erc20CrvPackageHash", erc20CrvPackageHash);

  console.log("ERC20 CRV deployed and saved");

  // --- Curve Rewards --- //

  console.log("Curve Rewards being deployed...");

  contractName = CURVE_REWARDS + version;
  runtimeArgs = RuntimeArgs.fromMap({
    token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    reward: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(rewardPackageHash, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_CURVE_REWARDS!,
    CURVE_REWARDS_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let curveRewardsContractHash = await utils.getContractHash(contractName);
  let curveRewardsPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "curveRewardsContractHash",
    curveRewardsContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "curveRewardsPackageHash",
    curveRewardsPackageHash
  );

  console.log("Curve Rewards deployed and saved");

  // --- Curve Token V3 --- //

  console.log("Curve Token V3 being deployed...");

  contractName = CURVE_TOKEN_V3 + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("Curve Token V3"),
    symbol: CLValueBuilder.string("CTV"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_CURVE_TOKEN_V3!,
    CURVE_TOKEN_V3_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let curveTokenV3ContractHash = await utils.getContractHash(contractName);
  let curveTokenV3PackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "curveTokenV3ContractHash",
    curveTokenV3ContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "curveTokenV3PackageHash",
    curveTokenV3PackageHash
  );

  console.log("Curve Token V3 deployed and saved");

  // --- Lp token Wrapper --- //

  console.log("LP Token Wrapper being deployed...");

  contractName = LP_TOKEN_WRAPPER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    uni: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LP_TOKEN_WRAPPER!,
    LP_TOKEN_WRAPPER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let lpTokenWrapperContractHash = await utils.getContractHash(contractName);
  let lpTokenWrapperPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "lpTokenWrapperContractHash",
    lpTokenWrapperContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "lpTokenWrapperPackageHash",
    lpTokenWrapperPackageHash
  );

  console.log("LP Token Wrapper deployed and saved");

  // --- Voting Escrow --- //

  console.log("Voting Escrow being deployed...");

  contractName = VOTING_ESCROW + version;
  runtimeArgs = RuntimeArgs.fromMap({
    token_addr: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(erc20CrvPackageHash, "hex")))
    ),
    name: CLValueBuilder.string("Vote-escrowed CRV"),
    symbol: CLValueBuilder.string("veCRV"),
    version: CLValueBuilder.string("veCRV_1.0.0"),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_VOTING_ESCROW!,
    VOTING_ESCROW_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let votingEscrowContractHash = await utils.getContractHash(contractName);
  let votingEscrowPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "votingEscrowContractHash",
    votingEscrowContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "votingEscrowPackageHash",
    votingEscrowPackageHash
  );

  console.log("Voting Escrow deployed and saved");

  // --- Fee Distributor --- //
  console.log("Fee distributor being deployed...");

  contractName = FEE_DISTRIBUTOR + version;
  runtimeArgs = RuntimeArgs.fromMap({
    voting_escrow: CLValueBuilder.key(
      new CLByteArray(
        Uint8Array.from(Buffer.from(votingEscrowPackageHash, "hex"))
      )
    ),
    start_time: CLValueBuilder.u256("100000000"),
    token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    emergency_return: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_FEE_DISTRIBUTOR!,
    FEE_DISTRIBUTOR_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let feeDistributorContractHash = await utils.getContractHash(contractName);
  let feeDistributorPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "feeDistributorContractHash",
    feeDistributorContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "feeDistributorPackageHash",
    feeDistributorPackageHash
  );

  console.log("Fee Distributor deployed and saved");

  // --- Gauge Controller --- //

  console.log("Gauge Controller being deployed...");

  contractName = GAUGE_CONTROLLER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    voting_escrow: CLValueBuilder.key(
      new CLByteArray(
        Uint8Array.from(Buffer.from(votingEscrowPackageHash, "hex"))
      )
    ),
    token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_GAUGE_CONTROLLER!,
    GAUGE_CONTROLLER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let gaugeControllerContractHash = await utils.getContractHash(contractName);
  let gaugeControllerPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "gaugeControllerContractHash",
    gaugeControllerContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "gaugeControllerPackageHash",
    gaugeControllerPackageHash
  );

  console.log("Gauge Controller deployed and saved");

  // --- Gauge Proxy --- //

  console.log("Gauge Proxy being deployed...");

  contractName = GAUGE_PROXY + version;
  runtimeArgs = RuntimeArgs.fromMap({
    ownership_admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    emergency_admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_GAUGE_PROXY!,
    GAUGE_PROXY_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let gaugeProxyContractHash = await utils.getContractHash(contractName);
  let gaugeProxyPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "gaugeProxyContractHash",
    gaugeProxyContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "gaugeProxyPackageHash",
    gaugeProxyPackageHash
  );

  console.log("Gauge Proxy deployed and saved");

  // --- Minter --- //

  console.log("Minter being deployed...");

  contractName = MINTER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    controller: CLValueBuilder.key(
      new CLByteArray(
        Uint8Array.from(Buffer.from(gaugeControllerPackageHash, "hex"))
      )
    ),
    token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(erc20CrvPackageHash, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(KEYS, PAYMENT_MINTER!, MINTER_WASM!, runtimeArgs);
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let minterContractHash = await utils.getContractHash(contractName);
  let minterPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "minterContractHash", minterContractHash);
  utils.writeHashToFile(BASE_PATH + "minterPackageHash", minterPackageHash);

  console.log("Minter deployed and saved");

  // --- LIQUIDITY Gauge V3 --- //

  console.log("liquidity gauge v3 being deployed...");

  contractName = LIQUIDITY_GAUGE_V3 + version;
  runtimeArgs = RuntimeArgs.fromMap({
    lp_addr: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    minter: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(minterPackageHash, "hex")))
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIQUIDITY_GAUGE_V3!,
    LIQUIDITY_GAUGE_V3_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let liquidityGaugeV3ContractHash = await utils.getContractHash(contractName);
  let liquidityGaugeV3PackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeV3ContractHash",
    liquidityGaugeV3ContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeV3PackageHash",
    liquidityGaugeV3PackageHash
  );

  console.log("liquidity gauge v3 deployed and saved");

  // --- Liquidity Gauge Reward --- //

  console.log("Liquidity gauge reward being deployed...");

  contractName = LIQUIDITY_GAUGE_REWARD + version;
  runtimeArgs = RuntimeArgs.fromMap({
    lp_addr: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    minter: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(minterPackageHash, "hex")))
    ),
    reward_contract: CLValueBuilder.key(
      new CLByteArray(
        Uint8Array.from(Buffer.from(curveRewardsPackageHash, "hex"))
      )
    ),
    rewarded_token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(rewardPackageHash, "hex")))
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIQUIDITY_GAUGE_REWARD!,
    LIQUIDITY_GAUGE_REWARD_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let liquidityGaugeRewardContractHash = await utils.getContractHash(
    contractName
  );
  let liquidityGaugeRewardPackageHash = await utils.getPackageHash(
    contractName
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeRewardContractHash",
    liquidityGaugeRewardContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeRewardPackageHash",
    liquidityGaugeRewardPackageHash
  );

  console.log("Liquidity Gauge Reward deployed and saved");

  // --- Liquidity Gauge Reward Wrapper --- //

  console.log("Liquidity Gauge Reward Wrapper being deployed...");

  contractName = LIQUIDITY_GAUGE_REWARD_WRAPPER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("LiquidityGaugeRewardWrapper"),
    symbol: CLValueBuilder.string("LGRW"),
    gauge: CLValueBuilder.key(
      new CLByteArray(
        Uint8Array.from(Buffer.from(liquidityGaugeRewardPackageHash, "hex"))
      )
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIQUIDITY_GAUGE_REWARD_WRAPPER!,
    LIQUIDITY_GAUGE_REWARD_WRAPPER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let liquidityGaugeRewardWrapperContractHash = await utils.getContractHash(
    contractName
  );
  let liquidityGaugeRewardWrapperPackageHash = await utils.getPackageHash(
    contractName
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeRewardWrapperContractHash",
    liquidityGaugeRewardWrapperContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeRewardWrapperPackageHash",
    liquidityGaugeRewardWrapperPackageHash
  );

  console.log("Liquidity Gauge Reward Wrapper deployed and saved");

  // --- Liquidity Gauge Wrapper --- //

  console.log("Liquidity Gauge Wrapper being deployed...");

  contractName = LIQUIDITY_GAUGE_WRAPPER + version;
  runtimeArgs = RuntimeArgs.fromMap({
    name: CLValueBuilder.string("LiquidityGaugeWrapper"),
    symbol: CLValueBuilder.string("LGW"),
    gauge: CLValueBuilder.key(
      new CLByteArray(
        Uint8Array.from(Buffer.from(liquidityGaugeV3PackageHash, "hex"))
      )
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_LIQUIDITY_GAUGE_WRAPPER!,
    LIQUIDITY_GAUGE_WRAPPER_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let liquidityGaugeWrapperContractHash = await utils.getContractHash(
    contractName
  );
  let liquidityGaugeWrapperPackageHash = await utils.getPackageHash(
    contractName
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeWrapperContractHash",
    liquidityGaugeWrapperContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "liquidityGaugeWrapperPackageHash",
    liquidityGaugeWrapperPackageHash
  );

  console.log("Liquidity Gauge Wrapper deployed and saved");

  // --- Reward Only Gauge --- //

  console.log("Reward Only Gauge being deployed...");

  contractName = REWARD_ONLY_GAUGE + version;
  runtimeArgs = RuntimeArgs.fromMap({
    lp_token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_REWARD_ONLY_GAUGE!,
    REWARD_ONLY_GAUGE_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let rewardOnlyGaugeContractHash = await utils.getContractHash(contractName);
  let rewardOnlyGaugePackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "rewardOnlyGaugeContractHash",
    rewardOnlyGaugeContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "rewardOnlyGaugePackageHash",
    rewardOnlyGaugePackageHash
  );

  console.log("Reward Only Gauge deployed and saved");

  // --- Ownable --- //

  console.log("Ownable being deployed...");

  contractName = OWNABLE + version;
  runtimeArgs = RuntimeArgs.fromMap({
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_OWNABLE!,
    OWNABLE_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let ownableContractHash = await utils.getContractHash(contractName);
  let ownablePackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(BASE_PATH + "ownableContractHash", ownableContractHash);
  utils.writeHashToFile(BASE_PATH + "ownablePackageHash", ownablePackageHash);

  console.log("Ownable deployed and saved");

  // --- I REWARD DISTRIBUTION RECIPIENT --- //

  console.log("I Reward Distribution Recipient being deployed...");

  contractName = I_REWARD_DISTRIBUTION_RECIPIENT + version;
  runtimeArgs = RuntimeArgs.fromMap({
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_I_REWARD_DISTRIBUTION_RECIPIENT!,
    I_REWARD_DISTRIBUTION_RECIPIENT_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let iRewardDistributionRecipientContractHash = await utils.getContractHash(
    contractName
  );
  let iRewardDistributionRecipientPackageHash = await utils.getPackageHash(
    contractName
  );
  utils.writeHashToFile(
    BASE_PATH + "iRewardDistributionRecipientContractHash",
    iRewardDistributionRecipientContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "iRewardDistributionRecipientPackageHash",
    iRewardDistributionRecipientPackageHash
  );

  console.log("I Reward Distribution Recipient deployed and saved");

  // --- Vesting Escrow --- //

  // incase we need to manually resume, comment the above out and manually paste in your erc20 token package hash here (from the hashes directory)
  // tokenPackageHash = "792fda2ac6db16cfe995f3a8b6001fdc9590dfa577c3a99a2eea56b37bc7ef52"

  console.log("Vesting Escrow being deployed...");

  contractName = VESTING_ESCROW + version;
  runtimeArgs = RuntimeArgs.fromMap({
    token: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    start_time: CLValueBuilder.u256(Date.now()),
    end_time: CLValueBuilder.u256(Date.now() + 86_400_000 * 365),
    can_disable: CLValueBuilder.bool(true),
    fund_admins: CLValueBuilder.list([
      CLValueBuilder.string(USER_ADDRESS_FRM_STR!),
    ]),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_VESTING_ESCROW!,
    VESTING_ESCROW_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let vestingEscrowContractHash = await utils.getContractHash(contractName);
  let vestingEscrowPackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "vestingEscrowContractHash",
    vestingEscrowContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "vestingEscrowPackageHash",
    vestingEscrowPackageHash
  );

  console.log("Vesting Escrow deployed and saved");

  // --- Vesting Escrow Factory --- //

  console.log("Vesting Escrow Factory being deployed...");

  contractName = VESTING_ESCROW_FACTORY + version;
  runtimeArgs = RuntimeArgs.fromMap({
    target: CLValueBuilder.key(
      new CLByteArray(Uint8Array.from(Buffer.from(tokenPackageHash, "hex")))
    ),
    admin: CLValueBuilder.key(
      new CLAccountHash(Uint8Array.from(Buffer.from(USER_ADDRESS!, "hex")))
    ),
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_VESTING_ESCROW_FACTORY!,
    VESTING_ESCROW_FACTORY_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let vestingEscrowFactoryContractHash = await utils.getContractHash(
    contractName
  );
  let vestingEscrowFactoryPackageHash = await utils.getPackageHash(
    contractName
  );
  utils.writeHashToFile(
    BASE_PATH + "vestingEscrowFactoryContractHash",
    vestingEscrowFactoryContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "vestingEscrowFactoryPackageHash",
    vestingEscrowFactoryPackageHash
  );

  console.log("Vesting Escrow Factory deployed and saved");

  // --- Vesting Escrow Simple --- //

  console.log("Vesting Escrow Simple being deployed...");

  contractName = VESTING_ESCROW_SIMPLE + version;
  runtimeArgs = RuntimeArgs.fromMap({
    contract_name: CLValueBuilder.string(contractName),
  });
  installHash = await install(
    KEYS,
    PAYMENT_VESTING_ESCROW_SIMPLE!,
    VESTING_ESCROW_SIMPLE_WASM!,
    runtimeArgs
  );
  await utils.getDeploy(NODE_ADDRESS!, installHash);
  let vestingEscrowSimpleContractHash = await utils.getContractHash(
    contractName
  );
  let vestingEscrowSimplePackageHash = await utils.getPackageHash(contractName);
  utils.writeHashToFile(
    BASE_PATH + "vestingEscrowSimpleContractHash",
    vestingEscrowSimpleContractHash
  );
  utils.writeHashToFile(
    BASE_PATH + "vestingEscrowSimplePackageHash",
    vestingEscrowSimplePackageHash
  );

  console.log("Vesting Escrow Simple deployed and saved");

  // UPDATE THE DEPLOYMENT COUNT (VERSION)

  utils.updateDeploymentCount();

  // --- HASHES ZIP CREATED --- //

  zip.sync.zip("hashes").compress().save("hashes.zip");

  console.log("hashes zip created successfully...");
};

deployContract();
