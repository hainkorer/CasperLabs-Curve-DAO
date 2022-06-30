use crate::{data::*, event::VotingEscrowEvent};
use alloc::vec::Vec;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
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
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::errors::*;
use common::keys::*;

/// @notice Votes have a weight depending on time, so that users are committed to the future of (whatever they are voting for)
/// @dev Vote weight decays linearly over time. Lock time cannot be more than `MAXTIME` (4 years).
///
/// Voting escrow to have time-weighted votes
/// Votes have a weight depending on time, so that users are committed
/// to the future of (whatever they are voting for).
/// The weight in this implementation is linear, and lock cannot be more than maxtime:
/// w ^
/// 1 +        /
///   |      /
///   |    /
///   |  /
///   |/
/// 0 +--------+------> time
///       maxtime (4 years?)

pub trait VOTINGESCROW<Storage: ContractStorage>: ContractContext<Storage> {
    /// @notice Contract constructor
    /// @param token_addr `ERC20CRV` token address
    /// @param _name Token name
    /// @param _symbol Token symbol
    /// @param _version Contract version - required for Aragon compatibility
    fn init(
        &self,
        token_addr: Key,
        name: String,
        symbol: String,
        version: String,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        Locked::init();
        UserPointHistory::init();
        UserPointEpoch::init();
        SlopeChanges::init();
        PointHistory::init();

        set_admin(self.get_caller());
        set_token(token_addr);
        let mut point_history: Point = PointHistory::instance().get(&U256::from(0));
        // self.point_history[0].blk = block.number
        point_history.ts = U256::from(u64::from(get_blocktime()));
        set_controller(self.get_caller());
        set_transfers_enabled(true);

        let decimals: u8 = runtime::call_versioned_contract(
            token_addr.into_hash().unwrap_or_revert().into(),
            None,
            "decimals",
            runtime_args! {},
        );
        let decimals: U256 = decimals.into();
        if !(decimals <= 255.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowInvalidDecimals))
        }

        set_decimals(decimals);
        set_name(name);
        set_symbol(symbol);
        set_version(version);
        set_contract_hash(contract_hash);
        set_package_hash(package_hash);
    }

    fn only_admin(&self) {
        if !(self.get_caller() == get_admin()) {
            runtime::revert(ApiError::from(Error::VotingEscrowAdminOnly));
        }
    }

    /// @notice Transfer ownership of VotingEscrow contract to `addr`
    /// @param addr Address to have ownership transferred to
    fn commit_transfer_ownership(&mut self, addr: Key) {
        self.only_admin();
        set_future_admin(addr);
        VOTINGESCROW::emit(self, &VotingEscrowEvent::CommitOwnership { admin: addr });
    }

    /// @notice Apply ownership transfer
    fn apply_transfer_ownership(&mut self) {
        self.only_admin();
        let admin: Key = get_future_admin();
        if !(admin != zero_address()) {
            runtime::revert(ApiError::from(Error::VotingEscrowZeroAddress));
        }
        set_admin(admin);
        VOTINGESCROW::emit(self, &VotingEscrowEvent::ApplyOwnership { admin });
    }

    /// @notice Get the most recently recorded rate of voting power decrease for `addr`
    /// @param addr Address of the user wallet
    /// @return Value of the slope
    fn get_last_user_slope(&self, addr: Key) -> U128 {
        let uepoch: U256 = UserPointEpoch::instance().get(&addr);
        UserPointHistory::instance().get(&addr, &uepoch).slope
    }

    /// @notice Get the timestamp for checkpoint `_idx` for `_addr`
    /// @param _addr User wallet address
    /// @param _idx User epoch number
    /// @return Epoch time of the checkpoint
    fn user_point_history_ts(&self, addr: Key, idx: U256) -> U256 {
        UserPointHistory::instance().get(&addr, &idx).ts
    }

    /// @notice Get timestamp when `_addr`'s lock finishes
    /// @param _addr User wallet
    /// @return Epoch time of the lock end
    fn locked_end(&self, addr: Key) -> U256 {
        Locked::instance().get(&addr).end
    }

    /// @notice Record global and per-user data to checkpoint
    /// @param addr User's wallet address. No user checkpoint if 0x0
    /// @param old_locked Pevious locked amount / end lock time for the user
    /// @param new_locked New locked amount / end lock time for the user
    fn _checkpoint(&self, addr: Key, old_locked: LockedBalance, new_locked: LockedBalance) {
        let mut u_old: Point = Point::default();
        let mut u_new: Point = Point::default();
        let mut old_dslope: U128 = 0.into();
        let mut new_dslope: U128 = 0.into();
        let mut epoch: U256 = get_epoch();
        if addr != zero_address() {
            //  Calculate slopes and biases
            //  Kept at zero when they have to
            if (old_locked.end > U256::from(u64::from(get_blocktime())))
                && (old_locked.amount > 0.into())
            {
                u_old.slope = old_locked
                    .amount
                    .checked_div(MAXTIME.as_u128().into())
                    .unwrap_or_revert();
                u_old.bias = u_old
                    .slope
                    .checked_mul(
                        old_locked
                            .end
                            .checked_sub(U256::from(u64::from(get_blocktime())))
                            .unwrap_or_revert()
                            .as_u128()
                            .into(),
                    )
                    .unwrap_or_revert();
            }
            if (new_locked.end > U256::from(u64::from(get_blocktime())))
                && (new_locked.amount > 0.into())
            {
                u_new.slope = new_locked
                    .amount
                    .checked_div(MAXTIME.as_u128().into())
                    .unwrap_or_revert();
                u_new.bias = u_new
                    .slope
                    .checked_mul(
                        new_locked
                            .end
                            .checked_sub(U256::from(u64::from(get_blocktime())))
                            .unwrap_or_revert()
                            .as_u128()
                            .into(),
                    )
                    .unwrap_or_revert()
            }
            // Read values of scheduled changes in the slope
            // old_locked.end can be in the past and in the future
            // new_locked.end can ONLY by in the FUTURE unless everything expired: than zeros
            old_dslope = SlopeChanges::instance().get(&old_locked.end);
            if new_locked.end != 0.into() {
                if new_locked.end == old_locked.end {
                    new_dslope = old_dslope
                } else {
                    new_dslope = SlopeChanges::instance().get(&new_locked.end);
                }
            }
        }
        let mut last_point: Point = Point {
            bias: 0.into(),
            slope: 0.into(),
            ts: U256::from(u64::from(get_blocktime())),
            // blk: block.number,
            blk: 12345.into(),
        };
        if epoch > 0.into() {
            last_point = PointHistory::instance().get(&epoch);
        }
        let mut last_checkpoint: U256 = last_point.ts;
        // initial_last_point is used for extrapolation to calculate block number
        // (approximately, for *At methods) and save them
        // as we cannot figure that out exactly from inside the contract
        let initial_last_point: Point = last_point;
        let block_slope: U256 = 0.into();
        if U256::from(u64::from(get_blocktime())) > last_point.ts {
            // block_slope =
            //     MULTIPLIER * (block.number - last_point.blk) / (block.timestamp - last_point.ts)
        }
        //  If last point is already recorded in this block, slope=0
        //  But that's ok b/c we know the block in such case
        let mut t_i: U256 = last_checkpoint
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        for _ in 0..255 {
            // Hopefully it won't happen that this won't get used in 5 years!
            // If it does, users will be able to withdraw but vote weight will be broken
            t_i = t_i.checked_add(WEEK).unwrap_or_revert();
            let mut d_slope: U128 = 0.into();
            if t_i > U256::from(u64::from(get_blocktime())) {
                t_i = U256::from(u64::from(get_blocktime()));
            } else {
                d_slope = SlopeChanges::instance().get(&t_i);
            }
            last_point.bias = last_point
                .bias
                .checked_sub(
                    last_point
                        .slope
                        .checked_mul(
                            t_i.checked_sub(last_checkpoint)
                                .unwrap_or_revert()
                                .as_u128()
                                .into(),
                        )
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert();
            last_point.slope = last_point.slope.checked_add(d_slope).unwrap_or_revert();
            if last_point.bias < 0.into() {
                // This can happen
                last_point.bias = 0.into();
            }
            if last_point.slope < 0.into() {
                // This cannot happen - just in case
                last_point.slope = 0.into();
            }
            last_checkpoint = t_i;
            last_point.ts = t_i;
            last_point.blk = initial_last_point
                .blk
                .checked_add(block_slope)
                .unwrap_or_revert()
                .checked_mul(t_i.checked_sub(initial_last_point.ts).unwrap_or_revert())
                .unwrap_or_revert()
                .checked_div(MULTIPLIER.as_u128().into())
                .unwrap_or_revert();
            epoch = epoch.checked_add(1.into()).unwrap_or_revert();
            if t_i == U256::from(u64::from(get_blocktime())) {
                // last_point.blk = block.number
                break;
            } else {
                PointHistory::instance().set(&epoch, last_point);
            }
        }
        set_epoch(epoch);
        // Now point_history is filled until t=now
        if addr != zero_address() {
            // If last point was in this block, the slope change has been applied already
            // But in such case we have 0 slope(s)
            last_point.slope = last_point
                .slope
                .checked_add(u_new.slope.checked_sub(u_old.slope).unwrap_or_revert())
                .unwrap_or_revert();
            last_point.bias = last_point
                .bias
                .checked_add(u_new.bias.checked_sub(u_old.bias).unwrap_or_revert())
                .unwrap_or_revert();
            if last_point.slope < 0.into() {
                last_point.slope = 0.into();
            }
            if last_point.bias < 0.into() {
                last_point.bias = 0.into();
            }
        }
        // Record the changed point into history
        PointHistory::instance().set(&epoch, last_point);
        if addr != zero_address() {
            // Schedule the slope changes (slope is going down)
            // We subtract new_user_slope from [new_locked.end]
            // and add old_user_slope to [old_locked.end]
            if old_locked.end > U256::from(u64::from(get_blocktime())) {
                // old_dslope was <something> - u_old.slope, so we cancel that
                old_dslope = old_dslope.checked_add(u_old.slope).unwrap_or_revert();
                if new_locked.end == old_locked.end {
                    // It was a new deposit, not extension
                    old_dslope = old_dslope.checked_sub(u_new.slope).unwrap_or_revert();
                }
                SlopeChanges::instance().set(&old_locked.end, old_dslope);
            }
            if new_locked.end > U256::from(u64::from(get_blocktime())) {
                if new_locked.end > old_locked.end {
                    new_dslope = new_dslope.checked_sub(u_new.slope).unwrap_or_revert(); // old slope disappeared at this point
                    SlopeChanges::instance().set(&new_locked.end, new_dslope);
                }
                // else: we recorded it already in old_dslope
            }
            // Now handle user history
            let user_epoch: U256 = UserPointEpoch::instance()
                .get(&addr)
                .checked_add(1.into())
                .unwrap_or_revert();
            UserPointEpoch::instance().set(&addr, user_epoch);
            u_new.ts = U256::from(u64::from(get_blocktime()));
            // u_new.blk = block.number
            UserPointHistory::instance().set(&addr, &user_epoch, u_new);
        }
    }

    /// @notice Deposit and lock tokens for a user
    /// @param _addr User's wallet address
    /// @param _value Amount to deposit
    /// @param unlock_time New time when to unlock the tokens, or 0 if unchanged
    /// @param locked_balance Previous locked amount / timestamp
    fn _deposit_for(
        &mut self,
        addr: Key,
        value: U256,
        unlock_time: U256,
        locked_balance: LockedBalance,
        _type: U128,
    ) {
        let mut locked: LockedBalance = locked_balance;
        let supply_before: U256 = get_supply();
        set_supply(supply_before.checked_add(value).unwrap_or_revert());
        let old_locked: LockedBalance = locked;
        // Adding to existing lock, or if a lock is expired - creating a new one
        locked.amount = locked
            .amount
            .checked_add(value.as_u128().into())
            .unwrap_or_revert();
        if unlock_time != 0.into() {
            locked.end = unlock_time;
        }
        Locked::instance().set(&addr, locked);
        // Possibilities:
        // Both old_locked.end could be current or expired (>/< block.timestamp)
        // value == 0 (extend lock) or value > 0 (add to lock or extend lock)
        // _locked.end > block.timestamp (always)
        self._checkpoint(addr, old_locked, locked);
        if value != 0.into() {
            let ret: Result<(), u32> = runtime::call_versioned_contract(
                get_token().into_hash().unwrap_or_revert().into(),
                None,
                "transfer_from",
                runtime_args! {
                    "owner" => addr,
                    "recipient" => Key::from(get_package_hash()),
                    "amount" => value
                },
            );
            if ret.is_err() {
                runtime::revert(ApiError::from(ret.err().unwrap_or_revert()));
            }
        }
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Deposit {
                provider: addr,
                value,
                locktime: locked.end,
                _type,
                ts: U256::from(u64::from(get_blocktime())),
            },
        );
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Supply {
                prev_supply: supply_before,
                supply: supply_before.checked_add(value).unwrap_or_revert(),
            },
        );
    }

    /// @notice Record global data to checkpoint
    fn checkpoint(&self) {
        self._checkpoint(
            zero_address(),
            LockedBalance::default(),
            LockedBalance::default(),
        )
    }

    /// @notice Deposit `_value` tokens for `_addr` and add to the lock
    /// @dev Anyone (even a smart contract) can deposit for someone else, but cannot extend their locktime and deposit for a brand new user
    /// @param _addr User's wallet address
    /// @param _value Amount to add to user's lock
    fn deposit_for(&mut self, addr: Key, value: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked1));
        }
        set_lock(true);
        let locked: LockedBalance = Locked::instance().get(&addr);
        if !(value > 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowNeedNonZeroValue1));
        }
        if !(locked.amount > 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowNoExistingLockFound1));
        }
        if !(locked.end > U256::from(u64::from(get_blocktime()))) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCannotAddToExpiredLockWithdraw1,
            ));
        }
        self._deposit_for(
            addr,
            value,
            0.into(),
            Locked::instance().get(&addr),
            DEPOSIT_FOR_TYPE,
        );
        set_lock(false);
    }

    /// @notice Deposit `_value` tokens for `self.get_caller()` and lock until `_unlock_time`
    /// @param _value Amount to deposit
    /// @param _unlock_time Epoch time when tokens unlock, rounded down to whole weeks
    fn create_lock(&mut self, value: U256, unlock_time: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked2));
        }
        set_lock(true);
        let unlock_time: U256 = unlock_time
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert(); // Locktime is rounded down to weeks
        let locked: LockedBalance = Locked::instance().get(&self.get_caller());
        if !(value > 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowNeedNonZeroValue2));
        }
        if !(locked.amount == 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowWithdrawOldTokensFirst));
        }
        if !(unlock_time > U256::from(u64::from(get_blocktime()))) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCanOnlyLockUntilTimeInTheFuture,
            ));
        }
        if !(unlock_time
            <= U256::from(u64::from(get_blocktime()))
                .checked_add(MAXTIME)
                .unwrap_or_revert())
        {
            runtime::revert(ApiError::from(Error::VotingEscrowVotingLockCanBe4YearsMax1));
        }
        self._deposit_for(
            self.get_caller(),
            value,
            unlock_time,
            locked,
            CREATE_LOCK_TYPE,
        );
        set_lock(false);
    }

    /// @notice Deposit `_value` additional tokens for `self.get_caller()` without modifying the unlock time
    /// @param _value Amount of tokens to deposit and add to the lock
    fn increase_amount(&mut self, value: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked3));
        }
        set_lock(true);
        let locked: LockedBalance = Locked::instance().get(&self.get_caller());
        if !(value > 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowNeedNonZeroValue3));
        }
        if !(locked.amount > 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowNoExistingLockFound2));
        }
        if !(locked.end > U256::from(u64::from(get_blocktime()))) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCannotAddToExpiredLockWithdraw2,
            ));
        }
        self._deposit_for(
            self.get_caller(),
            value,
            0.into(),
            locked,
            INCREASE_LOCK_AMOUNT,
        );
        set_lock(false);
    }

    /// @notice Extend the unlock time for `self.get_caller()` to `_unlock_time`
    /// @param _unlock_time New epoch time for unlocking
    fn increase_unlock_time(&mut self, unlock_time: U256) {
        if get_lock() {
            runtime::revert(ApiError::from(Error::VotingEscrowIsLocked4));
        }
        set_lock(true);
        let locked: LockedBalance = Locked::instance().get(&self.get_caller());
        let unlock_time: U256 = unlock_time
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert(); // Locktime is rounded down to weeks
        if !(locked.end > U256::from(u64::from(get_blocktime()))) {
            runtime::revert(ApiError::from(Error::VotingEscrowLockExpired));
        }
        if !(locked.amount > 0.into()) {
            runtime::revert(ApiError::from(Error::VotingEscrowNothingIsLocked));
        }
        if !(unlock_time > locked.end) {
            runtime::revert(ApiError::from(
                Error::VotingEscrowCanOnlyIncreaseLockDuration,
            ));
        }
        if !(unlock_time
            <= U256::from(u64::from(get_blocktime()))
                .checked_add(MAXTIME)
                .unwrap_or_revert())
        {
            runtime::revert(ApiError::from(Error::VotingEscrowVotingLockCanBe4YearsMax2));
        }
        self._deposit_for(
            self.get_caller(),
            0.into(),
            unlock_time,
            locked,
            INCREASE_UNLOCK_TIME,
        );
        set_lock(false);
    }

    /// @notice Withdraw all tokens for `self.get_caller()`
    /// @dev Only possible if the lock has expired
    fn withdraw(&mut self) {
        let mut locked: LockedBalance = Locked::instance().get(&self.get_caller());
        if !(U256::from(u64::from(get_blocktime())) >= locked.end) {
            runtime::revert(ApiError::from(Error::VotingEscrowTheLockDidntExpire));
        }
        let value: U256 = locked.amount.as_u128().into();
        let old_locked: LockedBalance = locked;
        locked.end = 0.into();
        locked.amount = 0.into();
        Locked::instance().set(&self.get_caller(), locked);
        let supply_before: U256 = get_supply();
        set_supply(supply_before.checked_sub(value).unwrap_or_revert());
        // old_locked can have either expired <= timestamp or zero end
        // _locked has only 0 end
        // Both can have >= 0 amount
        self._checkpoint(self.get_caller(), old_locked, locked);
        let ret: Result<(), u32> = runtime::call_versioned_contract(
            get_token().into_hash().unwrap_or_revert().into(),
            None,
            "transfer",
            runtime_args! {
                "recipient" => self.get_caller(),
                "amount" => value
            },
        );
        if ret.is_err() {
            runtime::revert(ApiError::from(ret.err().unwrap_or_revert()));
        }
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Withdraw {
                provider: self.get_caller(),
                value,
                ts: U256::from(u64::from(get_blocktime())),
            },
        );
        VOTINGESCROW::emit(
            self,
            &VotingEscrowEvent::Supply {
                prev_supply: supply_before,
                supply: supply_before.checked_sub(value).unwrap_or_revert(),
            },
        );
    }

    /// The following ERC20/minime-compatible methods are not real balanceOf and supply!
    /// They measure the weights for the purpose of voting, so they don't represent real coins.
    /// @notice Binary search to estimate timestamp for block number
    /// @param _block Block to find
    /// @param max_epoch Don't go beyond this epoch
    /// @return Approximate timestamp for block
    fn _find_block_epoch(&self, block: U256, max_epoch: U256) -> U256 {
        // Binary search
        let mut min: U256 = 0.into();
        let mut max: U256 = max_epoch;
        for _ in 0..128 {
            // Will be always enough for 128-bit numbers
            if min >= max {
                break;
            }
            let mid: U256 = min
                .checked_add(max)
                .unwrap_or_revert()
                .checked_add(1.into())
                .unwrap_or_revert()
                .checked_div(2.into())
                .unwrap_or_revert();
            if PointHistory::instance().get(&mid).blk <= block {
                min = mid;
            } else {
                max = mid.checked_sub(1.into()).unwrap_or_revert();
            }
        }
        min
    }

    /// @notice Get the current voting power for `self.get_caller()`
    /// @dev Adheres to the ERC20 `balanceOf` interface for Aragon compatibility
    /// @param addr User wallet address
    /// @param _t Epoch time to return voting power at
    /// @return User voting power
    fn balance_of(&self, addr: Key, t: Option<U256>) -> U256 {
        let t: U256 = match t {
            Some(val) => val,
            None => {
                let blocktime: u64 = runtime::get_blocktime().into();
                blocktime.into()
            }
        };

        let epoch: U256 = UserPointEpoch::instance().get(&addr);
        if epoch == 0.into() {
            0.into()
        } else {
            let mut last_point: Point = UserPointHistory::instance().get(&addr, &epoch);
            last_point.bias = last_point
                .bias
                .checked_sub(
                    last_point
                        .slope
                        .checked_mul(
                            t.checked_sub(last_point.ts)
                                .unwrap_or_revert()
                                .as_u128()
                                .into(),
                        )
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert();
            if last_point.bias < 0.into() {
                last_point.bias = 0.into();
            }
            last_point.bias.as_u128().into()
        }
    }

    /// @notice Measure voting power of `addr` at block height `_block`
    /// @dev Adheres to MiniMe `balanceOfAt` interface: https://github.com/Giveth/minime
    /// @param addr User's wallet address
    /// @param _block Block to calculate the voting power at
    /// @return Voting power
    #[allow(unused_assignments)]
    fn balance_of_at(&self, addr: Key, block: U256) -> U256 {
        // Copying and pasting totalSupply code because Vyper cannot pass by reference yet
        // if !(block <= block.number) {
        //     runtime::revert(ApiError::from(Error::VotingEscrowInvalidBlockNumber));
        // }
        // Binary search
        let mut min: U256 = 0.into();
        let mut max: U256 = UserPointEpoch::instance().get(&addr);
        for _ in 0..128 {
            // Will be always enough for 128-bit numbers
            if min >= max {
                break;
            }
            let mid: U256 = min
                .checked_add(max)
                .unwrap_or_revert()
                .checked_add(1.into())
                .unwrap_or_revert()
                .checked_add(2.into())
                .unwrap_or_revert();
            if UserPointHistory::instance().get(&addr, &mid).blk <= block {
                min = mid;
            } else {
                max = mid.checked_sub(1.into()).unwrap_or_revert();
            }
        }
        let mut upoint: Point = UserPointHistory::instance().get(&addr, &min);
        let max_epoch: U256 = get_epoch();
        let epoch: U256 = self._find_block_epoch(block, max_epoch);
        let point_0: Point = PointHistory::instance().get(&epoch);
        let mut d_block: U256 = 0.into();
        let mut d_t: U256 = 0.into();
        if epoch < max_epoch {
            let point_1: Point =
                PointHistory::instance().get(&epoch.checked_add(1.into()).unwrap_or_revert());
            d_block = point_1.blk.checked_sub(point_0.blk).unwrap_or_revert();
            d_t = point_1.ts - point_0.ts;
        } else {
            // d_block = block.number - point_0.blk
            d_t = U256::from(u64::from(get_blocktime()))
                .checked_sub(point_0.ts)
                .unwrap_or_revert();
        }
        let mut block_time: U256 = point_0.ts;
        if d_block != 0.into() {
            block_time = block_time
                .checked_add(
                    d_t.checked_mul(block.checked_sub(point_0.blk).unwrap_or_revert())
                        .unwrap_or_revert()
                        .checked_div(d_block)
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert();
        }
        upoint.bias = upoint
            .bias
            .checked_sub(
                upoint
                    .slope
                    .checked_mul(
                        block_time
                            .checked_sub(upoint.ts)
                            .unwrap_or_revert()
                            .as_u128()
                            .into(),
                    )
                    .unwrap_or_revert(),
            )
            .unwrap_or_revert();
        if upoint.bias >= 0.into() {
            upoint.bias.as_u128().into()
        } else {
            0.into()
        }
    }

    /// @notice Calculate total voting power at some point in the past
    /// @param point The point (bias/slope) to start search from
    /// @param t Time to calculate the total voting power at
    /// @return Total voting power at that time
    fn _supply_at(&self, point: Point, t: U256) -> U256 {
        let mut last_point: Point = point;
        let mut t_i: U256 = last_point
            .ts
            .checked_div(WEEK)
            .unwrap_or_revert()
            .checked_mul(WEEK)
            .unwrap_or_revert();
        for _ in 0..255 {
            t_i = t_i.checked_add(WEEK).unwrap_or_revert();
            let mut d_slope: U128 = 0.into();
            if t_i > t {
                t_i = t;
            } else {
                d_slope = SlopeChanges::instance().get(&t_i);
            }
            last_point.bias = last_point
                .bias
                .checked_sub(
                    last_point
                        .slope
                        .checked_mul(
                            t_i.checked_sub(last_point.ts)
                                .unwrap_or_revert()
                                .as_u128()
                                .into(),
                        )
                        .unwrap_or_revert(),
                )
                .unwrap_or_revert();
            if t_i == t {
                break;
            }
            last_point.slope = last_point.slope.checked_add(d_slope).unwrap_or_revert();
            last_point.ts = t_i;
        }
        if last_point.bias < 0.into() {
            last_point.bias = 0.into();
        }
        last_point.bias.as_u128().into()
    }

    /// @notice Calculate total voting power
    /// @dev Adheres to the ERC20 `totalSupply` interface for Aragon compatibility
    /// @return Total voting power
    fn total_supply(&self, t: Option<U256>) -> U256 {
        let t: U256 = match t {
            Some(val) => val,
            None => {
                let blocktime: u64 = runtime::get_blocktime().into();
                blocktime.into()
            }
        };

        let epoch: U256 = get_epoch();
        let last_point: Point = PointHistory::instance().get(&epoch);
        self._supply_at(last_point, t)
    }

    /// @notice Calculate total voting power at some point in the past
    /// @param _block Block to calculate the total voting power at
    /// @return Total voting power at `_block`
    fn total_supply_at(&self, block: U256) -> U256 {
        // assert block <= block.number
        let epoch: U256 = get_epoch();
        let target_epoch: U256 = self._find_block_epoch(block, epoch);
        let point: Point = PointHistory::instance().get(&target_epoch);
        let mut dt: U256 = 0.into();
        if target_epoch < epoch {
            let point_next: Point = PointHistory::instance()
                .get(&target_epoch.checked_add(1.into()).unwrap_or_revert());
            if point.blk != point_next.blk {
                dt = block
                    .checked_sub(point.blk)
                    .unwrap_or_revert()
                    .checked_mul(point_next.ts.checked_sub(point.ts).unwrap_or_revert())
                    .unwrap_or_revert()
                    .checked_div(point_next.blk - point.blk)
                    .unwrap_or_revert();
            }
        } else {
            //  if point.blk != block.number {
            //      dt = (_block - point.blk) * (block.timestamp - point.ts) / (block.number - point.blk)
            //  }
        }
        // Now dt contains info on how far are we beyond point
        self._supply_at(point, point.ts.checked_add(dt).unwrap_or_revert())
    }

    /// Dummy methods for compatibility with Aragon
    /// @dev Dummy method required for Aragon compatibility
    fn change_controller(&self, new_controller: Key) {
        if self.get_caller() == get_controller() {
            set_controller(new_controller);
        }
    }

    fn emit(&self, voting_escrow_event: &VotingEscrowEvent) {
        let mut events = Vec::new();
        let tmp = get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match voting_escrow_event {
            VotingEscrowEvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            VotingEscrowEvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            VotingEscrowEvent::Deposit {
                provider,
                value,
                locktime,
                _type,
                ts,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                event.insert("locktime", locktime.to_string());
                event.insert("_type", _type.to_string());
                event.insert("ts", ts.to_string());
                events.push(event);
            }
            VotingEscrowEvent::Withdraw {
                provider,
                value,
                ts,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                event.insert("ts", ts.to_string());
                events.push(event);
            }
            VotingEscrowEvent::Supply {
                prev_supply,
                supply,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", voting_escrow_event.type_name());
                event.insert("prev_supply", prev_supply.to_string());
                event.insert("supply", supply.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
