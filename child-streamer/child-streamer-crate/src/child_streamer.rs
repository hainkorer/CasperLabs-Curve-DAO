use crate::alloc::string::ToString;
use crate::data::{self, Allowances, Balances, Nonces};
use alloc::collections::BTreeMap;
use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    system::mint::Error as MintError, ApiError, BlockTime, ContractHash, ContractPackageHash, Key,
    URef, U256,
};
use contract_utils::{set_key, ContractContext, ContractStorage};
use cryptoxide::ed25519;
use hex::encode;
use renvm_sig::{hash_message, keccak256};

pub enum CHILDSTREAMEREvent {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
}

impl CHILDSTREAMEREvent {
    pub fn type_name(&self) -> String {
        match self {
            CHILDSTREAMEREvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            CHILDSTREAMEREvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "child_streamer_transfer",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,536 for (UniswapV2 Core CHILDSTREAMER EXPIRED)
    UniswapV2CoreCHILDSTREAMEREXPIRED = 0,
    /// 65,537 for (UniswapV2 Core CHILDSTREAMER Signature Verification Failed)
    UniswapV2CoreCHILDSTREAMERSignatureVerificationFailed = 1,
    /// 65,538 for (UniswapV2 Core CHILDSTREAMER OverFlow1)
    UniswapV2CoreCHILDSTREAMEROverFlow1 = 2,
    /// 65,539 for (UniswapV2 Core CHILDSTREAMER OverFlow2)
    UniswapV2CoreCHILDSTREAMEROverFlow2 = 3,
    /// 65,540 for (UniswapV2 Core CHILDSTREAMER OverFlow3)
    UniswapV2CoreCHILDSTREAMEROverFlow3 = 4,
    /// 65,541 for (UniswapV2 Core CHILDSTREAMER OverFlow4)
    UniswapV2CoreCHILDSTREAMEROverFlow4 = 5,
    /// 65,542 for (UniswapV2 Core CHILDSTREAMER UnderFlow1)
    UniswapV2CoreCHILDSTREAMERUnderFlow1 = 6,
    /// 65,543 for (UniswapV2 Core CHILDSTREAMER UnderFlow2)
    UniswapV2CoreCHILDSTREAMERUnderFlow2 = 7,
    /// 65,544 for (UniswapV2 Core CHILDSTREAMER UnderFlow3)
    UniswapV2CoreCHILDSTREAMERUnderFlow3 = 8,
    /// 65,545 for (UniswapV2 Core CHILDSTREAMER UnderFlow4)
    UniswapV2CoreCHILDSTREAMERUnderFlow4 = 9,
    /// 65,546 for (UniswapV2 Core CHILDSTREAMER UnderFlow5)
    UniswapV2CoreCHILDSTREAMERUnderFlow5 = 10,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait CHILDSTREAMER<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        name: String,
        owner: Key,
        reward_receiver: Key,
        reward_count: U256,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        data::set_name(name);
        data::set_owner(owner);
        data::set_reward_count(reward_count);
        data::set_reward_receiver(reward_receiver);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        Nonces::init();
        let nonces = Nonces::instance();
        nonces.set(&Key::from(self.get_caller()), U256::from(0));
        Allowances::init();
        Balances::init();
    }

    fn transfer(&mut self, recipient: Key, amount: U256) -> Result<(), u32> {
        self.make_transfer(self.get_caller(), recipient, amount)
    }

    fn make_transfer(&mut self, sender: Key, recipient: Key, amount: U256) -> Result<(), u32> {
        if sender == recipient {
            return Err(4); // Same sender recipient error
        }

        if amount.is_zero() {
            return Err(5); // Amount to transfer is 0
        }

        let balances: Balances = Balances::instance();
        let sender_balance: U256 = balances.get(&sender);
        let recipient_balance: U256 = balances.get(&recipient);
        balances.set(
            &sender,
            sender_balance
                .checked_sub(amount)
                .ok_or(Error::UniswapV2CoreCHILDSTREAMERUnderFlow5)
                .unwrap_or_revert(),
        );
        balances.set(
            &recipient,
            recipient_balance
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreCHILDSTREAMEROverFlow4)
                .unwrap_or_revert(),
        );
        self.emit(&CHILDSTREAMEREvent::Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
        Ok(())
    }

    fn emit(&mut self, child_streamer_event: &CHILDSTREAMEREvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match child_streamer_event {
            CHILDSTREAMEREvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", child_streamer_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            CHILDSTREAMEREvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", child_streamer_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
