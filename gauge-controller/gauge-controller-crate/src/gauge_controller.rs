use crate::alloc::string::ToString;
use crate::data::{
    self, CHANGESSUM, CHANGESWEIGHT, GAUGES, GAUGETYPENAMES, GAUGETYPES_, LASTUSERVOTE, POINTSSUM,
    POINTSTOTAL, POINTSTYPEWEIGHT, POINTSWEIGHT, TIMESUM, TIMETYPEWEIGHT, TIMEWEIGHT,
    VOTEUSERPOWER, VOTEUSERSLOPES, admin, time_total
};
use alloc::collections::BTreeMap;
use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    runtime_args, system::mint::Error as MintError, ApiError, BlockTime, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, URef, U256,U128
};
use contract_utils::{set_key, ContractContext, ContractStorage};
use cryptoxide::ed25519;
use hex::encode;
use renvm_sig::{hash_message, keccak256};

pub enum GAUGECOLTROLLEREvent {
    Minted {
        recipient: Key,
        gauge: Key,
        minted: U256,
    },
}

impl GAUGECOLTROLLEREvent {
    pub fn type_name(&self) -> String {
        match self {
            GAUGECOLTROLLEREvent::Minted {
                recipient: _,
                gauge: _,
                minted: _,
            } => "minted",
        }
        .to_string()
    }
}

