use crate::data::{self};

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use casperlabs_erc20::{self, data as erc20_data, ERC20};
use common::{errors::*, utils::*};

pub enum Erc20CrvEvent {
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    UpdateMiningParameters {
        time: U256,
        rate: U256,
        supply: U256,
    },
    SetMinter {
        minter: Key,
    },
    SetAdmin {
        admin: Key,
    },
}

impl Erc20CrvEvent {
    pub fn type_name(&self) -> String {
        match self {
            Erc20CrvEvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "transfer",
            Erc20CrvEvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approval",
            Erc20CrvEvent::UpdateMiningParameters {
                time: _,
                rate: _,
                supply: _,
            } => "update_mining_parameters",
            Erc20CrvEvent::SetMinter { minter: _ } => "set_minter",
            Erc20CrvEvent::SetAdmin { admin: _ } => "set_admin",
        }
        .to_string()
    }
}
pub trait ERC20CRV<Storage: ContractStorage>: ContractContext<Storage> + ERC20<Storage> {
    fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        let base: i32 = 10;
        data::set_init_supply(data::INITIAL_SUPPLY * (base.pow(u32::from(decimals))));
        data::set_is_updated(false);
        data::set_admin(self.get_caller());
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);

        ERC20::init(
            self,
            name,
            symbol,
            decimals,
            U256::from(0),
            "".to_string(),
            "".to_string(),
            data::get_hash(),
            data::get_package_hash(),
        );
        erc20_data::Balances::instance().set(&self.get_caller(), data::get_init_supply());
        erc20_data::set_total_supply(data::get_init_supply());
        self.erc20_crv_emit(&Erc20CrvEvent::Transfer {
            from: zero_address(),
            to: self.get_caller(),
            value: data::get_init_supply(),
        });
        let blocktime_u64: u64 = runtime::get_blocktime().into();
        let blocktime: U256 = U256::from(blocktime_u64);
        let start_epoch_time: U256 = blocktime
            .checked_add(data::INFLATION_DELAY)
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow8)
            .checked_sub(data::RATE_REDUCTION_TIME)
            .unwrap_or_revert_with(Error::Erc20CRVUnderFlow3);
        data::set_start_epoch_time(start_epoch_time);
        data::set_mining_epoch(0.into());
        data::set_rate(0.into());
        data::set_start_epoch_supply(data::get_init_supply());
    }
    /// @dev Update mining rate and supply at the start of the epoch
    /// Any modifying mining call must also call this
    fn _update_mining_parameters(&self) {
        let mut rate: U256 = data::get_rate();
        let mut start_epoch_supply = data::get_start_epoch_supply();
        data::set_start_epoch_time(
            data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow9),
        );

        if data::get_is_updated() {
            data::set_mining_epoch(
                data::get_mining_epoch()
                    .checked_add(1.into())
                    .unwrap_or_revert_with(Error::Erc20CRVOverFlow10),
            );
        } else {
            data::set_is_updated(true);
        }

        if rate == 0.into() {
            rate = data::INITIAL_RATE;
        } else {
            start_epoch_supply = start_epoch_supply
                .checked_add(
                    rate.checked_mul(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError4);
            data::set_start_epoch_supply(start_epoch_supply);
            rate = (rate.checked_mul(data::RATE_DENOMINATOR).unwrap_or_revert())
                .checked_div(data::RATE_REDUCTION_COEFFICIENT)
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError5);
        }
        data::set_rate(rate);
        let blocktime: u64 = runtime::get_blocktime().into();
        self.erc20_crv_emit(&Erc20CrvEvent::UpdateMiningParameters {
            time: U256::from(blocktime),
            rate,
            supply: data::get_start_epoch_supply(),
        });
    }

    /// @notice Update mining rate and supply at the start of the epoch
    /// @dev Callable by any address, but only once per epochTotal supply becomes slightly larger if this function is called late
    /// Total supply becomes slightly larger if this function is called late
    fn update_mining_parameters(&self) {
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            < data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow11)
        {
            runtime::revert(ApiError::from(Error::Erc20CRVTooSoon));
        }
        self._update_mining_parameters();
    }
    ///@notice Get timestamp of the current mining epoch start
    ///        while simultaneously updating mining parameters
    ///@return Timestamp of the epoch
    fn start_epoch_time_write(&self) -> U256 {
        let start_epoch_time = data::get_start_epoch_time();
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            >= data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow12)
        {
            self._update_mining_parameters();
            data::get_start_epoch_time()
        } else {
            start_epoch_time
        }
    }
    ///@notice Get timestamp of the next mining epoch start
    ///        while simultaneously updating mining parameters
    ///@return Timestamp of the next epoch
    fn future_epoch_time_write(&self) -> U256 {
        let start_epoch_time = data::get_start_epoch_time();
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            >= data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow13)
        {
            self._update_mining_parameters();
            data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow14)
        } else {
            start_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow15)
        }
    }

    fn _available_supply(&self) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        let blocktime_sub_st_epoch: U256 = U256::from(blocktime)
            .checked_sub(data::get_start_epoch_time())
            .unwrap_or_revert_with(Error::Erc20CRVUnderFlow4);
        let ans: U256 = blocktime_sub_st_epoch
            .checked_mul(data::get_rate())
            .unwrap_or_revert();
        data::get_start_epoch_supply()
            .checked_add(ans)
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow16)
    }
    ///@notice Current number of tokens in existence (claimed or unclaimed)
    fn available_supply(&self) -> U256 {
        self._available_supply()
    }
    ///@notice How much supply is mintable from start timestamp till end timestamp
    ///@param start Start of the time interval (timestamp)
    ///@param end End of the time interval (timestamp)
    ///@return Tokens mintable from `start` till `end`
    fn mintable_in_timeframe(&self, start: U256, end: U256) -> U256 {
        if start > end {
            runtime::revert(ApiError::from(Error::Erc20CRVStartGreaterThanEnd));
        }
        let mut to_mint: U256 = 0.into();
        let mut current_epoch_time: U256 = data::get_start_epoch_time();
        let mut current_rate: U256 = data::get_rate();
        if end
            > current_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow1)
        {
            current_epoch_time = current_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow2);
            current_rate = current_rate
                .checked_mul(
                    data::RATE_DENOMINATOR
                        .checked_div(data::RATE_REDUCTION_COEFFICIENT)
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError1);
        }
        if end
            > current_epoch_time
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow3)
        {
            runtime::revert(ApiError::from(Error::Erc20CRVTooFarInFuture));
        }
        let mut current_end: U256;
        let mut current_start: U256;

        for _i in 0..999 {
            if end >= current_epoch_time {
                current_end = end;
                if current_end
                    > current_epoch_time
                        .checked_add(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow4)
                {
                    current_end = current_epoch_time
                        .checked_add(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow5);
                }
                current_start = start;
                if current_start
                    >= current_epoch_time
                        .checked_add(data::RATE_REDUCTION_TIME)
                        .unwrap_or_revert_with(Error::Erc20CRVOverFlow6)
                {
                    break;
                } else if current_start < current_epoch_time {
                    current_start = current_epoch_time;
                }
                let current_end_sub_current_st: U256 = current_end
                    .checked_sub(current_start)
                    .unwrap_or_revert_with(Error::Erc20CRVOverFlow7);
                to_mint = to_mint
                    .checked_add(current_rate)
                    .unwrap_or_revert()
                    .checked_mul(current_end_sub_current_st)
                    .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError2);
                if start >= current_epoch_time {
                    break;
                }
            }
            current_epoch_time = current_epoch_time
                .checked_sub(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVUnderFlow1);
            current_rate = current_rate
                .checked_mul(
                    data::RATE_REDUCTION_COEFFICIENT
                        .checked_div(data::RATE_DENOMINATOR)
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert_with(Error::Erc20CRVAirthmeticError3);
            if current_rate > data::INITIAL_RATE {
                runtime::revert(ApiError::from(Error::Erc20CRVCurrRateLessThanInitRate));
            }
        }
        to_mint
    }
    ///@notice Set the minter address
    ///@dev Only callable once, when minter has not yet been set
    ///@param minter Address of the minter
    fn set_minter(&self, minter: Key) {
        if self.get_caller() != data::get_admin() {
            runtime::revert(ApiError::from(Error::Erc20CRVInvalidMinter));
        }
        data::set_minter(minter);
        self.erc20_crv_emit(&Erc20CrvEvent::SetMinter { minter });
    }
    ///@notice Set the new admin.
    ///@dev After all is set up, admin only can change the token name
    ///@param _admin New admin address
    fn set_admin(&self, admin: Key) {
        if self.get_caller() != data::get_admin() {
            runtime::revert(ApiError::from(Error::Erc20CRVAdminOnly));
        }
        data::set_admin(admin);
        self.erc20_crv_emit(&Erc20CrvEvent::SetAdmin { admin });
    }
    ///@notice Mint `amount` tokens and assign them to `to`
    ///@dev Emits a Transfer event originating from 0x00
    ///@param _to The account that will receive the created tokens
    ///@param amount The amount that will be created
    ///@return bool success
    fn mint(&self, to: Key, amount: U256) -> bool {
        if self.get_caller() != data::get_minter() {
            runtime::revert(ApiError::from(Error::Erc20CRVMinterOnly));
        }
        if to == zero_address() {
            runtime::revert(ApiError::from(Error::Erc20CRVZeroAddress));
        }
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime)
            >= data::get_start_epoch_time()
                .checked_add(data::RATE_REDUCTION_TIME)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow17)
        {
            self._update_mining_parameters();
        }
        let total_supply: U256 = erc20_data::total_supply()
            .checked_add(amount)
            .unwrap_or_revert_with(Error::Erc20CRVOverFlow18);
        if total_supply > self.available_supply() {
            runtime::revert(ApiError::from(Error::Erc20CRVExceedsAllowableMint));
        }
        erc20_data::set_total_supply(total_supply);
        let existing_balance: U256 = erc20_data::Balances::instance().get(&to);
        erc20_data::Balances::instance().set(
            &to,
            existing_balance
                .checked_add(amount)
                .unwrap_or_revert_with(Error::Erc20CRVOverFlow19),
        );
        self.erc20_crv_emit(&Erc20CrvEvent::Transfer {
            from: zero_address(),
            to,
            value: amount,
        });

        true
    }
    ///@notice Change the token name and symbol to `name` and `symbol`
    ///@dev Only callable by the admin account
    ///@param name New token name
    ///@param symbol New token symbol
    fn set_name(&self, name: String, symbol: String) {
        if data::get_minter() != self.get_caller() {
            runtime::revert(ApiError::from(Error::Erc20CRVOnlyMinterAllowed1));
        }
        erc20_data::set_name(name);
        erc20_data::set_symbol(symbol);
    }
    ///@notice Burn `value` tokens belonging to `msg.sender`
    ///@dev Emits a Transfer event with a destination of 0x00
    ///@param value The amount that will be burned
    fn burn_caller(&mut self, value: U256) {
        ERC20::burn(self, self.get_caller(), value);
    }
    fn erc20_crv_emit(&self, erc20_crv_event: &Erc20CrvEvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match erc20_crv_event {
            Erc20CrvEvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            Erc20CrvEvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            Erc20CrvEvent::UpdateMiningParameters { time, rate, supply } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("time", time.to_string());
                event.insert("rate", rate.to_string());
                event.insert("supply", supply.to_string());
                events.push(event);
            }
            Erc20CrvEvent::SetMinter { minter } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("minter", minter.to_string());
                events.push(event);
            }
            Erc20CrvEvent::SetAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc20_crv_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
