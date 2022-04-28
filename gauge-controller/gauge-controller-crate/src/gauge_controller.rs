use crate::alloc::string::ToString;
use crate::data::{
    self, Point, CHANGESSUM, CHANGESWEIGHT, GAUGES, GAUGETYPENAMES, GAUGETYPES_, LASTUSERVOTE,
    MULTIPLIER, POINTSSUM, POINTSTOTAL, POINTSTYPEWEIGHT, POINTSWEIGHT, TIMESUM, TIMETYPEWEIGHT,
    TIMEWEIGHT, VOTEUSERPOWER, VOTEUSERSLOPES, WEEK,
};
use alloc::collections::BTreeMap;
use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::U128;
use casper_types::{
    runtime_args, system::mint::Error as MintError, ApiError, BlockTime, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, URef, U256,
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
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    NewTypeWeight {
        type_id: U128,
        time: U256,
        weight: U256,
        total_weight: U256,
    },
    NewGaugeWeight {
        gauge_address: Key,
        time: U256,
        weight: U256,
        total_weight: U256,
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
            GAUGECOLTROLLEREvent::CommitOwnership { admin: _ } => "CommitOwnership",
            GAUGECOLTROLLEREvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            GAUGECOLTROLLEREvent::NewTypeWeight {
                type_id: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewTypeWeight",
            GAUGECOLTROLLEREvent::NewGaugeWeight {
                gauge_address: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewTypeWeight",
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
    /// 65,539 for (Gauge Controller Only Admin1)
    GaugeControllerOnlyAdmin1 = 3,
    /// 65,540 for (Gauge Controller Only Admin2)
    GaugeControllerOnlyAdmin2 = 4,
    /// 65,541 for (Gauge Controller Admin Not Set)
    GaugeControllerAdminNotSet = 5,
    /// 65,542 for (Gauge Controller Gauge Type Is Zero)
    GaugeControllerGaugeTypeIsZero,
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

    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Gauge Controller Only Admin
            runtime::revert(Error::GaugeControllerOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&GAUGECOLTROLLEREvent::CommitOwnership { admin: addr });
    }
    fn apply_transfer_ownership(&mut self) {
        if self.get_caller() != self.admin() {
            //Gauge Controller Only Admin
            runtime::revert(Error::GaugeControllerOnlyAdmin2);
        }
        let _admin = data::future_admin();
        if _admin == data::zero_address() {
            //Gauge Controller Admin Not Set
            runtime::revert(Error::GaugeControllerAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&GAUGECOLTROLLEREvent::ApplyOwnership { admin: _admin });
    }

    fn gauge_types(&mut self, _addr: Key) -> U128 {
        let gauge_type = GAUGETYPES_::instance().get(&_addr);
        if gauge_type == U128::from(0) {
            //Gauge Controller Gauge Type Is Zero
            runtime::revert(Error::GaugeControllerGaugeTypeIsZero);
        }
        return gauge_type - U128::from(1);
    }

    fn checkpoint(&mut self) {
        self._get_total();
    }

    fn checkpoint_gauge(&mut self, addr: Key) {
        self._get_weight(addr);
        self._get_total();
    }

    /// """
    /// @notice Fill historic gauge weights week-over-week for missed checkins
    ///         and return the total for the future week
    /// @param gauge_addr Address of the gauge
    /// @return Gauge weight
    /// """
    fn _get_weight(&mut self, gauge_addr: Key) -> U256 {
        let mut t: U256 = TIMEWEIGHT::instance().get(&gauge_addr);
        if t > U256::from(0) {
            let mut pt: Point = POINTSWEIGHT::instance().get(&gauge_addr, &t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t + WEEK;
                let d_bias: U256 = pt.slope * WEEK;
                if pt.bias > d_bias {
                    pt.bias = pt.bias - d_bias;
                    let d_slope: U256 = CHANGESWEIGHT::instance().get(&gauge_addr, &t);
                    pt.slope = pt.slope - d_slope;
                } else {
                    pt.bias = 0.into();
                    pt.slope = 0.into();
                }
                POINTSWEIGHT::instance().set(&gauge_addr, &t, pt);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TIMEWEIGHT::instance().set(&gauge_addr, t);
                }
            }
            return pt.bias;
        } else {
            return U256::from(0);
        }
    }
    /// """
    /// @notice Fill historic total weights week-over-week for missed checkins
    ///         and return the total for the future week
    /// @return Total weight
    /// """
    fn _get_total(&mut self) -> U256 {
        let mut t: U256 = data::time_total();
        let mut _n_gauge_types: U128 = data::n_gauge_types();
        if t > U256::from(u64::from(runtime::get_blocktime())) {
            // # If we have already checkpointed - still need to change the value
            t = t - WEEK;
        }
        let mut pt: U256 = POINTSTOTAL::instance().get(&t);
        for gauge_type in 0..(100) {
            if U128::from(gauge_type) == _n_gauge_types {
                break;
            }
            self._get_sum(U128::from(gauge_type));
            self._get_type_weight(U128::from(gauge_type));
        }
        for _ in 0..(500) {
            if t > U256::from(u64::from(runtime::get_blocktime())) {
                break;
            }
            t = t + WEEK;
            pt = U256::from(0);
            for gauge_type in 0..(100) {
                if U128::from(gauge_type) == _n_gauge_types {
                    break;
                }
                let type_sum: U256 = POINTSSUM::instance().get(&U128::from(gauge_type), &t).bias;
                let type_weight: U256 =
                    POINTSTYPEWEIGHT::instance().get(&U128::from(gauge_type), &t);
                pt = pt + (type_sum * type_weight);
            }
            POINTSTOTAL::instance().set(&t, pt);
            if t > U256::from(u64::from(runtime::get_blocktime())) {
                data::set_time_total(t);
            }
        }
        return pt;
    }

    /// """
    /// @notice Fill sum of gauge weights for the same type week-over-week for
    ///         missed checkins and return the sum for the future week
    /// @param gauge_type Gauge type id
    /// @return Sum of weights
    /// """
    fn _get_sum(&mut self, gauge_type: U128) -> U256 {
        let mut t: U256 = TIMESUM::instance().get(&U256::from(gauge_type.as_u128()));
        if t > U256::from(0) {
            let mut pt: Point = POINTSSUM::instance().get(&gauge_type, &t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t + WEEK;
                let d_bias: U256 = pt.slope * WEEK;
                if pt.bias > d_bias {
                    pt.bias = pt.bias - d_bias;
                    let d_slope: U256 = CHANGESSUM::instance().get(&gauge_type, &t);
                    pt.slope = d_slope;
                } else {
                    pt.bias = U256::from(0);
                    pt.slope = U256::from(0);
                }
                POINTSSUM::instance().set(&gauge_type, &t, pt);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TIMESUM::instance().set(&U256::from(gauge_type.as_u128()), t)
                }
            }
            return pt.bias;
        } else {
            return U256::from(0);
        }
    }
    /// """
    /// @notice Fill historic type weights week-over-week for missed checkins
    ///         and return the type weight for the future week
    /// @param gauge_type Gauge type id
    /// @return Type weight
    /// """
    fn _get_type_weight(&mut self, gauge_type: U128) -> U256 {
        let mut t: U256 = TIMETYPEWEIGHT::instance().get(&U256::from(gauge_type.as_u128()));
        if t > U256::from(0) {
            let w: U256 = POINTSTYPEWEIGHT::instance().get(&gauge_type, &t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t + WEEK;
                POINTSTYPEWEIGHT::instance().set(&gauge_type, &t, w);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TIMETYPEWEIGHT::instance().set(&U256::from(gauge_type.as_u128()), t)
                }
            }
            return w;
        } else {
            return U256::from(0);
        }
    }

    /// """
    /// @notice Get Gauge relative weight (not more than 1.0) normalized to 1e18
    ///         (e.g. 1.0 == 1e18). Inflation which will be received by it is
    ///         inflation_rate * relative_weight / 1e18
    /// @param addr Gauge address
    /// @param time Relative weight at the specified timestamp in the past or present
    /// @return Value of relative weight normalized to 1e18
    /// """
    fn _gauge_relative_weight(&mut self, addr: Key, time: U256) -> U256 {
        let t: U256 = time / WEEK * WEEK;
        let _total_weight = POINTSTOTAL::instance().get(&t);

        if _total_weight > U256::from(0) {
            let gauge_type: U128 = GAUGETYPES_::instance().get(&addr);
            let _type_weight: U256 = POINTSTYPEWEIGHT::instance().get(&gauge_type, &t);
            let _gauge_weight: U256 = POINTSWEIGHT::instance().get(&addr, &t).bias;
            return MULTIPLIER * _type_weight * _gauge_weight / _total_weight;
        } else {
            return U256::from(0);
        }
    }

    /// """
    /// @notice Change type weight
    /// @param type_id Type id
    /// @param weight New type weight
    /// """
    fn _change_type_weight(&mut self, type_id: U128, weight: U256) {
        let old_weight: U256 = self._get_type_weight(type_id);
        let old_sum: U256 = self._get_sum(type_id);
        let _total_weight: U256 = self._get_total();
        let next_time: U256 =
            (U256::from(u64::from(runtime::get_blocktime())) + WEEK) / WEEK * WEEK;

        let _total_weight = _total_weight + old_sum * weight - old_sum * old_weight;

        POINTSTOTAL::instance().set(&next_time, _total_weight);
        POINTSTYPEWEIGHT::instance().set(&type_id, &next_time, weight);
        data::set_time_total(next_time);
        TIMETYPEWEIGHT::instance().set(&U256::from(type_id.as_u128()), next_time);
        self.emit(&GAUGECOLTROLLEREvent::NewTypeWeight {
            type_id: type_id,
            time: next_time,
            weight: weight,
            total_weight: _total_weight,
        });
    }

    // # Change gauge weight
    // # Only needed when testing in reality
    fn _change_gauge_weight(&mut self, addr: Key, weight: U256) {
        let gauge_type: U128 = GAUGETYPES_::instance().get(&addr) - U128::from(1);
        let old_gauge_weight: U256 = self._get_weight(addr);
        let type_weight: U256 = self._get_type_weight(gauge_type);
        let old_sum: U256 = self._get_sum(gauge_type);
        let _total_weight: U256 = self._get_total();
        let next_time: U256 =
            (U256::from(u64::from(runtime::get_blocktime())) + WEEK) / WEEK * WEEK;
        let mut points_wight = POINTSWEIGHT::instance().get(&addr, &next_time);
        points_wight.bias = weight;
        POINTSWEIGHT::instance().set(&addr, &next_time, points_wight);
        TIMEWEIGHT::instance().set(&addr, next_time);
        let new_sum: U256 = old_sum + weight - old_gauge_weight;
        let mut point_sum: Point = POINTSSUM::instance().get(&gauge_type, &next_time);
        point_sum.bias = new_sum;
        POINTSSUM::instance().set(&gauge_type, &next_time, point_sum);
        TIMESUM::instance().set(&U256::from(gauge_type.as_u128()), next_time);
        let _total_weight = _total_weight + new_sum * type_weight - old_sum * type_weight;
        POINTSTOTAL::instance().set(&next_time, _total_weight);
        data::set_time_total(next_time);

        self.emit(&GAUGECOLTROLLEREvent::NewGaugeWeight {
            gauge_address: addr,
            time: U256::from(u64::from(runtime::get_blocktime())),
            weight: weight,
            total_weight: _total_weight,
        });
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
    fn admin(&mut self) -> Key {
        data::admin()
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
            GAUGECOLTROLLEREvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            GAUGECOLTROLLEREvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            GAUGECOLTROLLEREvent::NewTypeWeight {
                type_id,
                time,
                weight,
                total_weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("type_id", type_id.to_string());
                event.insert("time", time.to_string());
                event.insert("weight", weight.to_string());
                event.insert("total_weight", total_weight.to_string());
                events.push(event);
            }
            GAUGECOLTROLLEREvent::NewGaugeWeight {
                gauge_address,
                time,
                weight,
                total_weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("gauge_address", gauge_address.to_string());
                event.insert("time", time.to_string());
                event.insert("weight", weight.to_string());
                event.insert("total_weight", total_weight.to_string());
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
