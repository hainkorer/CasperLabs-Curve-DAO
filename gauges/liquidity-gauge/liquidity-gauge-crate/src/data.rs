use alloc::string::ToString;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractHash, ContractPackageHash, Key, U128, U256};
use contract_utils::{get_key, set_key, Dict};

pub const TOKENLESS_PRODUCTION: U256 = U256([40, 0, 0, 0]);
pub const BOOST_WARMUP: U256 = U256([1209600, 0, 0, 0]);
pub const WEEK: U256 = U256([604800, 0, 0, 0]);

#[allow(non_snake_case)]
pub fn ZERO_ADDRESS() -> Key {
    Key::from_formatted_str(
        "hash-0000000000000000000000000000000000000000000000000000000000000000".into(),
    )
    .unwrap()
}

const MINTER: &str = "minter";
const CRV_TOKEN: &str = "crv_token";
const LP_TOKEN: &str = "lp_token";
const CONTROLLER: &str = "controller";
const VOTING_ESCROW: &str = "voting_escrow";
const TOTAL_SUPPLY: &str = "total_supply";
const FUTURE_EPOCH_TIME: &str = "future_epoch_time";
const WORKING_SUPPLY: &str = "working_supply";
const PERIOD: &str = "period";
const INFLATION_RATE: &str = "inflation_rate";
const ADMIN: &str = "admin";
const FUTURE_ADMIN: &str = "future_admin";
const IS_KILLED: &str = "is_killed";

const CONTRACT_HASH: &str = "contract_hash";
const PACKAGE_HASH: &str = "package_hash";
const LOCK: &str = "lock";

const BALANCE_OF: &str = "balance_of";
pub struct BalanceOf {
    dict: Dict,
}

impl BalanceOf {
    pub fn instance() -> BalanceOf {
        BalanceOf {
            dict: Dict::instance(BALANCE_OF),
        }
    }