#[repr(u16)]
pub enum Error {
    /// 65,537 for (Gauge Controller Address Zero1)
    GaugeControllerAddressZero1 = 1,
    /// 65,538 for (Gauge Controller Address Zero2)
    GaugeControllerAddressZero2 = 2,
    /// 65,539 for (Gauge Controller Not Admin1)
    GaugeControllerNotAdmin1 = 3,
    /// 65,540 for (Gauge Controller Not Admin2)
    GaugeControllerNotAdmin2 = 4,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait GAUGECOLTROLLER<Storage: ContractStorage>: ContractContext<Storage> {
    /// """
    /// @notice Contract constructor
    /// @param _token `ERC20CRV` contract address
    /// @param _voting_escrow `VotingEscrow` contract address
    /// """
    fn init(
        &mut self,
        token: Key,
        voting_escrow: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        let address_0: Key = data::zero_address();
        if token == address_0 {
            //Gauge Controller Address Zero 1
            runtime::revert(Error::GaugeControllerAddressZero1);
        }
        if voting_escrow == address_0 {
            //Gauge Controller Address Zero 2
            runtime::revert(Error::GaugeControllerAddressZero2);
        }
        data::set_token(token);
        data::set_voting_escrow(voting_escrow);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_admin(self.get_caller());
        data::set_time_total(
            U256::from(u64::from(runtime::get_blocktime())) / data::WEEK * data::WEEK,
        );
        GAUGETYPENAMES::init();
        GAUGETYPES_::init();
        VOTEUSERSLOPES::init();
        VOTEUSERPOWER::init();
        LASTUSERVOTE::init();
        POINTSWEIGHT::init();
        CHANGESWEIGHT::init();
        TIMEWEIGHT::init();
        GAUGES::init();
        TIMESUM::init();
        POINTSSUM::init();
        CHANGESSUM::init();
        POINTSTOTAL::init();
        POINTSTYPEWEIGHT::init();
        TIMETYPEWEIGHT::init();
    }

    fn gauge_relative_weight(&mut self, addr: Key) -> U256 {

        return self._gauge_relative_weight(addr, U256::from(u64::from(runtime::get_blocktime())));
    }

    fn gauge_relative_weight_write(&mut self, addr: Key) -> U256{

        //self._get_weight(addr);
        //self._get_total();  // Also calculates get_sum
        return self._gauge_relative_weight(addr, U256::from(u64::from(runtime::get_blocktime())));
    }

    fn _gauge_relative_weight(&mut self, addr: Key, time:U256) -> U256
    {
        // """
        // @notice Get Gauge relative weight (not more than 1.0) normalized to 1e18
        //         (e.g. 1.0 == 1e18). Inflation which will be received by it is
        //         inflation_rate * relative_weight / 1e18
        // @param addr Gauge address
        // @param time Relative weight at the specified timestamp in the past or present
        // @return Value of relative weight normalized to 1e18
        // """
        // t: uint256 = time / WEEK * WEEK
        // _total_weight: uint256 = self.points_total[t]
    
        // if _total_weight > 0:
        //     gauge_type: int128 = self.gauge_types_[addr] - 1
        //     _type_weight: uint256 = self.points_type_weight[gauge_type][t]
        //     _gauge_weight: uint256 = self.points_weight[addr][t].bias
        //     return MULTIPLIER * _type_weight * _gauge_weight / _total_weight
    
        // else:
        // return 0
        return 0.into();
    }

    fn change_type_weight(&mut self, type_id: U128, weight:U256) {

        if self.get_caller() == data::admin()
        {
            self._change_type_weight(type_id,weight);
        }
        else {
            runtime::revert(Error::GaugeControllerNotAdmin1);
        }
       
    }

    fn _change_type_weight(&mut self, type_id: U128, weight:U256) {
        // """
        // @notice Change type weight
        // @param type_id Type id
        // @param weight New type weight
        // """
        // old_weight: uint256 = self._get_type_weight(type_id)
        // old_sum: uint256 = self._get_sum(type_id)
        // _total_weight: uint256 = self._get_total()
        // next_time: uint256 = (block.timestamp + WEEK) / WEEK * WEEK
    
        // _total_weight = _total_weight + old_sum * weight - old_sum * old_weight
        // self.points_total[next_time] = _total_weight
        // self.points_type_weight[type_id][next_time] = weight
        // self.time_total = next_time
        // self.time_type_weight[type_id] = next_time
    
        // log NewTypeWeight(type_id, next_time, weight, _total_weight)
       
    }

    fn change_gauge_weight(&mut self, addr: Key, weight:U256) {

        if self.get_caller() == data::admin()
        {
            self._change_gauge_weight(addr,weight);
        }
        else {
            runtime::revert(Error::GaugeControllerNotAdmin2);
        }
       
    }

    fn _change_gauge_weight(&mut self, addr: Key, weight:U256) {
        // # Change gauge weight
        // # Only needed when testing in reality
        // gauge_type: int128 = self.gauge_types_[addr] - 1
        // old_gauge_weight: uint256 = self._get_weight(addr)
        // type_weight: uint256 = self._get_type_weight(gauge_type)
        // old_sum: uint256 = self._get_sum(gauge_type)
        // _total_weight: uint256 = self._get_total()
        // next_time: uint256 = (block.timestamp + WEEK) / WEEK * WEEK

        // self.points_weight[addr][next_time].bias = weight
        // self.time_weight[addr] = next_time

        // new_sum: uint256 = old_sum + weight - old_gauge_weight
        // self.points_sum[gauge_type][next_time].bias = new_sum
        // self.time_sum[gauge_type] = next_time

        // _total_weight = _total_weight + new_sum * type_weight - old_sum * type_weight
        // self.points_total[next_time] = _total_weight
        // self.time_total = next_time

        // log NewGaugeWeight(addr, block.timestamp, weight, _total_weight)
       
    }

    fn get_gauge_weight( &mut self, addr: Key) -> U256{

        return POINTSWEIGHT::instance().get(&addr,&TIMEWEIGHT::instance().get(&addr)).bias;
    }

    fn get_type_weight(&mut self, type_id: U128) -> U256{

        return POINTSTYPEWEIGHT::instance().get(&type_id,&TIMETYPEWEIGHT::instance().get(&U256::from(type_id.as_u128())));
    }

    fn get_total_weight(&mut self) -> U256{

        return POINTSTOTAL::instance().get(&data::time_total());
    }

    fn get_weights_sum_per_type(&mut self, type_id: U128) -> U256{

        return POINTSSUM::instance().get(&type_id,&TIMESUM::instance().get(&U256::from(type_id.as_u128()))).bias;
    }

    // fn _mint_for(&mut self, gauge_addr: Key, _for: Key) {
    //     let controller: Key = self.controller();
    //     let to_mint = 0;
    //     let controller_hash_add_array = match controller {
    //         Key::Hash(package) => package,
    //         _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    //     };
    //     let controller_package_hash = ContractPackageHash::new(controller_hash_add_array);
    //     let ret: U256 = runtime::call_versioned_contract(
    //         controller_package_hash,
    //         None,
    //         "gauge_types",
    //         runtime_args! {"gauge_addr" => gauge_addr},
    //     );

    //     if ret <= U256::from(0) {
    //         //dev: gauge is not added
    //         runtime::revert(Error::MinterGaugeIsNotAdded);
    //     }

    //     let gauge_addr_hash_add_array = match gauge_addr {
    //         Key::Hash(package) => package,
    //         _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    //     };
    //     let gauge_addr_package_hash = ContractPackageHash::new(gauge_addr_hash_add_array);
    //     let ret: () = runtime::call_versioned_contract(
    //         gauge_addr_package_hash,
    //         None,
    //         "user_checkpoint",
    //         runtime_args! {"_for" => _for},
    //     );
    //     let total_mint: U256 = runtime::call_versioned_contract(
    //         gauge_addr_package_hash,
    //         None,
    //         "integrate_fraction",
    //         runtime_args! {"_for" => _for},
    //     );

    //     let minted = self.minted(_for, gauge_addr);
    //     let to_mint: U256 = total_mint - minted;
    //     if to_mint != U256::from(0) {
    //         let token = self.token();
    //         let token_hash_add_array = match token {
    //             Key::Hash(package) => package,
    //             _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    //         };
    //         let token_package_hash = ContractPackageHash::new(token_hash_add_array);
    //         let _result: () = runtime::call_versioned_contract(
    //             token_package_hash,
    //             None,
    //             "mint",
    //             runtime_args! {"to" => _for,"amount" => to_mint},
    //         );
    //         Minted::instance().set(&_for, &gauge_addr, total_mint);
    //         self.emit(&GAUGECOLTROLLEREvent::Minted {
    //             recipient: _for,
    //             gauge: gauge_addr,
    //             minted: total_mint,
    //         });
    //     }
    // }
    // fn mint(&mut self, gauge_addr: Key) {
    //     self._mint_for(gauge_addr, self.get_caller())
    // }
    // fn mint_many(&mut self, gauge_addrs: Vec<Key>) {
    //     for i in 0..(gauge_addrs.len() - 1) {
    //         self._mint_for(gauge_addrs[i], self.get_caller())
    //     }
    // }
    // fn mint_for(&mut self, gauge_addr: Key, _for: Key) {
    //     let is_allowed = self.allowed_to_mint_for(self.get_caller(), _for);
    //     if is_allowed == true {
    //         self._mint_for(gauge_addr, _for)
    //     }
    // }

    // fn toggle_approve_mint(&mut self, minting_user: Key) {
    //     let is_allowed = self.allowed_to_mint_for(minting_user, self.get_caller());
    //     AllowedToMintFor::instance().set(&minting_user, &self.get_caller(), !is_allowed);
    // }

    // fn allowed_to_mint_for(&mut self, owner: Key, spender: Key) -> bool {
    //     AllowedToMintFor::instance().get(&owner, &spender)
    // }
    // fn minted(&mut self, owner: Key, spender: Key) -> U256 {
    //     Minted::instance().get(&owner, &spender)
    // }

    fn token(&mut self) -> Key {
        data::token()
    }
    fn voting_escrow(&mut self) -> Key {
        data::voting_escrow()
    }

    fn emit(&mut self, gauge_controller_event: &GAUGECOLTROLLEREvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match gauge_controller_event {
            GAUGECOLTROLLEREvent::Minted {
                recipient,
                gauge,
                minted,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("gauge", gauge.to_string());
                event.insert("minted", minted.to_string());
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
