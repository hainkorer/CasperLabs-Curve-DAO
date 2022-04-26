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

pub enum MINTEREvent {
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

impl MINTEREvent {
    pub fn type_name(&self) -> String {
        match self {
            MINTEREvent::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            MINTEREvent::Transfer {
                from: _,
                to: _,
                value: _,
            } => "minter_transfer",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,536 for (UniswapV2 Core MINTER EXPIRED)
    UniswapV2CoreMINTEREXPIRED = 0,
    /// 65,537 for (UniswapV2 Core MINTER Signature Verification Failed)
    UniswapV2CoreMINTERSignatureVerificationFailed = 1,
    /// 65,538 for (UniswapV2 Core MINTER OverFlow1)
    UniswapV2CoreMINTEROverFlow1 = 2,
    /// 65,539 for (UniswapV2 Core MINTER OverFlow2)
    UniswapV2CoreMINTEROverFlow2 = 3,
    /// 65,540 for (UniswapV2 Core MINTER OverFlow3)
    UniswapV2CoreMINTEROverFlow3 = 4,
    /// 65,541 for (UniswapV2 Core MINTER OverFlow4)
    UniswapV2CoreMINTEROverFlow4 = 5,
    /// 65,542 for (UniswapV2 Core MINTER UnderFlow1)
    UniswapV2CoreMINTERUnderFlow1 = 6,
    /// 65,543 for (UniswapV2 Core MINTER UnderFlow2)
    UniswapV2CoreMINTERUnderFlow2 = 7,
    /// 65,544 for (UniswapV2 Core MINTER UnderFlow3)
    UniswapV2CoreMINTERUnderFlow3 = 8,
    /// 65,545 for (UniswapV2 Core MINTER UnderFlow4)
    UniswapV2CoreMINTERUnderFlow4 = 9,
    /// 65,546 for (UniswapV2 Core MINTER UnderFlow5)
    UniswapV2CoreMINTERUnderFlow5 = 10,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait MINTER<Storage: ContractStorage>: ContractContext<Storage> {
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
                .ok_or(Error::UniswapV2CoreMINTERUnderFlow5)
                .unwrap_or_revert(),
        );
        balances.set(
            &recipient,
            recipient_balance
                .checked_add(amount)
                .ok_or(Error::UniswapV2CoreMINTEROverFlow4)
                .unwrap_or_revert(),
        );
        self.emit(&MINTEREvent::Transfer {
            from: sender,
            to: recipient,
            value: amount,
        });
        Ok(())
    }

    fn emit(&mut self, minter_event: &MINTEREvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match minter_event {
            MINTEREvent::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", minter_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            MINTEREvent::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", minter_event.type_name());
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