    pub fn init() {
        Dict::init(BALANCE_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const APPROVED_TO_DEPOSIT: &str = "approved_to_deposit";
pub struct ApprovedToDeposit {
    dict: Dict,
}

impl ApprovedToDeposit {
    pub fn instance() -> ApprovedToDeposit {
        ApprovedToDeposit {
            dict: Dict::instance(APPROVED_TO_DEPOSIT),
        }
    }

    pub fn init() {
        Dict::init(APPROVED_TO_DEPOSIT)
    }

    pub fn get(&self, (key0, key1): (&Key, &Key)) -> bool {
        self.dict.get_by_keys((key0, key1)).unwrap_or_default()
    }

    pub fn set(&self, (key0, key1): (&Key, &Key), value: bool) {
        self.dict.set_by_keys((key0, key1), value);
    }
}

const WORKING_BALANCES: &str = "working_balances";
pub struct WorkingBalances {
    dict: Dict,
}

impl WorkingBalances {
    pub fn instance() -> WorkingBalances {
        WorkingBalances {
            dict: Dict::instance(WORKING_BALANCES),
        }
    }

    pub fn init() {
        Dict::init(WORKING_BALANCES)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const PERIOD_TIMESTAMP: &str = "period_timestamp";
pub struct PeriodTimestamp {
    dict: Dict,
}

impl PeriodTimestamp {
    pub fn instance() -> PeriodTimestamp {
        PeriodTimestamp {
            dict: Dict::instance(PERIOD_TIMESTAMP),
        }
    }

    pub fn init() {
        Dict::init(PERIOD_TIMESTAMP)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }
}

const INTEGRATE_INV_SUPPLY: &str = "integrate_inv_supply";
pub struct IntegrateInvSupply {
    dict: Dict,
}

impl IntegrateInvSupply {
    pub fn instance() -> IntegrateInvSupply {
        IntegrateInvSupply {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_INV_SUPPLY)
    }

    pub fn get(&self, key: &U256) -> U256 {
        self.dict.get(key.to_string().as_str()).unwrap_or_default()
    }

    pub fn set(&self, key: &U256, value: U256) {
        self.dict.set(key.to_string().as_str(), value);
    }
}

const INTEGRATE_INV_SUPPLY_OF: &str = "integrate_inv_supply_of";
pub struct IntegrateInvSupplyOf {
    dict: Dict,
}

impl IntegrateInvSupplyOf {
    pub fn instance() -> IntegrateInvSupplyOf {
        IntegrateInvSupplyOf {
            dict: Dict::instance(INTEGRATE_INV_SUPPLY_OF),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_INV_SUPPLY_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const INTEGRATE_CHECKPOINT_OF: &str = "integrate_checkpoint_of";
pub struct IntegrateCheckpointOf {
    dict: Dict,
}

impl IntegrateCheckpointOf {
    pub fn instance() -> IntegrateCheckpointOf {
        IntegrateCheckpointOf {
            dict: Dict::instance(INTEGRATE_CHECKPOINT_OF),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_CHECKPOINT_OF)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

const INTEGRATE_FRACTION: &str = "integrate_fraction";
pub struct IntegrateFraction {
    dict: Dict,
}

impl IntegrateFraction {
    pub fn instance() -> IntegrateFraction {
        IntegrateFraction {
            dict: Dict::instance(INTEGRATE_FRACTION),
        }
    }

    pub fn init() {
        Dict::init(INTEGRATE_FRACTION)
    }

    pub fn get(&self, key: &Key) -> U256 {
        self.dict.get_by_key(key).unwrap_or_default()
    }

    pub fn set(&self, key: &Key, value: U256) {
        self.dict.set_by_key(key, value);
    }
}

pub fn set_minter(minter: Key) {
    set_key(MINTER, minter);
}

pub fn get_minter() -> Key {
    get_key(MINTER).unwrap_or_revert()
}

pub fn set_crv_token(crv_token: Key) {
    set_key(CRV_TOKEN, crv_token);
}

pub fn get_crv_token() -> Key {
    get_key(CRV_TOKEN).unwrap_or_revert()
}

pub fn set_lp_token(lp_token: Key) {
    set_key(LP_TOKEN, lp_token);
}

pub fn get_lp_token() -> Key {
    get_key(LP_TOKEN).unwrap_or_revert()
}

pub fn set_controller(controller: Key) {
    set_key(CONTROLLER, controller);
}

pub fn get_controller() -> Key {
    get_key(CONTROLLER).unwrap_or_revert()
}

pub fn set_voting_escrow(voting_escrow: Key) {
    set_key(VOTING_ESCROW, voting_escrow);
}

pub fn get_voting_escrow() -> Key {
    get_key(VOTING_ESCROW).unwrap_or_revert()
}

pub fn set_total_supply(total_supply: U256) {
    set_key(TOTAL_SUPPLY, total_supply);
}

pub fn get_total_supply() -> U256 {
    get_key(TOTAL_SUPPLY).unwrap_or_revert()
}

pub fn set_future_epoch_time(future_epoch_time: U256) {
    set_key(FUTURE_EPOCH_TIME, future_epoch_time);
}

pub fn get_future_epoch_time() -> U256 {
    get_key(FUTURE_EPOCH_TIME).unwrap_or_revert()
}

pub fn set_working_supply(working_supply: U256) {
    set_key(WORKING_SUPPLY, working_supply);
}

pub fn get_working_supply() -> U256 {
    get_key(WORKING_SUPPLY).unwrap_or_revert()
}

pub fn set_period(period: U128) {
    set_key(PERIOD, period);
}

pub fn get_period() -> U128 {
    get_key(PERIOD).unwrap_or_revert()
}

pub fn set_inflation_rate(inflation_rate: U256) {
    set_key(INFLATION_RATE, inflation_rate);
}

pub fn get_inflation_rate() -> U256 {
    get_key(INFLATION_RATE).unwrap_or_revert()
}

pub fn set_admin(admin: Key) {
    set_key(ADMIN, admin);
}

pub fn get_admin() -> Key {
    get_key(ADMIN).unwrap_or_revert()
}

pub fn set_future_admin(future_admin: Key) {
    set_key(FUTURE_ADMIN, future_admin);
}

pub fn get_future_admin() -> Key {
    get_key(FUTURE_ADMIN).unwrap_or_revert()
}

pub fn set_is_killed(is_killed: bool) {
    set_key(IS_KILLED, is_killed);
}

pub fn get_is_killed() -> bool {
    get_key(IS_KILLED).unwrap_or_revert()
}

pub fn set_contract_hash(contract_hash: ContractHash) {
    set_key(CONTRACT_HASH, contract_hash);
}

pub fn get_contract_hash() -> ContractHash {
    get_key(CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(PACKAGE_HASH, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(PACKAGE_HASH).unwrap_or_revert()
}

pub fn set_lock(lock: bool) {
    set_key(LOCK, lock);
}

pub fn get_lock() -> bool {
    get_key(LOCK).unwrap_or_revert()
}
