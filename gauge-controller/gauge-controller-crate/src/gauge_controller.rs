use crate::alloc::string::ToString;
use crate::data::{
    self, admin, time_total,n_gauge_types,n_gauges,voting_escrow, WEIGHT_VOTE_DELAY, VotedSlope, Point, ChangeSum, ChangesWeight, Gauges, GaugeTypeNames, GaugeTypes_,
    LastUserVote, MULTIPLIER, PointsSum, PointsTotal, PointsTypeWeight, PointsWeight, TimeSum,
    TimeTypeWeight, TimeWeight, VoteUserPower, VoteUserSlopes, WEEK, 
};
use alloc::collections::BTreeMap;
use alloc::{format, string::String, vec::Vec};
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    runtime_args, system::mint::Error as MintError, ApiError, BlockTime, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{set_key, ContractContext, ContractStorage};
use cryptoxide::ed25519;
use hex::encode;
use renvm_sig::{hash_message, keccak256};

pub enum GAUGECONLTROLLEREvent {
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
    AddType {
        name: String,
        type_id: U128
    },
    VoteForGauge {
        time: U256,
        user: Key,
        gauge_addr: Key,
        weight: U256
    },
    NewGauge {
        addr: Key,
        gauge_type: U128,
        weight: U256
    },
}

