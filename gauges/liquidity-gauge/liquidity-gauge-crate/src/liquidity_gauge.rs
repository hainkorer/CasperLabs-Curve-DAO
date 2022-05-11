use core::convert::TryInto;

use crate::{alloc::string::ToString, data, error::Error, event::*};

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDITYTGAUGE<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        lp_addr: Key,
        minter: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        if !(lp_addr != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::User(Error::NotZeroAddress as u16))
        }
        if !(minter != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::User(Error::NotZeroAddress as u16))
        }
        data::ApprovedToDeposit::init();
        data::BalanceOf::init();
        data::IntegrateCheckpointOf::init();
        data::IntegrateFraction::init();
        data::IntegrateInvSupply::init();
        data::IntegrateInvSupplyOf::init();
        data::PeriodTimestamp::init();
        data::WorkingBalances::init();
        data::set_lp_token(lp_addr);
        data::set_minter(minter);
        let crv_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "token",
            runtime_args! {},
        );
        data::set_crv_token(crv_addr);
        let controller_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "controller",
            runtime_args! {},
        );
        data::set_controller(controller_addr);
        data::set_voting_escrow(runtime::call_versioned_contract(
            controller_addr.into_hash().unwrap_or_revert().into(),
            None,
            "voting_escrow",
            runtime_args! {},
        ));
        let block_timestamp: u64 = runtime::get_blocktime().into();
        data::PeriodTimestamp::instance().set(&U256::from(0), block_timestamp.into());
        data::set_inflation_rate(runtime::call_versioned_contract(
            crv_addr.into_hash().unwrap_or_revert().into(),
            None,
            "rate",
            runtime_args! {},
        ));
        data::set_future_epoch_time(runtime::call_versioned_contract(
            crv_addr.into_hash().unwrap_or_revert().into(),
            None,
            "future_epoch_time_write",
            runtime_args! {},
        ));
        data::set_admin(admin);
        data::set_package_hash(package_hash);
        data::set_contract_hash(contract_hash);
        data::set_lock(false);
    }

    #[allow(non_snake_case)]
    fn _update_liquidity_limit(&self, addr: Key, l: U256, L: U256) {
        let voting_escrow: Key = data::get_voting_escrow();
        let voting_balance: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => addr
            },
        );
        let voting_total: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "total_supply",
            runtime_args! {},
        );
        let mut lim: U256 = l
            .checked_mul(data::TOKENLESS_PRODUCTION)
            .unwrap_or_revert()
            .checked_div(100.into())
            .unwrap_or_revert();
        let block_timestamp: u64 = runtime::get_blocktime().into();
        if (voting_total > 0.into())
            && (U256::from(block_timestamp)
                > data::PeriodTimestamp::instance()
                    .get(&U256::from(0))
                    .checked_add(data::BOOST_WARMUP)
                    .unwrap_or_revert())
        {
            lim = lim
                .checked_add(
                    L.checked_mul(voting_balance)
                        .unwrap_or_revert()
                        .checked_div(voting_total)
                        .unwrap_or_revert()
                        .checked_mul(
                            U256::from(100)
                                .checked_sub(data::TOKENLESS_PRODUCTION)
                                .unwrap_or_revert(),
                        )
                        .unwrap_or_revert()
                        .checked_div(100.into())
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert();
        }
        lim = U256::min(l, lim);
        let old_bal: U256 = data::WorkingBalances::instance().get(&addr);
        data::WorkingBalances::instance().set(&addr, lim);
        let working_supply: U256 = data::get_working_supply()
            .checked_add(lim)
            .unwrap_or_revert()
            .checked_sub(old_bal)
            .unwrap_or_revert();
        data::set_working_supply(working_supply);
        LIQUIDITYTGAUGE::emit(
            self,
            &LiquidityGaugeEvent::UpdateLiquidityLimit {
                user: addr,
                original_balance: l,
                original_supply: L,
                working_balance: lim,
                working_supply: working_supply,
            },
        );
    }

    fn _checkpoint(&self, addr: Key) {
        let token: Key = data::get_crv_token();
        let controller: Key = data::get_controller();
        let mut period: U128 = data::get_period();
        let period_time: U256 =
            data::PeriodTimestamp::instance().get(&U256::from(period.as_u128()));
        let mut integrate_inv_supply: U256 =
            data::IntegrateInvSupply::instance().get(&U256::from(period.as_u128()));
        let mut rate: U256 = data::get_inflation_rate();
        let mut new_rate: U256 = rate;
        let prev_future_epoch: U256 = data::get_future_epoch_time();
        if prev_future_epoch >= period_time {
            data::set_future_epoch_time(runtime::call_versioned_contract(
                token.into_hash().unwrap_or_revert().into(),
                None,
                "future_epoch_time_write",
                runtime_args! {},
            ));
            new_rate = runtime::call_versioned_contract(
                token.into_hash().unwrap_or_revert().into(),
                None,
                "rate",
                runtime_args! {},
            );
            data::set_inflation_rate(new_rate);
        }
        let () = runtime::call_versioned_contract(
            controller.into_hash().unwrap_or_revert().into(),
            None,
            "checkpoint_gauge",
            runtime_args! {
                "addr" => Key::from(data::get_package_hash())
            },
        );
        let working_balance: U256 = data::WorkingBalances::instance().get(&addr);
        let working_supply: U256 = data::get_working_supply();
        if data::get_is_killed() {
            rate = 0.into();
        }
        let block_timestamp: u64 = runtime::get_blocktime().into();
        if U256::from(block_timestamp) > period_time {
            let mut prev_week_time: U256 = period_time;
            let mut week_time: U256 = U256::min(
                (period_time.checked_add(data::WEEK).unwrap_or_revert())
                    .checked_div(data::WEEK)
                    .unwrap_or_revert()
                    .checked_mul(data::WEEK)
                    .unwrap_or_revert(),
                U256::from(block_timestamp),
            );
            for _ in 0..500 {
                let dt: U256 = week_time.checked_sub(prev_week_time).unwrap_or_revert();
                let w: U256 = runtime::call_versioned_contract(
                    controller.into_hash().unwrap_or_revert().into(),
                    None,
                    "gauge_relative_weight",
                    runtime_args! {
                        "addr" => Key::from(data::get_package_hash()),
                        "time" => prev_week_time.checked_div(data::WEEK).unwrap_or_revert().checked_mul(data::WEEK).unwrap_or_revert()
                    },
                );
                if working_supply > 0.into() {
                    if (prev_future_epoch >= prev_week_time) && (prev_future_epoch < week_time) {
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert()
                                    .checked_mul(
                                        prev_future_epoch
                                            .checked_sub(prev_week_time)
                                            .unwrap_or_revert(),
                                    )
                                    .unwrap_or_revert()
                                    .checked_div(working_supply)
                                    .unwrap_or_revert(),
                            )
                            .unwrap_or_revert();
                        rate = new_rate;
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert()
                                    .checked_mul(
                                        week_time.checked_sub(prev_future_epoch).unwrap_or_revert(),
                                    )
                                    .unwrap_or_revert()
                                    .checked_div(working_supply)
                                    .unwrap_or_revert(),
                            )
                            .unwrap_or_revert();
                    } else {
                        integrate_inv_supply = integrate_inv_supply
                            .checked_add(
                                rate.checked_mul(w)
                                    .unwrap_or_revert()
                                    .checked_mul(dt)
                                    .unwrap_or_revert()
                                    .checked_div(working_supply)
                                    .unwrap_or_revert(),
                            )
                            .unwrap_or_revert();
                    }
                }
                if week_time == block_timestamp.into() {
                    break;
                }
                prev_week_time = week_time;
                week_time = U256::min(
                    week_time.checked_add(data::WEEK).unwrap_or_revert(),
                    block_timestamp.into(),
                );
            }
        }
        period = period.checked_add(1.into()).unwrap_or_revert();
        data::set_period(period);
        data::PeriodTimestamp::instance()
            .set(&U256::from(period.as_u128()), block_timestamp.into());
        data::IntegrateInvSupply::instance()
            .set(&U256::from(period.as_u128()), integrate_inv_supply);
        data::IntegrateFraction::instance().set(
            &addr,
            data::IntegrateFraction::instance()
                .get(&addr)
                .checked_add(working_balance)
                .unwrap_or_revert()
                .checked_mul(
                    integrate_inv_supply
                        .checked_sub(data::IntegrateInvSupplyOf::instance().get(&addr))
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert()
                .checked_div(U256::from(10).pow(18.into()))
                .unwrap_or_revert(),
        );
        data::IntegrateInvSupplyOf::instance().set(&addr, integrate_inv_supply);
        data::IntegrateCheckpointOf::instance().set(&addr, block_timestamp.into());
    }

    fn user_checkpoint(&self, addr: Key) -> bool {
        if !((self.get_caller() == addr) || (self.get_caller() == data::get_minter())) {
            runtime::revert(ApiError::User(Error::InvalidCaller as u16));
        }
        self._checkpoint(addr);
        self._update_liquidity_limit(
            addr,
            data::BalanceOf::instance().get(&addr),
            data::get_total_supply(),
        );
        true
    }

    fn claimable_tokens(&self, addr: Key) -> U256 {
        self._checkpoint(addr);
        data::IntegrateFraction::instance()
            .get(&addr)
            .checked_sub(runtime::call_versioned_contract(
                data::get_minter().into_hash().unwrap_or_revert().into(),
                None,
                "minted",
                runtime_args! {
                    "user" => addr,
                    "gauge" => Key::from(data::get_package_hash())
                },
            ))
            .unwrap_or_revert()
    }

    fn kick(&self, addr: Key) {
        let voting_escrow: Key = data::get_voting_escrow();
        let t_last: U256 = data::IntegrateCheckpointOf::instance().get(&addr);
        let ret: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "addr" => addr,
            },
        );
        let t_ve: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history__ts",
            runtime_args! {
                "addr" => addr,
                "epoch" => ret
            },
        );
        let balance: U256 = data::BalanceOf::instance().get(&addr);
        let ret: U256 = runtime::call_versioned_contract(
            data::get_voting_escrow()
                .into_hash()
                .unwrap_or_revert()
                .into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => addr
            },
        );
        if !((ret == 0.into()) || (t_ve > t_last)) {
            runtime::revert(ApiError::User(Error::KickNotAllowed as u16));
        }
        if !(data::WorkingBalances::instance().get(&addr)
            > balance
                .checked_mul(data::TOKENLESS_PRODUCTION)
                .unwrap_or_revert()
                .checked_div(100.into())
                .unwrap_or_revert())
        {
            runtime::revert(ApiError::User(Error::KickNotAllowed as u16));
        }
        self._checkpoint(addr);
        self._update_liquidity_limit(
            addr,
            data::BalanceOf::instance().get(&addr),
            data::get_total_supply(),
        );
    }

    fn set_approve_deposit(&self, addr: Key, can_deposit: bool) {
        data::ApprovedToDeposit::instance().set((&addr, &self.get_caller()), can_deposit);
    }

    fn deposit(&self, value: U256, addr: Key) {
        // Non Reentrant Locking System
        if data::get_lock() {
            runtime::revert(ApiError::User(Error::Locked as u16));
        }
        data::set_lock(true);
        if addr != self.get_caller() {
            if !(data::ApprovedToDeposit::instance().get((&self.get_caller(), &addr))) {
                runtime::revert(ApiError::User(Error::NotApproved as u16));
            }
        }
        self._checkpoint(addr);
        if value != 0.into() {
            let balance: U256 = data::BalanceOf::instance()
                .get(&addr)
                .checked_add(value)
                .unwrap_or_revert();
            let supply: U256 = data::get_total_supply()
                .checked_add(value)
                .unwrap_or_revert();
            data::BalanceOf::instance().set(&addr, balance);
            data::set_total_supply(supply);
            self._update_liquidity_limit(addr, balance, supply);
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                data::get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => self.get_caller(),
                    "recipient" => Key::from(data::get_package_hash()),
                    "amount" => value
                },
            );
            if ret.is_err() {
                runtime::revert(ApiError::User(
                    ret.err().unwrap_or_revert().try_into().unwrap(),
                ));
            }
        }
        LIQUIDITYTGAUGE::emit(
            self,
            &LiquidityGaugeEvent::Deposit {
                provider: addr,
                value: value,
            },
        );
        data::set_lock(false);
    }

    fn withdraw(&self, value: U256) {
        // Non Reentrant Locking System
        if data::get_lock() {
            runtime::revert(ApiError::User(Error::Locked as u16));
        }
        data::set_lock(true);
        self._checkpoint(self.get_caller());
        let balance: U256 = data::BalanceOf::instance()
            .get(&self.get_caller())
            .checked_sub(value)
            .unwrap_or_revert();
        let supply: U256 = data::get_total_supply()
            .checked_sub(value)
            .unwrap_or_revert();
        data::BalanceOf::instance().set(&self.get_caller(), balance);
        data::set_total_supply(supply);
        self._update_liquidity_limit(self.get_caller(), balance, supply);
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            data::get_lp_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => self.get_caller(),
                "amount" => value
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::User(
                ret.err().unwrap_or_revert().try_into().unwrap(),
            ));
        }
        LIQUIDITYTGAUGE::emit(
            self,
            &LiquidityGaugeEvent::Withdraw {
                provider: self.get_caller(),
                value: value,
            },
        );
        data::set_lock(false);
    }

    fn integrate_checkpoint(&self) -> U256 {
        data::PeriodTimestamp::instance().get(&data::get_period().as_u128().into())
    }

    fn kill_me(&self) {
        if !(self.get_caller() == data::get_admin()) {
            runtime::revert(ApiError::User(Error::AdminOnly as u16));
        }
        data::set_is_killed(!data::get_is_killed());
    }

    fn commit_transfer_ownership(&self, addr: Key) {
        if !(self.get_caller() == data::get_admin()) {
            runtime::revert(ApiError::User(Error::AdminOnly as u16));
        }
        data::set_future_admin(addr);
        LIQUIDITYTGAUGE::emit(self, &LiquidityGaugeEvent::CommitOwnership { admin: addr });
    }

    fn apply_transfer_ownership(&self) {
        if !(self.get_caller() == data::get_admin()) {
            runtime::revert(ApiError::User(Error::AdminOnly as u16));
        }
        let admin: Key = data::get_future_admin();
        if !(admin != data::ZERO_ADDRESS()) {
            runtime::revert(ApiError::User(Error::AdminNotSet as u16));
        }
        data::set_admin(admin);
        LIQUIDITYTGAUGE::emit(self, &LiquidityGaugeEvent::ApplyOwnership { admin: admin });
    }

    fn emit(&self, liquidity_gauge_event: &LiquidityGaugeEvent) {
        let mut events = Vec::new();
        let tmp = data::get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match liquidity_gauge_event {
            LiquidityGaugeEvent::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeEvent::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeEvent::UpdateLiquidityLimit {
                user,
                original_balance,
                original_supply,
                working_balance,
                working_supply,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("user", user.to_string());
                event.insert("original_balance", original_balance.to_string());
                event.insert("original_supply", original_supply.to_string());
                event.insert("working_balance", working_balance.to_string());
                event.insert("working_supply", working_supply.to_string());
                events.push(event);
            }
            LiquidityGaugeEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            LiquidityGaugeEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
