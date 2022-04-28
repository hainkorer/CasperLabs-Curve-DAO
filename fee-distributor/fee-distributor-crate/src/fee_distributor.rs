use crate::{data::*, error::Error, event::FeeDistributorEvent};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::ToString};
use casper_contract::{
    contract_api::{
        runtime::{self, get_blocktime},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait FEEDISTRIBUTOR<Storage: ContractStorage>: ContractContext<Storage> {
    /// @notice Contract constructor
    /// @param _voting_escrow VotingEscrow contract address
    /// @param _start_time Epoch time for fee distribution to start
    /// @param _token Fee token address (3CRV)
    /// @param _admin Admin address
    /// @param _emergency_return Address to transfer `_token` balance to if this contract is killed
    fn init(
        &self,
        voting_escrow: Key,
        start_time: U256,
        token: Key,
        admin: Key,
        emergency_return: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        TimeCursorOf::init();
        UserEpochOf::init();
        TokensPerWeek::init();
        VeSupply::init();
        let t: U256 = start_time
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        set_start_time(t);
        set_last_token_time(t);
        set_time_cursor(t);
        set_token(token);
        set_voting_escrow(voting_escrow);
        set_admin(admin);
        set_emergency_return(emergency_return);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    fn _checkpoint_token(&self) {
        let token_balance: U256 = runtime::call_versioned_contract(
            get_token().into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Key::from(get_package_hash())
            },
        );
        let to_distribute: U256 = token_balance
            .checked_sub(get_token_last_balance())
            .unwrap_or_revert();
        set_token_last_balance(token_balance);
        let mut t: U256 = get_last_token_time();
        let since_last: U256 = U256::from(u64::from(get_blocktime()))
            .checked_sub(t)
            .unwrap_or_revert();
        set_last_token_time(U256::from(u64::from(get_blocktime())));
        let mut this_week: U256 = t
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        let mut next_week: U256 = 0.into();
        for _ in 0..20 {
            next_week = this_week.checked_add(WEEK).unwrap_or_revert();
            if U256::from(u64::from(get_blocktime())) < next_week {
                if since_last == 0.into() && U256::from(u64::from(get_blocktime())) == t {
                    TokensPerWeek::instance().set(
                        &this_week,
                        TokensPerWeek::instance()
                            .get(&this_week)
                            .checked_add(to_distribute)
                            .unwrap_or_revert(),
                    );
                } else {
                    TokensPerWeek::instance().set(
                        &this_week,
                        TokensPerWeek::instance()
                            .get(&this_week)
                            .checked_add(to_distribute)
                            .unwrap_or_revert()
                            .checked_mul(
                                U256::from(u64::from(get_blocktime()))
                                    .checked_sub(t)
                                    .unwrap_or_revert(),
                            )
                            .unwrap_or_revert()
                            .checked_div(since_last)
                            .unwrap_or_revert(),
                    );
                }
                break;
            } else {
                if since_last == 0.into() && next_week == t {
                    TokensPerWeek::instance().set(
                        &this_week,
                        TokensPerWeek::instance()
                            .get(&this_week)
                            .checked_add(to_distribute)
                            .unwrap_or_revert(),
                    );
                } else {
                    TokensPerWeek::instance().set(
                        &this_week,
                        TokensPerWeek::instance()
                            .get(&this_week)
                            .checked_add(to_distribute)
                            .unwrap_or_revert()
                            .checked_mul(next_week.checked_sub(t).unwrap_or_revert())
                            .unwrap_or_revert()
                            .checked_div(since_last)
                            .unwrap_or_revert(),
                    );
                }
            }
            t = next_week;
            this_week = next_week;
        }
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::CheckpointToken {
                time: U256::from(u64::from(get_blocktime())),
                tokens: to_distribute,
            },
        );
    }

    /// @notice Update the token checkpoint
    /// @dev Calculates the total number of tokens to be distributed in a given week.
    ///     During setup for the initial distribution this function is only callable
    ///     by the contract owner. Beyond initial distro, it can be enabled for anyone to call.
    fn checkpoint_token(&self) {
        if !((self.get_caller() == get_admin())
            || (get_can_checkpoint_token()
                && (U256::from(u64::from(get_blocktime()))
                    > get_last_token_time()
                        .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                        .unwrap_or_revert())))
        {
            runtime::revert(ApiError::from(Error::InvalidTokenCheckpointUpdate))
        }
        self._checkpoint_token();
    }

    fn _find_timestamp_epoch(&self, ve: Key, timestamp: U256) -> U256 {
        let mut min: U256 = 0.into();
        let mut max: U256 = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "epoch",
            runtime_args! {},
        );
        for _ in 0..128 {
            if min >= max {
                break;
            }
            let mid: U256 = (min
                .checked_add(max)
                .unwrap_or_revert()
                .checked_add(2.into())
                .unwrap_or_revert())
            .checked_div(2.into())
            .unwrap_or_revert();
            let pt: Point = runtime::call_versioned_contract(
                ve.into_hash().unwrap_or_revert().into(),
                None,
                "point_history",
                runtime_args! {
                    "key" => mid
                },
            );
            if pt.ts <= timestamp {
                min = mid;
            } else {
                max = mid.checked_sub(1.into()).unwrap_or_revert();
            }
        }
        min
    }

    fn _find_timestamp_user_epoch(
        &self,
        ve: Key,
        user: Key,
        timestamp: U256,
        max_user_epoch: U256,
    ) -> U256 {
        let mut min: U256 = 0.into();
        let mut max: U256 = max_user_epoch;
        for _ in 0..128 {
            if min >= max {
                break;
            }
            let mid: U256 = (min
                .checked_add(max)
                .unwrap_or_revert()
                .checked_add(2.into()))
            .unwrap_or_revert()
            .checked_div(2.into())
            .unwrap_or_revert();
            let pt: Point = runtime::call_versioned_contract(
                ve.into_hash().unwrap_or_revert().into(),
                None,
                "user_point_history",
                runtime_args! {
                    "key1" => user,
                    "key2" => mid
                },
            );
            if pt.ts <= timestamp {
                min = mid;
            } else {
                max = mid.checked_sub(1.into()).unwrap_or_revert();
            }
        }
        min
    }

    /// @notice Get the veCRV balance for `_user` at `_timestamp`
    /// @param _user Address to query balance for
    /// @param _timestamp Epoch time
    /// @return uint256 veCRV balance
    fn ve_for_at(&self, user: Key, timestamp: U256) -> U256 {
        let ve: Key = get_voting_escrow();
        let max_user_epoch: U256 = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "key" => user
            },
        );
        let epoch: U256 = self._find_timestamp_user_epoch(ve, user, timestamp, max_user_epoch);
        let pt: Point = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history",
            runtime_args! {
                "key1" => user,
                "key2" => epoch
            },
        );
        U256::max(
            (pt.bias
                .checked_sub(pt.slope)
                .unwrap_or_revert()
                .checked_mul(
                    timestamp
                        .checked_sub(pt.ts.as_u128().into())
                        .unwrap_or_revert()
                        .as_u128()
                        .into(),
                )
                .unwrap_or_revert())
            .as_u128()
            .into(),
            0.into(),
        )
    }

    fn _checkpoint_total_supply(&self) {
        let ve: Key = get_voting_escrow();
        let mut t: U256 = get_time_cursor();
        let rounded_timestamp: U256 = U256::from(u64::from(get_blocktime()))
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        let () = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "checkpoint",
            runtime_args! {},
        );
        for _ in 0..20 {
            if t > rounded_timestamp {
                break;
            } else {
                let epoch: U256 = self._find_timestamp_epoch(ve, t);
                let pt: Point = runtime::call_versioned_contract(
                    ve.into_hash().unwrap_or_revert().into(),
                    None,
                    "point_history",
                    runtime_args! {
                        "key" => epoch
                    },
                );
                let mut dt: U128 = 0.into();
                if t > pt.ts {
                    // If the point is at 0 epoch, it can actually be earlier than the first deposit
                    // Then make dt 0
                    dt = t.checked_sub(pt.ts).unwrap_or_revert().as_u128().into();
                }
                VeSupply::instance().set(
                    &t,
                    U128::max(
                        pt.bias
                            .checked_sub(pt.slope)
                            .unwrap_or_revert()
                            .checked_mul(dt)
                            .unwrap_or_revert(),
                        0.into(),
                    )
                    .as_u128()
                    .into(),
                );
            }
            t = t.checked_add(WEEK).unwrap_or_revert();
        }
        set_time_cursor(t);
    }

    /// @notice Update the veCRV total supply checkpoint
    /// @dev The checkpoint is also updated by the first claimant each
    ///     new epoch week. This function may be called independently
    ///     of a claim, to reduce claiming gas costs.
    fn checkpoint_total_supply(&self) {
        self._checkpoint_total_supply();
    }

    fn _claim(&self, addr: Key, ve: Key, last_token_time: U256) -> U256 {
        // Minimal user_epoch is 0 (if user had no point)
        let mut user_epoch: U256 = 0.into();
        let mut to_distribute: U256 = 0.into();
        let max_user_epoch: U256 = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_epoch",
            runtime_args! {
                "key" => addr
            },
        );
        let start_time: U256 = get_start_time();
        if max_user_epoch == 0.into() {
            // No lock = no fees
            return 0.into();
        }
        let mut week_cursor: U256 = TimeCursorOf::instance().get(&addr);
        if week_cursor == 0.into() {
            // Need to do the initial binary search
            user_epoch = self._find_timestamp_user_epoch(ve, addr, start_time, max_user_epoch);
        } else {
            user_epoch = UserEpochOf::instance().get(&addr);
        }
        if user_epoch == 0.into() {
            user_epoch = 1.into();
        }
        let mut user_point: Point = runtime::call_versioned_contract(
            ve.into_hash().unwrap_or_revert().into(),
            None,
            "user_point_history",
            runtime_args! {
                "key1" => addr,
                "key2" => user_epoch
            },
        );
        if week_cursor == 0.into() {
            week_cursor = (user_point
                .ts
                .checked_add(WEEK)
                .unwrap_or_revert()
                .checked_sub(1.into()))
            .unwrap_or_revert()
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        }
        if week_cursor >= last_token_time {
            return 0.into();
        }
        if week_cursor < start_time {
            week_cursor = start_time;
        }
        let mut old_user_point: Point = Point::default();
        // Iterate over weeks
        for _ in 0..50 {
            if week_cursor >= last_token_time {
                break;
            }
            if week_cursor >= user_point.ts && user_epoch <= max_user_epoch {
                user_epoch = user_epoch.checked_add(1.into()).unwrap_or_revert();
                old_user_point = user_point;
                if user_epoch > max_user_epoch {
                    user_point = Point::default();
                } else {
                    user_point = runtime::call_versioned_contract(
                        ve.into_hash().unwrap_or_revert().into(),
                        None,
                        "user_point_history",
                        runtime_args! {
                            "key1" => addr,
                            "key2" => user_epoch
                        },
                    );
                }
            } else {
                // Calc
                // + i * 2 is for rounding errors
                let dt: U128 = week_cursor
                    .checked_sub(old_user_point.ts)
                    .unwrap_or_revert()
                    .as_u128()
                    .into();
                let balance_of: U256 = U256::max(
                    old_user_point
                        .bias
                        .checked_sub(dt)
                        .unwrap_or_revert()
                        .checked_mul(old_user_point.slope)
                        .unwrap_or_revert()
                        .as_u128()
                        .into(),
                    0.into(),
                );
                if balance_of == 0.into() && user_epoch > max_user_epoch {
                    break;
                }
                if balance_of > 0.into() {
                    to_distribute = to_distribute
                        .checked_add(
                            balance_of
                                .checked_mul(
                                    TokensPerWeek::instance()
                                        .get(&week_cursor)
                                        .checked_div(VeSupply::instance().get(&week_cursor))
                                        .unwrap_or_revert(),
                                )
                                .unwrap_or_revert(),
                        )
                        .unwrap_or_revert();
                }
                week_cursor = week_cursor.checked_add(WEEK).unwrap_or_revert();
            }
        }
        user_epoch = U256::min(
            max_user_epoch,
            user_epoch.checked_sub(1.into()).unwrap_or_revert(),
        );
        UserEpochOf::instance().set(&addr, user_epoch);
        TimeCursorOf::instance().set(&addr, week_cursor);
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::Claimed {
                recipient: addr,
                amount: to_distribute,
                claim_epoch: user_epoch,
                max_epoch: max_user_epoch,
            },
        );
        to_distribute
    }

    /// @notice Claim fees for `_addr`
    /// @dev Each call to claim look at a maximum of 50 user veCRV points.
    ///     For accounts with many veCRV related actions, this function
    ///     may need to be called more than once to claim all available
    ///     fees. In the `Claimed` event that fires, if `claim_epoch` is
    ///     less than `max_epoch`, the account may claim again.
    /// @param _addr Address to claim fees for
    /// @return uint256 Amount of fees claimed in the call
    fn claim(&self, addr: Key /*self.get_caller()*/) -> U256 {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::Killed));
        }
        if U256::from(u64::from(get_blocktime())) >= get_time_cursor() {
            self._checkpoint_total_supply();
        }
        let mut last_token_time: U256 = get_last_token_time();
        if get_can_checkpoint_token()
            && (U256::from(u64::from(get_blocktime()))
                > last_token_time
                    .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                    .unwrap_or_revert())
        {
            self._checkpoint_token();
            set_last_token_time(U256::from(u64::from(get_blocktime())));
        }
        last_token_time = last_token_time
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        let amount: U256 = self._claim(addr, get_voting_escrow(), last_token_time);
        if amount != 0.into() {
            let token: Key = get_token();
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                token.into_hash().unwrap_or_revert().into(),
                None,
                "transfer",
                runtime_args! {
                    "key1" => addr,
                    "key2" => amount
                },
            );
            if ret.is_err() {
                runtime::revert(ApiError::User(ret.err().unwrap() as u16));
            }
            set_token_last_balance(
                get_token_last_balance()
                    .checked_sub(amount)
                    .unwrap_or_revert(),
            );
        }
        set_lock(false);
        amount
    }

    /// @notice Make multiple fee claims in a single call
    /// @dev Used to claim for many accounts at once, or to make
    ///     multiple claims for the same address when that address
    ///     has significant veCRV history
    /// @param _receivers List of addresses to claim for. Claiming terminates at the first `ZERO_ADDRESS`.
    /// @return bool success
    fn claim_many(&self, receivers: Vec<Key>) -> bool {
        if get_lock() {
            runtime::revert(ApiError::from(Error::IsLocked));
        }
        set_lock(true);
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::Killed));
        }
        if U256::from(u64::from(get_blocktime())) >= get_time_cursor() {
            self._checkpoint_total_supply();
        }
        let mut last_token_time: U256 = get_last_token_time();
        if get_can_checkpoint_token()
            && (U256::from(u64::from(get_blocktime()))
                > last_token_time
                    .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                    .unwrap_or_default())
        {
            self._checkpoint_token();
            last_token_time = U256::from(u64::from(get_blocktime()));
        }
        last_token_time = last_token_time
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        let voting_escrow: Key = get_voting_escrow();
        let token: Key = get_token();
        let mut total: U256 = 0.into();
        for addr in receivers {
            if addr == zero_address() {
                break;
            }
            let amount: U256 = self._claim(addr, voting_escrow, last_token_time);
            if amount != 0.into() {
                let ret: Result<(), u32> = runtime::call_versioned_contract(
                    token.into_hash().unwrap_or_revert().into(),
                    None,
                    "transfer",
                    runtime_args! {
                        "recipient" => addr,
                        "amount" => amount
                    },
                );
                if ret.is_err() {
                    runtime::revert(ApiError::User(ret.err().unwrap() as u16));
                }
                total = total.checked_add(amount).unwrap_or_revert();
            }
        }
        if total != 0.into() {
            set_token_last_balance(
                get_token_last_balance()
                    .checked_sub(total)
                    .unwrap_or_revert(),
            );
        }
        set_lock(false);
        true
    }

    /// @notice Receive 3CRV into the contract and trigger a token checkpoint
    /// @param _coin Address of the coin being received (must be 3CRV)
    /// @return bool success
    fn burn(&self, coin: Key) -> bool {
        if !(coin == get_token()) {
            runtime::revert(ApiError::from(Error::InvalidCoin));
        }
        if get_is_killed() {
            runtime::revert(ApiError::from(Error::Killed));
        }
        let amount: U256 = runtime::call_versioned_contract(
            coin.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => self.get_caller()
            },
        );
        if amount != 0.into() {
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                coin.into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => self.get_caller(),
                    "recipient" => Key::from(get_package_hash()),
                    "amount" => amount
                },
            );
            if ret.is_err() {
                runtime::revert(ApiError::User(ret.err().unwrap() as u16));
            }
            if get_can_checkpoint_token()
                && (U256::from(u64::from(get_blocktime()))
                    > get_last_token_time()
                        .checked_add(TOKEN_CHECKPOINT_DEADLINE)
                        .unwrap_or_revert())
            {
                self._checkpoint_token();
            }
        }
        true
    }

    /// @notice Commit transfer of ownership
    /// @param _addr New admin address
    fn commit_admin(&self, addr: Key) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::AccessDenied));
        }
        set_future_admin(addr);
        FEEDISTRIBUTOR::emit(self, &FeeDistributorEvent::CommitAdmin { admin: addr });
    }

    /// @notice Apply transfer of ownership
    fn apply_admin(&self) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::InvalidAdmin));
        }
        if !(get_future_admin() != zero_address()) {
            runtime::revert(ApiError::from(Error::ZeroFutureAdmin));
        }
        let future_admin: Key = get_future_admin();
        set_admin(future_admin);
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::ApplyAdmin {
                admin: future_admin,
            },
        );
    }

    /// @notice Toggle permission for checkpointing by any account
    fn toggle_allow_checkpoint_token(&self) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::InvalidAdmin));
        }
        let flag: bool = !get_can_checkpoint_token();
        set_can_checkpoint_token(flag);
        FEEDISTRIBUTOR::emit(
            self,
            &FeeDistributorEvent::ToggleAllowCheckpointToken { toggle_flag: flag },
        );
    }

    /// @notice Kill the contract
    /// @dev Killing transfers the entire 3CRV balance to the emergency return address
    ///     and blocks the ability to claim or burn. The contract cannot be unkilled.
    fn kill_me(&self) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::InvalidAdmin));
        }
        set_is_killed(true);
        let token: Key = get_token();
        let balance: U256 = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Key::from(get_package_hash())
            },
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            token.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => get_emergency_return(),
                "amount" => balance
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::User(ret.err().unwrap() as u16));
        }
    }

    /// @notice Recover ERC20 tokens from this contract
    /// @dev Tokens are sent to the emergency return address.
    /// @param _coin Token address
    /// @return bool success
    fn recover_balance(&self, coin: Key) -> bool {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::InvalidAdmin));
        }
        if !(coin == get_token()) {
            runtime::revert(ApiError::from(Error::InvalidCoin));
        }
        let amount: U256 = runtime::call_versioned_contract(
            coin.into_hash().unwrap_or_revert().into(),
            None,
            "balance_of",
            runtime_args! {
                "owner" => Key::from(get_package_hash())
            },
        );
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            coin.into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => get_emergency_return(),
                "amount" => amount
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::User(ret.err().unwrap() as u16));
        }
        true
    }

    fn emit(&self, fee_distributor_event: &FeeDistributorEvent) {
        let mut events = Vec::new();
        let tmp = get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match fee_distributor_event {
            FeeDistributorEvent::CommitAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            FeeDistributorEvent::ApplyAdmin { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            FeeDistributorEvent::ToggleAllowCheckpointToken { toggle_flag } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("toggle_flag", toggle_flag.to_string());
                events.push(event);
            }
            FeeDistributorEvent::CheckpointToken { time, tokens } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("time", time.to_string());
                event.insert("tokens", tokens.to_string());
                events.push(event);
            }
            FeeDistributorEvent::Claimed {
                recipient,
                amount,
                claim_epoch,
                max_epoch,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", fee_distributor_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("amount", amount.to_string());
                event.insert("claim_epoch", claim_epoch.to_string());
                event.insert("max_epoch", max_epoch.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