impl GAUGECONLTROLLEREvent {
    pub fn type_name(&self) -> String {
        match self {
            GAUGECONLTROLLEREvent::Minted {
                recipient: _,
                gauge: _,
                minted: _,
            } => "minted",
            GAUGECONLTROLLEREvent::CommitOwnership { admin: _ } => "CommitOwnership",
            GAUGECONLTROLLEREvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            GAUGECONLTROLLEREvent::NewTypeWeight {
                type_id: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewTypeWeight",
            GAUGECONLTROLLEREvent::NewGaugeWeight {
                gauge_address: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewGaugeWeight",
            GAUGECONLTROLLEREvent::AddType {
                name: _,
                type_id: _,
            } => "AddType",
            GAUGECONLTROLLEREvent::VoteForGauge {
                time: _,
                user: _,
                gauge_addr: _,
                weight: _,
            } => "VoteForGauge",
            GAUGECONLTROLLEREvent::NewGauge {
                addr: _,
                gauge_type: _,
                weight: _, 
            } => "NewGauge",
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
    GaugeControllerGaugeTypeIsZero = 6,
    /// 65,543 for (Gauge Controller Not Admin1)
    GaugeControllerNotAdmin1 = 7,
    /// 65,544 for (Gauge Controller Not Admin2)
    GaugeControllerNotAdmin2 = 8,
    /// 65,545 for (Gauge Controller Not Admin3)
    GaugeControllerNotAdmin3 = 9,
    /// 65,546 for (Gauge Controller Not Admin3)
    GaugeControllerNotAdmin4 = 10,
    /// 65,547 for (Gauge Controller cannot add same gauge twice)
    GaugeControllerCannotAddSameGaugeTwice = 11,
    /// 65,548 for (Gauge Controller gauge type is greater than equal to zero and less than n_gauge_types)
    GaugeControllerGaugeType1= 12,
    /// 65,549 for (Gauge Controller Your token lock expires too soon)
    GaugeControllerTokenLockExpiresTooSoon= 13,
    /// 65,550 for (Gauge Controller You used all your voting power)
    GaugeControllerUsedAllYourVotingPower= 14,
    /// 65,551 for (Gauge Controller You Cannot vote so often)
    GaugeControllerCannotVoteSoOften= 15,
    /// 65,552 for (Gauge Controller Gauge not added)
    GaugeControllerGaugeNotAdded= 16,
    /// 65,552 for (Gauge Controller Used too much power)
    GaugeControllerUsedTooMuchPower= 17,
    

}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub trait GAUGECONLTROLLER<Storage: ContractStorage>: ContractContext<Storage> {
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
        GaugeTypeNames::init();
        GaugeTypes_::init();
        VoteUserSlopes::init();
        VoteUserPower::init();
        LastUserVote::init();
        PointsWeight::init();
        ChangesWeight::init();
        TimeWeight::init();
        Gauges::init();
        TimeSum::init();
        PointsSum::init();
        ChangeSum::init();
        PointsTotal::init();
        PointsTypeWeight::init();
        TimeTypeWeight::init();
    }

    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Gauge Controller Only Admin
            runtime::revert(Error::GaugeControllerOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&GAUGECONLTROLLEREvent::CommitOwnership { admin: addr });
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
        self.emit(&GAUGECONLTROLLEREvent::ApplyOwnership { admin: _admin });
    }

    fn gauge_types(&mut self, _addr: Key) -> U128 {
        let gauge_type = GaugeTypes_::instance().get(&_addr);
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
        let mut t: U256 = TimeWeight::instance().get(&gauge_addr);
        if t > U256::from(0) {
            let mut pt: Point = PointsWeight::instance().get(&gauge_addr, &t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t + WEEK;
                let d_bias: U256 = pt.slope * WEEK;
                if pt.bias > d_bias {
                    pt.bias = pt.bias - d_bias;
                    let d_slope: U256 = ChangesWeight::instance().get(&gauge_addr, &t);
                    pt.slope = pt.slope - d_slope;
                } else {
                    pt.bias = 0.into();
                    pt.slope = 0.into();
                }
                PointsWeight::instance().set(&gauge_addr, &t, pt);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TimeWeight::instance().set(&gauge_addr, t);
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
        let mut pt: U256 = PointsTotal::instance().get(&t);
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
                let type_sum: U256 = PointsSum::instance().get(&U128::from(gauge_type), &t).bias;
                let type_weight: U256 =
                    PointsTypeWeight::instance().get(&U128::from(gauge_type), &t);
                pt = pt + (type_sum * type_weight);
            }
            PointsTotal::instance().set(&t, pt);
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
        let mut t: U256 = TimeSum::instance().get(&U256::from(gauge_type.as_u128()));
        if t > U256::from(0) {
            let mut pt: Point = PointsSum::instance().get(&gauge_type, &t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t + WEEK;
                let d_bias: U256 = pt.slope * WEEK;
                if pt.bias > d_bias {
                    pt.bias = pt.bias - d_bias;
                    let d_slope: U256 = ChangeSum::instance().get(&gauge_type, &t);
                    pt.slope = d_slope;
                } else {
                    pt.bias = U256::from(0);
                    pt.slope = U256::from(0);
                }
                PointsSum::instance().set(&gauge_type, &t, pt);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TimeSum::instance().set(&U256::from(gauge_type.as_u128()), t)
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
        let mut t: U256 = TimeTypeWeight::instance().get(&U256::from(gauge_type.as_u128()));
        if t > U256::from(0) {
            let w: U256 = PointsTypeWeight::instance().get(&gauge_type, &t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t + WEEK;
                PointsTypeWeight::instance().set(&gauge_type, &t, w);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TimeTypeWeight::instance().set(&U256::from(gauge_type.as_u128()), t)
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
        let _total_weight = PointsTotal::instance().get(&t);

        if _total_weight > U256::from(0) {
            let gauge_type: U128 = GaugeTypes_::instance().get(&addr);
            let _type_weight: U256 = PointsTypeWeight::instance().get(&gauge_type, &t);
            let _gauge_weight: U256 = PointsWeight::instance().get(&addr, &t).bias;
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

        PointsTotal::instance().set(&next_time, _total_weight);
        PointsTypeWeight::instance().set(&type_id, &next_time, weight);
        data::set_time_total(next_time);
        TimeTypeWeight::instance().set(&U256::from(type_id.as_u128()), next_time);
        self.emit(&GAUGECONLTROLLEREvent::NewTypeWeight {
            type_id: type_id,
            time: next_time,
            weight: weight,
            total_weight: _total_weight,
        });
    }

    // # Change gauge weight
    // # Only needed when testing in reality
    fn _change_gauge_weight(&mut self, addr: Key, weight: U256) {
        let gauge_type: U128 = GaugeTypes_::instance().get(&addr) - U128::from(1);
        let old_gauge_weight: U256 = self._get_weight(addr);
        let type_weight: U256 = self._get_type_weight(gauge_type);
        let old_sum: U256 = self._get_sum(gauge_type);
        let _total_weight: U256 = self._get_total();
        let next_time: U256 =
            (U256::from(u64::from(runtime::get_blocktime())) + WEEK) / WEEK * WEEK;
        let mut points_wight = PointsWeight::instance().get(&addr, &next_time);
        points_wight.bias = weight;
        PointsWeight::instance().set(&addr, &next_time, points_wight);
        TimeWeight::instance().set(&addr, next_time);
        let new_sum: U256 = old_sum + weight - old_gauge_weight;
        let mut point_sum: Point = PointsSum::instance().get(&gauge_type, &next_time);
        point_sum.bias = new_sum;
        PointsSum::instance().set(&gauge_type, &next_time, point_sum);
        TimeSum::instance().set(&U256::from(gauge_type.as_u128()), next_time);
        let _total_weight = _total_weight + new_sum * type_weight - old_sum * type_weight;
        PointsTotal::instance().set(&next_time, _total_weight);
        data::set_time_total(next_time);

        self.emit(&GAUGECONLTROLLEREvent::NewGaugeWeight {
            gauge_address: addr,
            time: U256::from(u64::from(runtime::get_blocktime())),
            weight: weight,
            total_weight: _total_weight,
        });
    }

    fn gauge_relative_weight(&mut self, addr: Key) -> U256 {
        return self._gauge_relative_weight(addr, U256::from(u64::from(runtime::get_blocktime())));
    }

    fn gauge_relative_weight_write(&mut self, addr: Key) -> U256 {
        self._get_weight(addr);
        self._get_total();  // Also calculates get_sum
        return self._gauge_relative_weight(addr, U256::from(u64::from(runtime::get_blocktime())));
    }

    fn change_type_weight(&mut self, type_id: U128, weight: U256) {
        if self.get_caller() == data::admin() {
            self._change_type_weight(type_id, weight);
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin1);
        }
    }

    fn change_gauge_weight(&mut self, addr: Key, weight: U256) {
        if self.get_caller() == data::admin() {
            self._change_gauge_weight(addr, weight);
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin2);
        }
    }

    fn get_gauge_weight(&mut self, addr: Key) -> U256 {
        return PointsWeight::instance()
            .get(&addr, &TimeWeight::instance().get(&addr))
            .bias;
    }

    fn get_type_weight(&mut self, type_id: U128) -> U256 {
        return PointsTypeWeight::instance().get(
            &type_id,
            &TimeTypeWeight::instance().get(&U256::from(type_id.as_u128())),
        );
    }

    fn get_total_weight(&mut self) -> U256 {
        return PointsTotal::instance().get(&data::time_total());
    }

    fn get_weights_sum_per_type(&mut self, type_id: U128) -> U256 {
        return PointsSum::instance()
            .get(
                &type_id,
                &TimeSum::instance().get(&U256::from(type_id.as_u128())),
            )
            .bias;
    }

    fn add_type(&mut self,_name : String) {
        let weight: U256 = 0.into();
       
        if self.get_caller() == data::admin() {
            let type_id: U128 = data::n_gauge_types();
            GaugeTypeNames::instance().set(&type_id,_name);
            data::set_n_gauge_types(type_id + U128::from(1));
            if weight != U256::from(0)
            {
                self._change_type_weight(type_id, weight);
                self.emit(&GAUGECONLTROLLEREvent::AddType { name: _name, type_id: type_id});
            }
                
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin3);
        }
        
    }

    fn add_gauge(&mut self,addr : Key, gauge_type:U128) {
        let weight: U256 = 0.into();
        if self.get_caller() == data::admin() {
            if gauge_type >= U128::from(0) && gauge_type < data::n_gauge_types()
            {
                if GaugeTypes_::instance().get(&addr) == U128::from(0) // dev: cannot add the same gauge twice
                {
                    let n: U128 = data::n_gauges();
                    data::set_n_gauges(n +  U128::from(1));
                    Gauges::instance().set(&U256::from(n.as_u128()),addr);
                    GaugeTypes_::instance().set(&addr,gauge_type + U128::from(1));
                    let next_time: U256 = ((U256::from(u64::from(runtime::get_blocktime()))+ data::WEEK) / data::WEEK) * data::WEEK;
                    if weight > U256::from(0)
                    {
                        let mut _type_weight: U256 = self._get_type_weight(gauge_type);
                        let mut _old_sum: U256 = self._get_sum(gauge_type);
                        let mut _old_total: U256 = self._get_total();

                        let mut points_sum_result=PointsSum::instance().get(&gauge_type,&next_time);
                        (points_sum_result).bias = weight + _old_sum;
                        PointsSum::instance().set(&gauge_type,&next_time,points_sum_result);

                        TimeSum::instance().set(&U256::from(gauge_type.as_u128()),next_time);
                        PointsTotal::instance().set(&next_time, _old_total + (_type_weight * weight));
                        data::set_time_total(next_time);

                        let mut points_weight_result=PointsWeight::instance().get(&addr,&next_time);
                        (points_weight_result).bias=weight;
                        PointsWeight::instance().set(&addr,&next_time,points_weight_result);
                    }

                    if TimeSum::instance().get(&U256::from(gauge_type.as_u128())) == U256::from(0)
                    {
                        TimeSum::instance().set(&U256::from(gauge_type.as_u128()),next_time); 
                    }

                    TimeWeight::instance().set(&addr,next_time); 
                    self.emit(&GAUGECONLTROLLEREvent::NewGauge { addr: addr, gauge_type: gauge_type,weight: weight});

                } else {
                    runtime::revert(Error::GaugeControllerCannotAddSameGaugeTwice);
                }
            } else {
                runtime::revert(Error::GaugeControllerGaugeType1);
            }
        }
        else {
            runtime::revert(Error::GaugeControllerNotAdmin4);
        }
    }

    fn vote_for_gauge_weights(&mut self,_gauge_addr:Key, _user_weight:U256) {

        let escrow: Key = data::voting_escrow();

        //convert Key to ContractPackageHash
        let escrow_package_hash_add_array = match escrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let escrow_package_hash = ContractPackageHash::new(escrow_package_hash_add_array);

        let slope: U256 = runtime::call_versioned_contract(
            escrow_package_hash,
            None,
            "get_last_user_slope",
            runtime_args! {"msg.sender" => self.get_caller()},
        );
        
        let lock_end: U256 = runtime::call_versioned_contract(
            escrow_package_hash,
            None,
            "locked__end",
            runtime_args! {"msg.sender" => self.get_caller()},
        );

        let _n_gauges: U128 = data::n_gauges();
        let next_time: U256 = ((U256::from(u64::from(runtime::get_blocktime()))+ data::WEEK) / data::WEEK) * data::WEEK;

        if lock_end > next_time
        {
            if _user_weight >= U256::from(0) && _user_weight <= U256::from(10000){

                if (U256::from(u64::from(runtime::get_blocktime()))) >= (LastUserVote::instance().get(&self.get_caller(),&_gauge_addr) + data::WEIGHT_VOTE_DELAY) {
                    let gauge_type: U128 = GaugeTypes_::instance().get(&_gauge_addr) - U128::from(1);
                    if gauge_type >= U128::from(0)
                    {
                        // Prepare slopes and biases in memory
                        let old_slope: VotedSlope = VoteUserSlopes::instance().get(&self.get_caller(),&_gauge_addr);
                        let mut old_dt: U256 = 0.into();
                        if old_slope.end > next_time{
                            old_dt = old_slope.end - next_time;
                        }
                        let old_bias: U256 = old_slope.slope * old_dt;
                        let new_slope: VotedSlope = VotedSlope{
                            slope: slope * (_user_weight / U256::from(100000)),
                            end: lock_end,
                            power: _user_weight
                        };
                        let new_dt: U256 = lock_end - next_time;  // dev: raises when expired
                        let new_bias: U256 = new_slope.slope * new_dt;

                        // Check and update powers (weights) used
                        let mut power_used: U256 =  VoteUserPower::instance().get(&self.get_caller());
                        power_used = power_used + new_slope.power - old_slope.power;
                        VoteUserPower::instance().set(&self.get_caller(),power_used);

                        if (power_used >= 0.into()) && (power_used <= 10000.into()){

                            // Remove old and schedule new slope changes
                            // Remove slope changes for old slopes
                            // Schedule recording of initial slope for next_time

                            let old_weight_bias: U256 = self._get_weight(_gauge_addr);
                            let old_weight_slope: U256 = PointsWeight::instance().get(&_gauge_addr,&next_time).slope;
                            let old_sum_bias: U256 = self._get_sum(gauge_type);
                            let old_sum_slope: U256 = PointsSum::instance().get(&gauge_type,&next_time).slope;

                            let max_weight_bias = old_weight_bias + new_bias;
                            let max_sum_bias = old_sum_bias + new_bias;

                            if max_weight_bias > old_bias
                            {
                                let mut points_weight_result= PointsWeight::instance().get(&_gauge_addr,&next_time);
                                (points_weight_result).bias = max_weight_bias - old_bias;
                                PointsWeight::instance().set(&_gauge_addr,&next_time,points_weight_result);
                            }
                            else{
                             
                                let mut points_weight_result= PointsWeight::instance().get(&_gauge_addr,&next_time);
                                (points_weight_result).bias = old_bias - old_bias;
                                PointsWeight::instance().set(&_gauge_addr,&next_time,points_weight_result);
                            }

                            if max_sum_bias > old_bias
                            {

                                let mut points_sum_result= PointsSum::instance().get(&gauge_type,&next_time);
                                (points_sum_result).bias =  max_sum_bias - old_bias;
                                PointsSum::instance().set(&gauge_type,&next_time,points_sum_result);
                            }
                            else{
                                
                                let mut points_sum_result= PointsSum::instance().get(&gauge_type,&next_time);
                                (points_sum_result).bias =  old_bias - old_bias;
                                PointsSum::instance().set(&gauge_type,&next_time,points_sum_result);
                            }
        
                            if old_slope.end > next_time{

                                let max_weight_slope = old_weight_slope + new_slope.slope;
                                let max_sum_slope = old_sum_slope + new_slope.slope;

                                if max_weight_slope > old_slope.slope
                                {
                                    let mut points_weight_result= PointsWeight::instance().get(&_gauge_addr,&next_time);
                                    (points_weight_result).slope = max_weight_slope - old_slope.slope;
                                    PointsWeight::instance().set(&_gauge_addr,&next_time,points_weight_result);
                                }
                                else{
                                
                                    let mut points_weight_result= PointsWeight::instance().get(&_gauge_addr,&next_time);
                                    (points_weight_result).slope = old_slope.slope - old_slope.slope;
                                    PointsWeight::instance().set(&_gauge_addr,&next_time,points_weight_result);
                                }

                                if max_sum_slope > old_slope.slope
                                {

                                    let mut points_sum_result= PointsSum::instance().get(&gauge_type,&next_time);
                                    (points_sum_result).slope =  max_sum_slope - old_slope.slope;
                                    PointsSum::instance().set(&gauge_type,&next_time,points_sum_result);
                                }
                                else{
                                    
                                    let mut points_sum_result= PointsSum::instance().get(&gauge_type,&next_time);
                                    (points_sum_result).slope =  old_slope.slope - old_slope.slope;
                                    PointsSum::instance().set(&gauge_type,&next_time,points_sum_result);
                                }

                            }
                                
                            else{

                                let mut points_weight_result= PointsWeight::instance().get(&_gauge_addr,&next_time);
                                (points_weight_result).slope = (points_weight_result).slope + new_slope.slope;
                                PointsWeight::instance().set(&_gauge_addr,&next_time,points_weight_result);

                                let mut points_sum_result= PointsSum::instance().get(&gauge_type,&next_time);
                                (points_sum_result).slope =  (points_sum_result).slope + new_slope.slope;
                                PointsSum::instance().set(&gauge_type,&next_time,points_sum_result);

                            }
                                
                            if old_slope.end > U256::from(u64::from(runtime::get_blocktime())){
                                // Cancel old slope changes if they still didn't happen

                                let mut changes_weight_result= ChangesWeight::instance().get(&_gauge_addr,&old_slope.end);
                                changes_weight_result = changes_weight_result - old_slope.slope;
                                ChangesWeight::instance().set(&_gauge_addr,&old_slope.end,changes_weight_result);

                                let mut changes_sum_result= ChangeSum::instance().get(&gauge_type,&old_slope.end);
                                changes_sum_result = changes_sum_result - old_slope.slope;
                                ChangeSum::instance().set(&gauge_type,&old_slope.end,changes_sum_result);
                            }
                                
                            // Add slope changes for new slopes

                            let mut changes_weight_result= ChangesWeight::instance().get(&_gauge_addr,&new_slope.end);
                            changes_weight_result = changes_weight_result + new_slope.slope;
                            ChangesWeight::instance().set(&_gauge_addr,&old_slope.end,changes_weight_result);

                            let mut changes_sum_result= ChangeSum::instance().get(&gauge_type,&new_slope.end);
                            changes_sum_result = changes_sum_result + new_slope.slope;
                            ChangeSum::instance().set(&gauge_type,&old_slope.end,changes_sum_result);

                            self._get_total();

                            VoteUserSlopes::instance().set(&self.get_caller(),&_gauge_addr,new_slope);

                            //Record last action time
                            LastUserVote::instance().set(&self.get_caller(),&_gauge_addr,U256::from(u64::from(runtime::get_blocktime())));
                           
                            self.emit(&GAUGECONLTROLLEREvent::VoteForGauge { time:U256::from(u64::from(runtime::get_blocktime())), user:self.get_caller(), gauge_addr:_gauge_addr, weight:_user_weight});
                        }
                        else{
                            runtime::revert(Error::GaugeControllerUsedTooMuchPower);
                        }

                    }
                    else{
                        runtime::revert(Error::GaugeControllerGaugeNotAdded);
                    }
                    
                } else{
                    runtime::revert(Error::GaugeControllerCannotVoteSoOften);
                }

            } else{
                runtime::revert(Error::GaugeControllerUsedAllYourVotingPower);
            }
            
        } else{
            runtime::revert(Error::GaugeControllerTokenLockExpiresTooSoon);
        } 
        

        
        
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
    //         self.emit(&GAUGECONLTROLLEREvent::Minted {
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

    fn emit(&mut self, gauge_controller_event: &GAUGECONLTROLLEREvent) {
        let mut events = Vec::new();
        let package = data::get_package_hash();
        match gauge_controller_event {
            GAUGECONLTROLLEREvent::Minted {
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
            GAUGECONLTROLLEREvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            GAUGECONLTROLLEREvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            GAUGECONLTROLLEREvent::NewTypeWeight {
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
            GAUGECONLTROLLEREvent::NewGaugeWeight {
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
            GAUGECONLTROLLEREvent::AddType {
                name,
                type_id,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("name", name.to_string());
                event.insert("type_id", type_id.to_string());
                events.push(event);
            }
            GAUGECONLTROLLEREvent::NewGauge {
                addr,
                gauge_type,
                weight
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("addr", addr.to_string());
                event.insert("gauge_type", gauge_type.to_string());
                event.insert("weight", weight.to_string());
                events.push(event);
            }
            GAUGECONLTROLLEREvent::VoteForGauge {
                time,
                user,
                gauge_addr,
                weight
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("time", time.to_string());
                event.insert("user", user.to_string());
                event.insert("gauge_addr", gauge_addr.to_string());
                event.insert("weight", weight.to_string());
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
