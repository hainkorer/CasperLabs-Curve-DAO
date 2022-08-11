use crate::data::{
    self, get_lp_token, get_package_hash, zero_address, Allowance, BalanceOf, ClaimData,
    ClaimDataStruct, PeriodTimestamp, RewardData, RewardIntegral, RewardIntegralFor, RewardTokens,
    RewardsReceiver, CLAIM_FREQUENCY, MAX_REWARDS,
};
use crate::{alloc::string::ToString, event::*};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::*;
pub trait LIQUIDITYTGAUGEV3<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        lp_token: Key,
        minter: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        data::BalanceOf::init();
        data::IntegrateCheckpointOf::init();
        data::IntegrateFraction::init();
        data::IntegrateInvSupply::init();
        data::IntegrateInvSupplyOf::init();
        data::PeriodTimestamp::init();
        data::WorkingBalances::init();
        data::Allowance::init();
        data::RewardTokens::init();
        ClaimData::init();
        RewardsReceiver::init();

        let _lp_token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let _lp_token_package_hash = ContractPackageHash::new(_lp_token_hash_add_array);
        let symbol: String = runtime::call_versioned_contract(
            _lp_token_package_hash,
            None,
            "symbol",
            runtime_args! {},
        );
        let mut name: String = "Curve.fi ".to_string();
        let post_name: &str = "Gauge Deposit";
        name.push_str(symbol.as_str());
        name.push_str(post_name);
        data::set_name(name);
        data::set_symbol(symbol + "-gauge");

        let crv_token: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "token",
            runtime_args! {},
        );

        let controller_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "controller",
            runtime_args! {},
        );
        data::set_lp_token(lp_token);
        data::set_minter(minter);
        data::set_admin(admin);
        data::set_crv_token(crv_token);
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
            crv_token.into_hash().unwrap_or_revert().into(),
            None,
            "rate",
            runtime_args! {},
        ));
        data::set_future_epoch_time(runtime::call_versioned_contract(
            crv_token.into_hash().unwrap_or_revert().into(),
            None,
            "future_epoch_time_write",
            runtime_args! {},
        ));

        let decimals: u8 = 9;
        let total_supply: U256 = 0.into();
        data::set_total_supply(total_supply);
        data::set_decimals(decimals);
        data::set_package_hash(package_hash);
        data::set_contract_hash(contract_hash);
        data::set_lock(false);
    }
    fn total_supply(&mut self) -> U256 {
        data::get_total_supply()
    }

    fn name(&mut self) -> String {
        data::get_name()
    }

    fn symbol(&mut self) -> String {
        data::get_symbol()
    }
    fn reward_data(&mut self) -> RewardData {
        data::reward_data()
    }
    fn lp_token(&mut self) -> Key {
        data::get_lp_token()
    }
    fn admin(&mut self) -> Key {
        data::get_admin()
    }
    fn reward_integral(&mut self, reward_token: Key) -> U256 {
        RewardIntegral::instance().get(&reward_token)
    }
    fn reward_tokens(&mut self, index: U256) -> Key {
        RewardTokens::instance().get(&index)
    }

    fn future_admin(&mut self) -> Key {
        data::get_future_admin()
    }
    fn claim_data(&mut self, user: Key, claiming_address: Key) -> ClaimDataStruct {
        ClaimData::instance().get(&user, &claiming_address)
    }
    fn balance_of(&mut self, owner: Key) -> U256 {
        BalanceOf::instance().get(&owner)
    }

    //function implementaion of liquidity gauge v3
    fn decimals(&mut self) -> u8 {
        data::get_decimals()
    }

    fn integrate_checkpoint(&self) -> U256 {
        PeriodTimestamp::instance().get(&U256::from(data::get_period().as_u128()))
    }

    fn _update_liquidity_limit(&self, addr: Key, l: U256, supply: U256) {
        let voting_escrow: Key = data::get_voting_escrow();
        let voting_balance: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "addr" => addr,
                "t" => None::<Key>
            },
        );
        let voting_total: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "total_supply",
            runtime_args! {
                "t" => None::<U256>
            },
        );
        let mut lim: U256 = l
            .checked_mul(data::TOKENLESS_PRODUCTION)
            .unwrap_or_revert()
            .checked_div(100.into())
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError1);
        let _block_timestamp: u64 = runtime::get_blocktime().into();
        if voting_total > 0.into() {
            lim = lim
                .checked_add(
                    supply
                        .checked_mul(voting_balance)
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
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError2);
        }
        lim = U256::min(l, lim);
        let old_bal: U256 = data::WorkingBalances::instance().get(&addr);
        data::WorkingBalances::instance().set(&addr, lim);
        let working_supply: U256 = data::get_working_supply()
            .checked_add(lim)
            .unwrap_or_revert()
            .checked_sub(old_bal)
            .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError3);
        data::set_working_supply(working_supply);
        self.emit(&LiquidityGaugeV3Event::UpdateLiquidityLimit {
            user: addr,
            original_balance: l,
            original_supply: supply,
            working_balance: lim,
            working_supply,
        });
    }

    fn _checkpoint_rewards(&mut self, user: Key, total_supply: U256, claim: bool, receiver: Key) {
        let mut reward_tokens: Vec<Key> = Vec::new();
        let mut reward_integrals: Vec<U256> = Vec::new();
        for i in 0..(MAX_REWARDS.as_usize()) {
            let token: Key = self.reward_tokens(i.into());
            if token == zero_address() {
                break;
            }
            reward_tokens.push(token);
            reward_integrals.push(self.reward_integral(token));
        }
        let mut reward_data: RewardData = self.reward_data();
        if total_supply != 0.into()
            && reward_data.address != zero_address()
            && reward_data.time_stamp != 0.into()
            && U256::from(u64::from(runtime::get_blocktime()))
                > (reward_data.time_stamp + U256::from(CLAIM_FREQUENCY.as_u128()))
        {
            let mut reward_balances: Vec<U256> = Vec::new();
            for i in 0..(MAX_REWARDS.as_usize()) {
                let token: Key = self.reward_tokens(i.into());
                if token == zero_address() {
                    break;
                }
                reward_balances.push(runtime::call_versioned_contract(
                    token.into_hash().unwrap_or_revert().into(),
                    None,
                    "balance_of",
                    runtime_args! {
                        "owner" => self.get_caller()
                    },
                ));
            }
            let reward_contract = reward_data.address;
            let reward_contract_hash_add_array = match reward_contract {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let reward_contract_package_hash =
                ContractPackageHash::new(reward_contract_hash_add_array);
            let () = runtime::call_versioned_contract(
                reward_contract_package_hash,
                None,
                "claim_sig",
                runtime_args! {},
            );
            reward_data.address = reward_contract;
            reward_data.time_stamp = U256::from(u64::from(runtime::get_blocktime()));

            for i in 0..(MAX_REWARDS.as_usize()) {
                let token: Key = self.reward_tokens(i.into());
                if token == zero_address() {
                    break;
                }

                let token_balance: U256 = runtime::call_versioned_contract(
                    token.into_hash().unwrap_or_revert().into(),
                    None,
                    "balance_of",
                    runtime_args! {
                        "owner" => self.get_caller()
                    },
                );

                let d_i: U256 = U256::from(1000000000)
                    .checked_mul(
                        token_balance
                            .checked_sub(reward_balances[i])
                            .unwrap_or_revert(),
                    )
                    .unwrap_or_revert()
                    .checked_div(total_supply)
                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError4);
                if d_i > 0.into() {
                    reward_integrals[i] = reward_integrals[i].checked_add(d_i).unwrap_or_revert();
                    data::RewardIntegral::instance().set(&token, reward_integrals[i]);
                }
            }
            let mut _receiver: Key = receiver;
            if user != zero_address() && claim && _receiver == zero_address() {
                _receiver = RewardsReceiver::instance().get(&user);
                if _receiver == zero_address() {
                    _receiver = user;
                }
            }
            let user_balance: U256 = BalanceOf::instance().get(&user);
            for (i, item) in reward_integrals
                .iter()
                .enumerate()
                .take(MAX_REWARDS.as_usize())
            {
                let token: Key = self.reward_tokens(i.into());
                if token == zero_address() {
                    break;
                }
                let integral = *item;
                let integral_for = RewardIntegralFor::instance().get(&token, &user);
                let mut new_claimable: U256 = 0.into();
                if integral_for < integral {
                    RewardIntegralFor::instance().set(&token, &user, integral);
                    new_claimable = user_balance
                        .checked_mul(integral.checked_sub(integral_for).unwrap_or_revert())
                        .unwrap_or_revert()
                        .checked_div(U256::from(1000000000))
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError5);
                }
                let mut claim_data: ClaimDataStruct = self.claim_data(user, token);
                let total_claimable: U256 = claim_data
                    .claimable_amount
                    .checked_add(new_claimable)
                    .ok_or(Error::LiquidityGaugeUnderFlow6)
                    .unwrap_or_revert();
                if total_claimable > 0.into() {
                    let total_claimed = claim_data.claimed_amount;
                    if claim {
                        let token_hash_add_array = match token {
                            Key::Hash(package) => package,
                            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                        };
                        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                        let () = runtime::call_versioned_contract(
                            token_package_hash,
                            None,
                            "transfer",
                            runtime_args! {"to" => _receiver,"amount" => total_claimable},
                        );
                        claim_data.claimed_amount = total_claimed
                            .checked_add(total_claimable)
                            .ok_or(Error::LiquidityGaugeOverFlow7)
                            .unwrap_or_revert();
                        ClaimData::instance().set(&user, &token, claim_data);
                    } else if new_claimable > 0.into() {
                        claim_data.claimed_amount = total_claimed;
                        claim_data.claimable_amount = total_claimable;
                        ClaimData::instance().set(&user, &token, claim_data);
                    }
                }
            }
        }
    }
    fn _checkpoint(&mut self, addr: Key) {
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
        if data::get_is_killed() {
            rate = 0.into();
        }
        let block_timestamp: u64 = runtime::get_blocktime().into();
        let mut prev_week_time: U256 = 0.into();
        let mut working_supply: U256 = 0.into();
        let mut week_time: U256 = 0.into();
        if U256::from(block_timestamp) > period_time {
            working_supply = data::get_working_supply();
            let () = runtime::call_versioned_contract(
                controller.into_hash().unwrap_or_revert().into(),
                None,
                "checkpoint_gauge",
                runtime_args! {
                    "addr" => Key::from(data::get_package_hash())
                },
            );
            prev_week_time = period_time;
            week_time = U256::min(
                (period_time.checked_add(data::WEEK).unwrap_or_revert())
                    .checked_div(data::WEEK)
                    .unwrap_or_revert()
                    .checked_mul(data::WEEK)
                    .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError7),
                U256::from(block_timestamp),
            );
        }

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
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError8);
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
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError9);
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
                        .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError10);
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
        period = period.checked_add(1.into()).unwrap_or_revert();
        data::set_period(period);
        data::PeriodTimestamp::instance()
            .set(&U256::from(period.as_u128()), block_timestamp.into());
        data::IntegrateInvSupply::instance()
            .set(&U256::from(period.as_u128()), integrate_inv_supply);
        let working_balance: U256 = data::WorkingBalances::instance().get(&addr);
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
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError11),
        );
        data::IntegrateInvSupplyOf::instance().set(&addr, integrate_inv_supply);
        data::IntegrateCheckpointOf::instance().set(&addr, block_timestamp.into());
    }
    fn user_checkpoint(&mut self, addr: Key) -> bool {
        if !(self.get_caller() == addr || self.get_caller() == data::get_minter()) {
            runtime::revert(Error::LiquidityGuageV3Unauthorized);
        }
        self._checkpoint(addr);
        self._update_liquidity_limit(
            addr,
            BalanceOf::instance().get(&addr),
            data::get_total_supply(),
        );
        true
    }

    fn claimable_tokens(&mut self, addr: Key) -> U256 {
        self._checkpoint(addr);
        data::IntegrateFraction::instance()
            .get(&addr)
            .checked_sub(runtime::call_versioned_contract(
                data::get_minter().into_hash().unwrap_or_revert().into(),
                None,
                "minted",
                runtime_args! {
                    "owner" => addr,
                    "spender" => Key::from(data::get_package_hash())
                },
            ))
            .unwrap_or_revert()
    }

    fn reward_contract(&mut self) -> Key {
        let address = self.reward_data().address;
        if address == zero_address() {
            zero_address()
        } else {
            address
        }
    }

    fn last_claim(&mut self) -> U256 {
        self.reward_data().time_stamp
    }

    fn claimed_reward(&mut self, addr: Key, token: Key) -> U256 {
        self.claim_data(addr, token).claimed_amount
    }

    fn claimable_reward(&mut self, addr: Key, token: Key) -> U256 {
        self.claim_data(addr, token).claimable_amount
    }

    fn claimable_reward_write(&mut self, addr: Key, token: Key) -> U256 {
        let lock = data::get_lock();
        if lock {
            // Locked
            runtime::revert(Error::LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        let reward_token = self.reward_tokens(0.into());
        if reward_token != zero_address() {
            let total_supply = self.total_supply();
            self._checkpoint_rewards(addr, total_supply, false, zero_address());
        }
        data::set_lock(false);
        self.claim_data(addr, token).claimable_amount
    }

    fn set_rewards_receiver(&mut self, receiver: Key) {
        RewardsReceiver::instance().set(&self.get_caller(), receiver)
    }

    fn claim_rewards(&mut self, addr: Option<Key>, receiver: Option<Key>) {
        let lock = data::get_lock();
        if lock {
            // Locked
            runtime::revert(Error::LiquidityGaugeLocked2);
        }
        data::set_lock(true);
        let _addr: Key = if let Some(..) = addr {
            addr.unwrap()
        } else {
            self.get_caller()
        };
        let _receiver: Key = if let Some(..) = receiver {
            receiver.unwrap()
        } else {
            zero_address()
        };
        if _receiver != zero_address() && _addr != self.get_caller() {
            runtime::revert(Error::LiquidityGaugeCannotRedirectWhenClaimingForAnotherUser);
        }
        let _total_supply = self.total_supply();
        self._checkpoint_rewards(_addr, _total_supply, true, _receiver);
        data::set_lock(false);
    }

    fn kick(&mut self, addr: Key) {
        let voting_escrow: Key = data::get_voting_escrow();
        let t_last: U256 = data::IntegrateCheckpointOf::instance().get(&addr);
        let ret: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "user" => addr,
            },
        );
        let t_ve: U256 = runtime::call_versioned_contract(
            voting_escrow.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history_ts",
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
            runtime::revert(ApiError::User(Error::LiquidityGuageV3KickNotAllowed as u16));
        }
        if data::WorkingBalances::instance().get(&addr)
            <= balance
                .checked_mul(data::TOKENLESS_PRODUCTION)
                .unwrap_or_revert()
                .checked_div(100.into())
                .unwrap_or_revert_with(Error::LiquidityGaugeArithmeticError12)
        {
            runtime::revert(ApiError::User(Error::LiquidityGuageV3KickNotNeeded as u16));
        }
        self._checkpoint(addr);
        self._update_liquidity_limit(
            addr,
            data::BalanceOf::instance().get(&addr),
            data::get_total_supply(),
        );
    }

    fn deposit(&mut self, value: U256, addr: Option<Key>, claim_rewards: Option<bool>) {
        let _claim_rewards: bool = if let Some(..) = claim_rewards {
            claim_rewards.unwrap()
        } else {
            false
        };
        let _addr: Key = if let Some(..) = addr {
            addr.unwrap()
        } else {
            self.get_caller()
        };

        let lock = data::get_lock();
        if lock {
            //Locked
            runtime::revert(Error::LiquidityGaugeLocked3);
        }
        data::set_lock(true);
        self._checkpoint(_addr);
        if value != 0.into() {
            let is_rewards: bool = self.reward_tokens(0.into()) != zero_address();
            let mut total_supply = self.total_supply();
            if is_rewards {
                self._checkpoint_rewards(_addr, total_supply, _claim_rewards, zero_address());
            }
            total_supply = total_supply
                .checked_add(value)
                .ok_or(Error::LiquidityGaugeOverFlow4)
                .unwrap_or_revert();
            let balance = self.balance_of(self.get_caller());
            let new_balance = balance
                .checked_add(value)
                .ok_or(Error::LiquidityGaugeOverFlow5)
                .unwrap_or_revert();
            BalanceOf::instance().set(&self.get_caller(), new_balance);
            data::set_total_supply(total_supply);
            self._update_liquidity_limit(_addr, new_balance, total_supply);
            let lp_token = self.lp_token();
            let token_hash_add_array = match lp_token {
                Key::Hash(package) => package,
                _ => runtime::revert(ApiError::UnexpectedKeyVariant),
            };
            let token_package_hash = ContractPackageHash::new(token_hash_add_array);
            let _ret: Result<(), u32> = runtime::call_versioned_contract(
                token_package_hash,
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => self.get_caller(),
                    "recipient" => Key::from(data::get_package_hash()),
                    "amount" => value
                },
            );
            if is_rewards {
                let reward_data: RewardData = self.reward_data();
                if reward_data.time_stamp > 0.into() {
                    let reward_contract = reward_data.address;
                    let () = runtime::call_versioned_contract(
                        reward_contract.into_hash().unwrap_or_revert().into(),
                        None,
                        "deposit",
                        runtime_args! {
                            "value"=>value
                        },
                    );
                }
            }
        }
        self.emit(&LiquidityGaugeV3Event::Deposit {
            provider: _addr,
            value,
        });
        self.emit(&LiquidityGaugeV3Event::Transfer {
            from: zero_address(),
            to: _addr,
            value,
        });
        data::set_lock(false);
    }
    fn withdraw(&mut self, value: U256, claim_rewards: Option<bool>) {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked7);
        }
        data::set_lock(true);
        let _claim_rewards: bool = if let Some(..) = claim_rewards {
            claim_rewards.unwrap()
        } else {
            false
        };

        self._checkpoint(self.get_caller());
        let mut _total_supply: U256 = 0.into();
        if value != 0.into() {
            let is_rewards: bool = data::RewardTokens::instance().get(&0.into()) != zero_address();
            _total_supply = data::get_total_supply();
            if is_rewards {
                self._checkpoint_rewards(
                    self.get_caller(),
                    _total_supply,
                    _claim_rewards,
                    zero_address(),
                )
            }
            _total_supply = _total_supply
                .checked_sub(value)
                .ok_or(Error::LiquidityGaugeUnderFlow10)
                .unwrap_or_revert();
            let balance = self.balance_of(self.get_caller());
            let new_balance = balance
                .checked_sub(value)
                .ok_or(Error::LiquidityGaugeUnderFlow11)
                .unwrap_or_revert();
            data::BalanceOf::instance().set(&self.get_caller(), new_balance);
            data::set_total_supply(_total_supply);
            self._update_liquidity_limit(self.get_caller(), new_balance, _total_supply);
            let mut _reward_data: U256 = 0.into();
            if is_rewards {
                _reward_data = self.reward_data().time_stamp;
                let reward_data: RewardData = self.reward_data();
                if reward_data.time_stamp > 0.into() {
                    let reward_contract = reward_data.address;
                    let () = runtime::call_versioned_contract(
                        reward_contract.into_hash().unwrap_or_revert().into(),
                        None,
                        "withdraw",
                        runtime_args! {
                            "value"=>value
                        },
                    );
                }
                let lp_token = self.lp_token();
                let token_hash_add_array = match lp_token {
                    Key::Hash(package) => package,
                    _ => runtime::revert(ApiError::UnexpectedKeyVariant),
                };
                let token_package_hash = ContractPackageHash::new(token_hash_add_array);
                let _result: () = runtime::call_versioned_contract(
                    token_package_hash,
                    None,
                    "transfer",
                    runtime_args! {"to" => self.get_caller(),"value" => value},
                );
            }
        }
        self.emit(&LiquidityGaugeV3Event::Withdraw {
            provider: self.get_caller(),
            value,
        });
        self.emit(&LiquidityGaugeV3Event::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value,
        });
        data::set_lock(false);
    }
    fn _transfer(&mut self, from: Key, to: Key, value: U256) {
        self._checkpoint(from);
        self._checkpoint(to);
        if value != 0.into() {
            let total_supply = self.total_supply();
            let is_rewards: bool = self.reward_tokens(0.into()) != zero_address();
            if is_rewards {
                self._checkpoint_rewards(from, total_supply, false, zero_address());
            }
            let balances: BalanceOf = BalanceOf::instance();
            let _from_balance: U256 = balances.get(&from);
            let from_new_balance = _from_balance
                .checked_sub(value)
                .ok_or(Error::LiquidityGaugeUnderFlow3)
                .unwrap_or_revert();
            balances.set(&from, from_new_balance);
            self._update_liquidity_limit(from, from_new_balance, total_supply);
            if is_rewards {
                self._checkpoint_rewards(to, total_supply, false, zero_address());
            }
            let balances: BalanceOf = BalanceOf::instance();
            let _to_balance: U256 = balances.get(&to);
            let to_new_balance = _from_balance
                .checked_sub(value)
                .ok_or(Error::LiquidityGaugeUnderFlow4)
                .unwrap_or_revert();
            balances.set(&to, to_new_balance);
            self._update_liquidity_limit(to, to_new_balance, total_supply);
        }
        self.emit(&LiquidityGaugeV3Event::Transfer { from, to, value });
    }

    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked4);
        }
        data::set_lock(true);
        self._transfer(self.get_caller(), recipient, amount);
        data::set_lock(false);
        Ok(())
    }
    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        let lock = data::get_lock();
        if lock {
            //Locked
            runtime::revert(Error::LiquidityGaugeLocked5);
        }
        data::set_lock(true);
        let allowances = Allowance::instance();
        let _allowance: U256 = allowances.get(&owner, &self.get_caller());

        if _allowance != U256::MAX {
            allowances.set(
                &owner,
                &self.get_caller(),
                _allowance
                    .checked_sub(amount)
                    .ok_or(Error::LiquidityGaugeUnderFlow2)
                    .unwrap_or_revert(),
            )
        }
        self._transfer(owner, recipient, amount);
        data::set_lock(false);
        Ok(())
    }

    fn approve(&self, spender: Key, amount: U256) {
        Allowance::instance().set(&self.get_caller(), &spender, amount);
        self.emit(&LiquidityGaugeV3Event::Approval {
            owner: self.get_caller(),
            spender,
            value: amount,
        });
    }
    fn increase_allowance(&self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowance::instance()
            .get(&self.get_caller(), &spender)
            .checked_add(amount)
            .ok_or(Error::LiquidityGaugeOverFlow1)
            .unwrap_or_revert();
        Allowance::instance().set(&self.get_caller(), &spender, allowance);
        self.emit(&LiquidityGaugeV3Event::Approval {
            owner: self.get_caller(),
            spender,
            value: amount,
        });
        Ok(())
    }
    fn decrease_allowance(&self, spender: Key, amount: U256) -> Result<(), u32> {
        let allowance: U256 = Allowance::instance()
            .get(&self.get_caller(), &spender)
            .checked_sub(amount)
            .ok_or(Error::LiquidityGaugeUnderFlow1)
            .unwrap_or_revert();
        Allowance::instance().set(&self.get_caller(), &spender, allowance);
        self.emit(&LiquidityGaugeV3Event::Approval {
            owner: self.get_caller(),
            spender,
            value: amount,
        });
        Ok(())
    }

    fn set_rewards(&mut self, reward_contract: Key, claim_sig: Bytes, reward_tokens: Vec<String>) {
        let lock = data::get_lock();
        if lock {
            runtime::revert(Error::LiquidityGaugeLocked6);
        }
        data::set_lock(true);
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin2);
        }
        let mut _reward_tokens: Vec<Key> = Vec::new();
        for reward_token in &reward_tokens {
            _reward_tokens.push(Key::from_formatted_str(reward_token).unwrap());
        }
        let _lp_token = self.lp_token();
        let current_reward_contract = self.reward_data().address;
        let total_supply = self.total_supply();
        if RewardTokens::instance().get(&0.into()) != zero_address() {
            self._checkpoint_rewards(zero_address(), total_supply, false, zero_address());
        }
        if current_reward_contract != zero_address() {
            if total_supply != 0.into() {
                let () = runtime::call_versioned_contract(
                    current_reward_contract
                        .into_hash()
                        .unwrap_or_revert()
                        .into(),
                    None,
                    "withdraw",
                    runtime_args! {
                        "value" => total_supply,
                        "addr" => None::<Key>
                    },
                );
            }
            let amount: U256 = 0.into();
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "approve",
                runtime_args! {"spender" => current_reward_contract,"amount" => amount},
            );
        }

        if reward_contract != zero_address() {
            let reward_token = self.reward_tokens(0.into());
            if reward_token == zero_address() {
                runtime::revert(Error::LiquidityGaugeTokenIsZeroAddress);
            }
            data::set_claim_sig(claim_sig);

            //is Contract Check is missing
            if total_supply == 0.into() {
                runtime::revert(Error::LiquidityGaugeZeroTotalSupply);
            }
            let () = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "approve",
                runtime_args! {"spender" => reward_contract,"amount" => U256::MAX},
            );
            let () = runtime::call_versioned_contract(
                reward_contract.into_hash().unwrap_or_revert().into(),
                None,
                "deposit",
                runtime_args! {
                    "value"=>total_supply
                },
            );
            let mut balance_of: U256 = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {"owner" => Key::from(get_package_hash())},
            );
            if balance_of != 0.into() {
                runtime::revert(Error::LiquidityGaugeFailedToDeposit);
            }
            let () = runtime::call_versioned_contract(
                reward_contract.into_hash().unwrap_or_revert().into(),
                None,
                "withdraw",
                runtime_args! {
                    "value" => total_supply,
                    "addr" => None::<Key>
                },
            );
            balance_of = runtime::call_versioned_contract(
                get_lp_token().into_hash().unwrap_or_revert().into(),
                None,
                "balance_of",
                runtime_args! {"owner" => Key::from(get_package_hash())},
            );
            if balance_of != total_supply {
                runtime::revert(Error::LiquidityGaugeFailedToWithdraw);
            }
            let () = runtime::call_versioned_contract(
                reward_contract.into_hash().unwrap_or_revert().into(),
                None,
                "deposit",
                runtime_args! {
                    "value"=>total_supply
                },
            );
        }
        let mut reward_data = self.reward_data();
        reward_data.address = reward_contract;
        for (i, reward_token) in _reward_tokens
            .iter()
            .enumerate()
            .take(MAX_REWARDS.as_usize())
        {
            let current_token = self.reward_tokens(i.into());
            let new_token: Key = *reward_token;

            if current_token != zero_address() {
                if current_token != new_token {
                    runtime::revert(Error::LiquidityGaugeCannotModifyExistingRewardtoken);
                }
            } else if new_token != zero_address() {
                RewardTokens::instance().set(&i.into(), new_token);
            } else {
                break;
            }
        }
        if reward_contract != zero_address() {
            // # do an initial checkpoint to verify that claims are working
            self._checkpoint_rewards(zero_address(), total_supply, false, zero_address())
        }
        data::set_lock(false);
    }

    fn set_killed(&mut self, is_killed: bool) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin1);
        }
        data::set_is_killed(is_killed);
    }

    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            runtime::revert(Error::LiquidityGaugeOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&LiquidityGaugeV3Event::CommitOwnership { admin: addr });
    }

    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            runtime::revert(Error::LiquidityGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&LiquidityGaugeV3Event::ApplyOwnership { admin: _admin });
    }

    fn emit(&self, liquidity_gauge_event: &LiquidityGaugeV3Event) {
        let mut events = Vec::new();
        let tmp = data::get_package_hash().to_formatted_string();
        let split: char = '-';
        let tmp: Vec<&str> = tmp.split(split).collect();
        let package_hash = tmp[1].to_string();
        match liquidity_gauge_event {
            LiquidityGaugeV3Event::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::UpdateLiquidityLimit {
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
            LiquidityGaugeV3Event::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::ApplyOwnership { admin } => {
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
