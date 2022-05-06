use crate::alloc::string::ToString;
use crate::data::{
    self, admin, time_total, ChangeSum, ChangesWeight, GaugeTypeNames, GaugeTypes_, Gauges,
    LastUserVote, Point, PointsSum, PointsTotal, PointsTypeWeight, PointsWeight, TimeSum,
    TimeTypeWeight, TimeWeight, VoteUserPower, VoteUserSlopes, VotedSlope, MULTIPLIER, WEEK,
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
    GaugeControllerGaugeTypeIsZero = 6,
    /// 65,539 for (Gauge Controller Not Admin1)
    GaugeControllerNotAdmin1 = 7,
    /// 65,540 for (Gauge Controller Not Admin2)
    GaugeControllerNotAdmin2 = 8,
    GaugeControllerOverFlow1 = 9,
    GaugeControllerOverFlow2 = 10,
    GaugeControllerOverFlow3 = 11,
    GaugeControllerOverFlow4 = 12,
    GaugeControllerOverFlow5 = 13,
    GaugeControllerOverFlow6 = 14,
    GaugeControllerOverFlow7 = 15,
    GaugeControllerOverFlow8 = 16,
    GaugeControllerOverFlow9 = 17,
    GaugeControllerOverFlow10 = 18,
    GaugeControllerUnderFlow1 = 19,
    GaugeControllerUnderFlow2 = 20,
    GaugeControllerUnderFlow3 = 21,
    GaugeControllerUnderFlow4 = 22,
    GaugeControllerUnderFlow5 = 23,
    GaugeControllerUnderFlow6 = 24,
    GaugeControllerUnderFlow7 = 25,
    GaugeControllerUnderFlow8 = 26,
    GaugeControllerUnderFlow9 = 27,
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
        let _admin = self.future_admin();
        if _admin == data::zero_address() {
            //Gauge Controller Admin Not Set
            runtime::revert(Error::GaugeControllerAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&GAUGECONLTROLLEREvent::ApplyOwnership { admin: _admin });
    }

    fn gauge_types(&mut self, _addr: Key) -> U128 {
        let gauge_type = self.gauge_types_(_addr);
        if gauge_type == U128::from(0) {
            //Gauge Controller Gauge Type Is Zero
            runtime::revert(Error::GaugeControllerGaugeTypeIsZero);
        }
        return gauge_type
            .checked_sub(U128::from(1))
            .ok_or(Error::GaugeControllerUnderFlow1)
            .unwrap_or_revert();
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
        let mut t: U256 = self.time_weight(gauge_addr);
        if t > U256::from(0) {
            let mut pt: Point = self.points_weight(gauge_addr, t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t
                    .checked_add(WEEK)
                    .ok_or(Error::GaugeControllerOverFlow1)
                    .unwrap_or_revert();

                let d_bias: U256 = pt.slope * WEEK;
                if pt.bias > d_bias {
                    pt.bias = pt
                        .bias
                        .checked_sub(d_bias)
                        .ok_or(Error::GaugeControllerUnderFlow2)
                        .unwrap_or_revert();
                    let d_slope: U256 = self.changes_weight(gauge_addr, t);
                    pt.slope = pt
                        .slope
                        .checked_sub(d_slope)
                        .ok_or(Error::GaugeControllerUnderFlow3)
                        .unwrap_or_revert();
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
        let mut t: U256 = self.time_total();
        let mut _n_gauge_types: U128 = self.n_gauge_types();
        if t > U256::from(u64::from(runtime::get_blocktime())) {
            // # If we have already checkpointed - still need to change the value
            t = t
                .checked_sub(WEEK)
                .ok_or(Error::GaugeControllerUnderFlow4)
                .unwrap_or_revert();
        }
        let mut pt: U256 = self.points_total(t);
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
            t = t
                .checked_add(WEEK)
                .ok_or(Error::GaugeControllerOverFlow2)
                .unwrap_or_revert();
            pt = U256::from(0);
            for gauge_type in 0..(100) {
                if U128::from(gauge_type) == _n_gauge_types {
                    break;
                }
                let type_sum: U256 = self.points_sum(U128::from(gauge_type), t).bias;
                let type_weight: U256 = self.points_type_weight(U128::from(gauge_type), t);
                pt = pt
                    .checked_add(type_sum * type_weight)
                    .ok_or(Error::GaugeControllerOverFlow3)
                    .unwrap_or_revert();
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
        let mut t: U256 = self.time_sum(U256::from(gauge_type.as_u128()));
        if t > U256::from(0) {
            let mut pt: Point = self.points_sum(gauge_type, t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t
                    .checked_add(WEEK)
                    .ok_or(Error::GaugeControllerOverFlow4)
                    .unwrap_or_revert();
                let d_bias: U256 = pt.slope * WEEK;
                if pt.bias > d_bias {
                    pt.bias = pt
                        .bias
                        .checked_sub(d_bias)
                        .ok_or(Error::GaugeControllerUnderFlow5)
                        .unwrap_or_revert();
                    let d_slope: U256 = self.change_sum(gauge_type, t);
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
        let mut t: U256 = self.time_type_weight(U256::from(gauge_type.as_u128()));
        if t > U256::from(0) {
            let w: U256 = self.points_type_weight(gauge_type, t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t
                    .checked_add(WEEK)
                    .ok_or(Error::GaugeControllerOverFlow5)
                    .unwrap_or_revert();
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
        let _total_weight = self.points_total(t);

        if _total_weight > U256::from(0) {
            let gauge_type: U128 = self.gauge_types_(addr);
            let _type_weight: U256 = self.points_type_weight(gauge_type, t);
            let _gauge_weight: U256 = self.points_weight(addr, t).bias;
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
        let next_time: U256 = (U256::from(u64::from(runtime::get_blocktime()))
            .checked_add(WEEK)
            .ok_or(Error::GaugeControllerOverFlow6)
            .unwrap_or_revert())
            / WEEK
            * WEEK;

        let _total_weight = _total_weight
            .checked_add(
                (old_sum * weight)
                    .checked_sub(old_sum * old_weight)
                    .ok_or(Error::GaugeControllerUnderFlow6)
                    .unwrap_or_revert(),
            )
            .ok_or(Error::GaugeControllerOverFlow7)
            .unwrap_or_revert();

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
        let gauge_type: U128 = self
            .gauge_types_(addr)
            .checked_sub(U128::from(1))
            .ok_or(Error::GaugeControllerUnderFlow7)
            .unwrap_or_revert();
        let old_gauge_weight: U256 = self._get_weight(addr);
        let type_weight: U256 = self._get_type_weight(gauge_type);
        let old_sum: U256 = self._get_sum(gauge_type);
        let _total_weight: U256 = self._get_total();
        let next_time: U256 = (U256::from(u64::from(runtime::get_blocktime()))
            .checked_add(WEEK)
            .ok_or(Error::GaugeControllerOverFlow8)
            .unwrap_or_revert())
            / WEEK
            * WEEK;
        let mut points_wight = self.points_weight(addr, next_time);
        points_wight.bias = weight;
        PointsWeight::instance().set(&addr, &next_time, points_wight);
        TimeWeight::instance().set(&addr, next_time);
        let new_sum: U256 = old_sum
            .checked_add(
                weight
                    .checked_sub(old_gauge_weight)
                    .ok_or(Error::GaugeControllerUnderFlow8)
                    .unwrap_or_revert(),
            )
            .ok_or(Error::GaugeControllerOverFlow9)
            .unwrap_or_revert();
        let mut point_sum: Point = self.points_sum(gauge_type, next_time);
        point_sum.bias = new_sum;
        PointsSum::instance().set(&gauge_type, &next_time, point_sum);
        TimeSum::instance().set(&U256::from(gauge_type.as_u128()), next_time);
        let _total_weight = _total_weight
            .checked_add(
                (new_sum * type_weight)
                    .checked_sub(old_sum * type_weight)
                    .ok_or(Error::GaugeControllerUnderFlow9)
                    .unwrap_or_revert(),
            )
            .ok_or(Error::GaugeControllerOverFlow10)
            .unwrap_or_revert();
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
        self._get_total(); // Also calculates get_sum
        return self._gauge_relative_weight(addr, U256::from(u64::from(runtime::get_blocktime())));
    }

    fn change_type_weight(&mut self, type_id: U128, weight: U256) {
        if self.get_caller() == self.admin() {
            self._change_type_weight(type_id, weight);
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin1);
        }
    }

    fn change_gauge_weight(&mut self, addr: Key, weight: U256) {
        if self.get_caller() == self.admin() {
            self._change_gauge_weight(addr, weight);
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin2);
        }
    }

    fn get_gauge_weight(&mut self, addr: Key) -> U256 {
        let time_weight = self.time_weight(addr);
        return self.points_weight(addr, time_weight).bias;
    }

    fn get_type_weight(&mut self, type_id: U128) -> U256 {
        let time_type_weight = self.time_type_weight(U256::from(type_id.as_u128()));
        return self.points_type_weight(type_id, time_type_weight);
    }

    fn get_total_weight(&mut self) -> U256 {
        let time_total = self.time_total();
        return self.points_total(time_total);
    }

    fn get_weights_sum_per_type(&mut self, type_id: U128) -> U256 {
        let total_sum = self.time_sum(U256::from(type_id.as_u128()));
        return self.points_sum(type_id, total_sum).bias;
    }

    fn change_sum(&mut self, key0: U128, key1: U256) -> U256 {
        ChangeSum::instance().get(&key0, &key1)
    }
    fn changes_weight(&mut self, key0: Key, key1: U256) -> U256 {
        ChangesWeight::instance().get(&key0, &key1)
    }
    fn gauge_type_names(&mut self, key0: U128) -> String {
        GaugeTypeNames::instance().get(&key0)
    }
    fn gauge_types_(&mut self, key0: Key) -> U128 {
        GaugeTypes_::instance().get(&key0)
    }
    fn gauges(&mut self, key0: U256) -> Key {
        Gauges::instance().get(&key0)
    }
    fn last_user_vote(&mut self, key0: Key, key1: Key) -> U256 {
        LastUserVote::instance().get(&key0, &key1)
    }
    fn points_sum(&mut self, key0: U128, key1: U256) -> Point {
        PointsSum::instance().get(&key0, &key1)
    }
    fn points_total(&mut self, key0: U256) -> U256 {
        PointsTotal::instance().get(&key0)
    }
    fn points_type_weight(&mut self, key0: U128, key1: U256) -> U256 {
        PointsTypeWeight::instance().get(&key0, &key1)
    }
    fn points_weight(&mut self, key0: Key, key1: U256) -> Point {
        PointsWeight::instance().get(&key0, &key1)
    }
    fn time_sum(&mut self, type_id: U256) -> U256 {
        TimeSum::instance().get(&type_id)
    }
    fn time_type_weight(&mut self, type_id: U256) -> U256 {
        TimeTypeWeight::instance().get(&type_id)
    }
    fn time_weight(&mut self, key0: Key) -> U256 {
        TimeWeight::instance().get(&key0)
    }
    fn vote_user_power(&mut self, key0: Key) -> U256 {
        VoteUserPower::instance().get(&key0)
    }
    fn vote_user_slopes(&mut self, key0: Key, key1: Key) -> VotedSlope {
        VoteUserSlopes::instance().get(&key0, &key1)
    }
    // TimeWeight, VoteUserPower, VoteUserSlopes,

    fn time_total(&mut self) -> U256 {
        data::time_total()
    }
    fn token(&mut self) -> Key {
        data::token()
    }
    fn admin(&mut self) -> Key {
        data::admin()
    }
    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }
    fn voting_escrow(&mut self) -> Key {
        data::voting_escrow()
    }
    fn n_gauge_types(&mut self) -> U128 {
        data::n_gauge_types()
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
        };

        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }

    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
