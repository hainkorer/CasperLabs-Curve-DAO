//use crate::data::{self, Allowances, Balances, Nonces};
use crate::data;
use casper_contract::contract_api::runtime;
use casper_types::{ApiError, ContractHash, ContractPackageHash, Key};
use common::errors::*;
use contract_utils::{ContractContext, ContractStorage};
use ownable_crate::OWNABLE;
pub trait IREWARDDISTRIBUTIONRECIPIENT<Storage: ContractStorage>:
    ContractContext<Storage> + OWNABLE<Storage>
{
    fn init(&mut self, contract_hash: ContractHash, package_hash: ContractPackageHash) {
        OWNABLE::init(self, contract_hash, package_hash);
        data::set_contract_hash(contract_hash);
        data::set_package_hash(package_hash);
    }
    fn only_reward_distribution(&self) {
        if !(self.get_caller() == data::get_reward_distribution()) {
            runtime::revert(ApiError::from(Error::NotRewardDistribution));
        }
    }
    fn set_reward_distribution(&self, reward_distribution: Key) {
        OWNABLE::only_owner(self);
        data::set_reward_distribution(reward_distribution);
    }
}
