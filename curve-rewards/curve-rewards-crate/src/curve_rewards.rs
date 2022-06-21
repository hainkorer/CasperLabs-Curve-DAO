use crate::alloc::string::ToString;
use crate::{data::*, event::CurveRewardsEvent};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use common::errors::*;
use contract_utils::{ContractContext, ContractStorage};
use i_reward_distribution_recipient_crate::{
    self, data as reward_distribution, IREWARDDISTRIBUTIONRECIPIENT,
};
use lp_token_wrapper_crate::{self, data as lp_data, LPTOKENWRAPPER};

pub trait CURVEREWARDS<Storage: ContractStorage>:
    ContractContext<Storage> + LPTOKENWRAPPER<Storage> + IREWARDDISTRIBUTIONRECIPIENT<Storage>
{
    fn init(
        &mut self,
        token: Key,
        reward: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        IREWARDDISTRIBUTIONRECIPIENT::init(self, contract_hash, package_hash);
        LPTOKENWRAPPER::init(self, token, contract_hash, package_hash);
        reward_distribution::set_reward_distribution(self.get_caller());
        set_snx(reward);
        set_hash(contract_hash);
        set_package_hash(package_hash);
        UserRewardPerTokenPaid::init();
        Rewards::init();
    }
    fn last_time_reward_applicable(&self) -> U256 {
        let blocktime: u64 = runtime::get_blocktime().into();
        return U256::min(U256::from(blocktime), get_period_finish());
    }
    fn reward_per_token(&self) -> U256 {
        if lp_data::get_total_supply() == 0.into() {
            return get_reward_per_token_stored();
        }
        return get_reward_per_token_stored()
            .checked_add(
                self.last_time_reward_applicable()
                    .checked_sub(get_last_update_time())
                    .unwrap_or_revert_with(Error::CurveRewardsSubtractionError1)
                    .checked_mul(get_reward_rate())
                    .unwrap_or_revert_with(Error::CurveRewardsMultiplyError1)
                    .checked_mul(U256::from(TEN_E_NINE))
                    .unwrap_or_revert_with(Error::CurveRewardsMultiplyError2)
                    .checked_div(lp_data::get_total_supply())
                    .unwrap_or_revert_with(Error::CurveRewardsDivisionError1),
            )
            .unwrap_or_revert_with(Error::CurveRewardsAdditionError1);
    }
    fn earned(&self, account: Key) -> U256 {
        return LPTOKENWRAPPER::balance_of(self, account)
            .checked_mul(
                self.reward_per_token()
                    .checked_sub(UserRewardPerTokenPaid::instance().get(&account))
                    .unwrap_or_revert_with(Error::CurveRewardsSubtractionError2),
            )
            .unwrap_or_revert_with(Error::CurveRewardsMultiplyError3)
            .checked_div(U256::from(TEN_E_NINE))
            .unwrap_or_revert_with(Error::CurveRewardsDivisionError2)
            .checked_add(Rewards::instance().get(&account))
            .unwrap_or_revert_with(Error::CurveRewardsAdditionError2);
    }
    fn stake(&mut self, amount: U256) {
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CurveRewardsCannotStake));
        }
        LPTOKENWRAPPER::stake(self, amount);
        CURVEREWARDS::emit(
            self,
            &CurveRewardsEvent::Staked {
                user: self.get_caller(),
                amount: amount,
            },
        );
    }
    fn withdraw(&mut self, amount: U256) {
        self.update_reward(self.get_caller());
        if !(amount > 0.into()) {
            runtime::revert(ApiError::from(Error::CurveRewardsCannotWithdraw));
        }
        LPTOKENWRAPPER::withdraw(self, amount);
        CURVEREWARDS::emit(
            self,
            &CurveRewardsEvent::Withdrawn {
                user: self.get_caller(),
                amount: amount,
            },
        );
    }
    fn get_reward(&mut self) {
        self.update_reward(self.get_caller());
        let reward: U256 = self.earned(self.get_caller());
        if reward > 0.into() {
            Rewards::instance().set(&self.get_caller(), 0.into());
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                get_snx().into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "recipient" => self.get_caller(),
                    "amount" => reward
                },
            );
            CURVEREWARDS::emit(
                self,
                &CurveRewardsEvent::RewardPaid {
                    user: self.get_caller(),
                    reward: reward,
                },
            );
        }
    }
    fn exit(&mut self) {
        CURVEREWARDS::withdraw(self, LPTOKENWRAPPER::balance_of(self, self.get_caller()));
        self.get_reward();
    }
    fn notify_reward_amount(&mut self, reward: U256) {
        IREWARDDISTRIBUTIONRECIPIENT::only_reward_distribution(self);
        self.update_reward(zero_address());
        let blocktime: u64 = runtime::get_blocktime().into();
        if U256::from(blocktime) >= get_period_finish() {
            set_reward_rate(
                reward
                    .checked_div(DURATION)
                    .unwrap_or_revert_with(Error::CurveRewardsDivisionError3),
            );
        } else {
            let remaining: U256 = get_period_finish()
                .checked_sub(U256::from(blocktime))
                .unwrap_or_revert_with(Error::CurveRewardsSubtractionError3);
            let left_over: U256 = remaining
                .checked_mul(get_reward_rate())
                .unwrap_or_revert_with(Error::CurveRewardsMultiplyError4);
            set_reward_rate(
                reward
                    .checked_add(left_over)
                    .unwrap_or_revert_with(Error::CurveRewardsAdditionError3)
                    .checked_div(DURATION)
                    .unwrap_or_revert_with(Error::CurveRewardsDivisionError4),
            );
        }
        set_last_update_time(U256::from(blocktime));
        set_period_finish(
            U256::from(blocktime)
                .checked_add(DURATION)
                .unwrap_or_revert_with(Error::CurveRewardsAdditionError4),
        );
        CURVEREWARDS::emit(self, &CurveRewardsEvent::RewardAdded { reward: reward });
    }

    fn update_reward(&self, account: Key) {
        set_reward_per_token_stored(self.reward_per_token());
        set_last_update_time(self.last_time_reward_applicable());
        if account != zero_address() {
            Rewards::instance().set(&account, self.earned(account));
            UserRewardPerTokenPaid::instance().set(&account, get_reward_per_token_stored());
        }
    }

    fn emit(&mut self, curve_rewards_event: &CurveRewardsEvent) {
        let mut events = Vec::new();
        let package = get_package_hash();
        match curve_rewards_event {
            CurveRewardsEvent::RewardAdded { reward } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_rewards_event.type_name());
                event.insert("reward", reward.to_string());
                events.push(event);
            }
            CurveRewardsEvent::Staked { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            CurveRewardsEvent::Withdrawn { user, amount } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("amount", amount.to_string());
                events.push(event);
            }
            CurveRewardsEvent::RewardPaid { user, reward } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", curve_rewards_event.type_name());
                event.insert("user", user.to_string());
                event.insert("reward", reward.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
