use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use casperlabs_test_env::{TestContract, TestEnv};

pub struct LIQUIDITYGAUGEREWARDInstance(TestContract);
// //#[clippy::must_use]
#[allow(clippy::too_many_arguments)]
impl LIQUIDITYGAUGEREWARDInstance {
    pub fn new_deploy(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        lp_addr: Key,
        minter: Key,
        reward_contract: Key,
        rewarded_token: Key,
        admin: Key,
    ) -> LIQUIDITYGAUGEREWARDInstance {
        LIQUIDITYGAUGEREWARDInstance(TestContract::new(
            env,
            "liquidity-gauge-reward.wasm",
            contract_name,
            sender,
            runtime_args! {
                "lp_addr" => lp_addr,
                "minter" => minter,
                "reward_contract" => reward_contract,
                "rewarded_token" => rewarded_token,
                "admin" => admin,
            },
            0,
        ))
    }

    pub fn user_checkpoint(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "user_checkpoint",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn claimable_tokens(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "claimable_tokens",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn claimable_reward(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "claimable_reward",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn kick(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "kick",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn set_approve_deposit(&self, owner: AccountHash, addr: Key, can_deposit: bool) {
        self.0.call_contract(
            owner,
            "set_approve_deposit",
            runtime_args! {
                "addr" => addr,
                "can_deposit" => can_deposit
            },
            0,
        );
    }

    pub fn deposit(&self, owner: AccountHash, addr: Option<Key>, value: U256) {
        self.0.call_contract(
            owner,
            "deposit",
            runtime_args! {
                "addr" => addr,
                "value" => value
            },
            0,
        );
    }

    pub fn withdraw(&self, owner: AccountHash, claim_rewards: bool, value: U256) {
        self.0.call_contract(
            owner,
            "withdraw",
            runtime_args! {
                "claim_rewards" => claim_rewards,
                "value" => value
            },
            0,
        );
    }

    pub fn claim_rewards(&self, owner: AccountHash, addr: Option<Key>) {
        self.0.call_contract(
            owner,
            "claim_rewards",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn integrate_checkpoint(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "integrate_checkpoint", runtime_args! {}, 0);
    }

    pub fn kill_me(&self, owner: AccountHash) {
        self.0.call_contract(owner, "kill_me", runtime_args! {}, 0);
    }

    pub fn commit_transfer_ownership(&self, owner: AccountHash, addr: Key) {
        self.0.call_contract(
            owner,
            "commit_transfer_ownership",
            runtime_args! {
                "addr" => addr
            },
            0,
        );
    }

    pub fn apply_transfer_ownership(&self, owner: AccountHash) {
        self.0
            .call_contract(owner, "apply_transfer_ownership", runtime_args! {}, 0);
    }

    pub fn toggle_external_rewards_claim(&self, owner: AccountHash, val: bool) {
        self.0.call_contract(
            owner,
            "toggle_external_rewards_claim",
            runtime_args! {
                "val" => val
            },
            0,
        );
    }

    // Get stored key values
    pub fn package_hash(&self) -> [u8; 32] {
        self.0.package_hash()
    }

    pub fn key_value<T: CLTyped + FromBytes>(&self, key: String) -> T {
        self.0.query_named_key(key)
    }
}
