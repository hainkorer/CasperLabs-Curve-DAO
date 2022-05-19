use core::convert::TryInto;
use crate::data::{
    self,  Allowance, BalanceOf, zero_address,ClaimData,ClaimDataStruct, RewardTokens, RewardsReceiver,MAX_REWARDS,CLAIM_FREQUENCY,
    RewardIntegral, RewardIntegralFor,RewardData
};
use crate::{alloc::string::ToString, error::Error, event::*};
use alloc::{collections::BTreeMap, string::String};
use alloc::vec::Vec;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::bytesrepr::Bytes;
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U128, U256,
};
use contract_utils::{ContractContext, ContractStorage};

pub trait LIQUIDITYTGAUGEV3<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(
        &mut self,
        lp_token: Key,
        minter: Key,
        admin: Key,
        contract_hash: ContractHash,
        package_hash: ContractPackageHash,
    ) {
        data::BalanceOf::init();
        data::IntegrateCheckpointOf::init();
        data::IntegrateFraction::init();
        data::IntegrateInvSupply::init();
        data::IntegrateInvSupplyOf::init();
        data::PeriodTimestamp::init();
        data::WorkingBalances::init();
        
        let _lp_token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let _lp_token_package_hash = ContractPackageHash::new(_lp_token_hash_add_array);
        let symbol: String = runtime::call_versioned_contract(
            _lp_token_package_hash,
            None,
            "symbol",
            runtime_args! {},
        );
        let mut name: String = "Curve.fi ".to_string();
        let post_name: &str = "RewardGauge Deposit";
        name.push_str(symbol.as_str());
        name.push_str(post_name);
        let decimals: u8 = 9;
        let total_supply: U256 = 0.into();
        data::set_name(name);
        data::set_symbol(symbol + "-gauge");
        data::set_total_supply(total_supply);
        data::set_decimals(decimals);
        data::set_lp_token(lp_token);
        data::set_admin(admin);
        data::set_minter(minter);
        let crv_addr: Key = runtime::call_versioned_contract(
            minter.into_hash().unwrap_or_revert().into(),
            None,
            "token",
            runtime_args! {},
        );
        data::set_crv_token(crv_addr);
       
        let block_timestamp: u64 = runtime::get_blocktime().into();
       // data::PeriodTimestamp::instance().set(&U256::from(0), block_timestamp.into());
        data::set_inflation_rate(runtime::call_versioned_contract(
            crv_addr.into_hash().unwrap_or_revert().into(),
            None,
            "rate",
            runtime_args! {},
        ));
        data::set_future_epoch_time(runtime::call_versioned_contract(
            crv_addr.into_hash().unwrap_or_revert().into(),
            None,
            "future_epoch_time_write",
            runtime_args! {},
        ));
        
        data::set_package_hash(package_hash);
        data::set_contract_hash(contract_hash);
        data::set_lock(false);
        

    }
    fn total_supply(&mut self) -> U256 {
        data::get_total_supply()
    }

    fn name(&mut self) -> String {
        data::get_name()
    }

    fn symbol(&mut self) -> String {
        data::get_symbol()
    }

    fn decimals(&mut self) -> u8 {
        data::get_decimals()
    }

    fn reward_data(&mut self) -> RewardData {
        data::reward_data()
    }
    fn lp_token(&mut self) -> Key {
        data::get_lp_token()
    }
    fn admin(&mut self) -> Key {
        data::get_admin()
    }
    fn reward_integral(&mut self, reward_token: Key) -> U256 {
        RewardIntegral::instance().get(&reward_token)
    }
    fn reward_tokens(&mut self, index: U256) -> Key {
        RewardTokens::instance().get(&index)
    }

    fn future_admin(&mut self) -> Key {
        data::get_future_admin()
    }
    fn claim_data(&mut self, user: Key, claiming_address: Key) -> ClaimDataStruct {
        ClaimData::instance().get(&user, &claiming_address)
    }
    fn balance_of(&mut self, owner: Key) -> U256 {
        BalanceOf::instance().get(&owner)
    }


    fn reward_contract(&mut self) -> Key {
        let address = self.reward_data().address;
        if address == zero_address() {
            zero_address()
        } else {
            address
        }
    }
    fn last_claim(&mut self) -> U256 {
        self.reward_data().time_stamp
    }
    fn claimed_reward(&mut self, _addr: Key, _token: Key) -> U256 {
        self.claim_data(_addr, _token).claimed_amount
    }
    fn claimable_reward(&mut self, _addr: Key, _token: Key) -> U256 {
        self.claim_data(_addr, _token).claimable_amount
    }
    fn claimable_reward_write(&mut self, _addr: Key, _token: Key) -> U256 {
        let lock = data::get_lock();
        if lock != false {
            // Locked
            runtime::revert(Error:: LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        let reward_token = self.reward_tokens(0.into());
        if reward_token != zero_address() {
            let total_supply = self.total_supply();
            self._checkpoint_rewards(_addr, total_supply, false, zero_address());
        }
        data::set_lock(false);
        self.claim_data(_addr, _addr).claimable_amount
    }
    fn set_rewards_receiver(&mut self, _receiver: Key) {
        RewardsReceiver::instance().set(&self.get_caller(), _receiver)
    }

    fn _checkpoint_rewards(
        &mut self,
        _user: Key,
        _total_supply: U256,
        _claim: bool,
        _receiver: Key,
    ) {
   
        // let mut reward_tokens:Vec<Key>=Vec::new();
        // let mut reward_integrals:Vec<U256>=Vec::new();
        // for i in 0..(MAX_REWARDS.as_usize()) {
        //     let token:Key=self.reward_tokens(i.into());
        //     if (token==zero_address()){
        //         break;
        //     }
        //     reward_tokens.push(token);
        //     reward_integrals.push( self.reward_integral(token));
            
        // }
        // let mut reward_data: RewardData = self.reward_data();
        // if _total_supply != 0.into()
        //     && reward_data.address != zero_address()
        //     && reward_data.time_stamp != 0.into()
        //     && U256::from(u64::from(runtime::get_blocktime()))
        //         > (reward_data.time_stamp + U256::from(CLAIM_FREQUENCY.as_u128()))
        // {

        // }   

    }
    fn _checkpoint(&mut self,addr:Key){

    }

    
    // lock
    fn claim_rewards(&mut self, _addr: Option<Key>, _receiver: Option<Key>) {
        let lock = data::get_lock();
        if lock != false {
            // Locked
            runtime::revert(Error:: LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        let addr: Key;
        let receiver: Key;
        if _addr.is_none() {
            addr = self.get_caller();
        } else {
            addr = _addr.unwrap();
        }
        if _receiver.is_none() {
            receiver = zero_address();
        } else {
            receiver = _receiver.unwrap();
        }
        if receiver != zero_address() {
            if addr != self.get_caller() {
                // Reward Only Gauge Cannot Redirect When Claiming For Another User
                runtime::revert(Error:: LiquidityGaugeCannotRedirectWhenClaimingForAnotherUser);
            }
        }
        let total_supply = self.total_supply();
        //self._checkpoint_rewards(addr, total_supply, true, receiver);
        data::set_lock(false);
    }
    fn set_killed(&mut self,is_killed:bool){
        if self.get_caller() != self.admin() {
            runtime::revert(Error:: LiquidityGaugeOnlyAdmin1);
        }
        data::set_is_killed(is_killed);
        
    }
    fn _update_liquidity_limit(&mut self,addr:Key,l:U256,L:U256){

    }
    fn deposit(&mut self, _value: U256, _addr: Option<Key>, _claim_rewards: Option<bool>) {
        let claim_rewards: bool;
        if _claim_rewards.is_none() {
            claim_rewards = false;
        } else {
            claim_rewards = _claim_rewards.unwrap();
        }
        let addr: Key;
        if _addr.is_none() {
            addr = self.get_caller();
        } else {
            addr = _addr.unwrap();
        }
        //self.checpoint(_addr);
        let lock = data::get_lock();
        if lock != false {
            //Locked
            runtime::revert(Error:: LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        let is_rewards:bool=self.reward_tokens(0.into())!=zero_address();
        let mut total_supply = self.total_supply();
        if (is_rewards){
            self._checkpoint_rewards(addr, total_supply, claim_rewards, zero_address());
        }
        total_supply = total_supply
        .checked_add(_value)
        .ok_or(Error::LiquidityGaugeV3OverFlow4)
        .unwrap_or_revert();
    let balance = self.balance_of(self.get_caller());
    let new_balance = balance
        .checked_add(_value)
        .ok_or(Error::LiquidityGaugeV3OverFlow5)
        .unwrap_or_revert();
    BalanceOf::instance().set(&self.get_caller(), new_balance);
    data::set_total_supply(total_supply);

    self._update_liquidity_limit(addr, new_balance, total_supply);

    let lp_token = self.lp_token();
        let token_hash_add_array = match lp_token {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let token_package_hash = ContractPackageHash::new(token_hash_add_array);
        let _result: () = runtime::call_versioned_contract(
            token_package_hash,
            None,
            "transfer_from",
            runtime_args! {"_from" => self.get_caller(),"_to" =>  data::get_package_hash(),"_value" => _value},
        );
        if (is_rewards){
            let mut reward_data: RewardData = self.reward_data();
            if (reward_data.time_stamp>0.into()){

                // let deposit_sig:Bytes=self.reward_sigs
                // if convert(deposit_sig, uint256) != 0:
                //     raw_call(
                //         convert(reward_data % 2**160, address),
                //         concat(deposit_sig, convert(_value, bytes32))
                //     )
            }

        }
        self.emit(&LiquidityGaugeV3Event::Deposit {
            provider: self.get_caller(),
            value: _value,
        });
        self.emit(&LiquidityGaugeV3Event::Transfer {
            from: self.get_caller(),
            to: zero_address(),
            value: _value,
        });
        data::set_lock(false);
    }
    // fn withdraw(&mut self, _value: U256, _claim_rewards: Option<bool>) {
    //     let lock = data::get_lock();
    //     if lock != false {
    //         //Locked
    //         runtime::revert(Error::LiquidityGaugeLocked1);
    //     }
    //     data::set_lock(true);
    //     self._checkpoint(self.get_caller());
    //     if _value != 0.into() {
    //         // Reward Only Gauge Value Is Zero
    //         runtime::revert(Error::RewardOnlyGaugeValueIsZero1);
    //     }
    // }
    fn _transfer(&mut self, _from: Key, _to: Key, _value: U256){
        // self._checkpoint(_from);
        // self._checkpoint(_to);
        if _value != 0.into() {
            let total_supply = self.total_supply();
            let is_rewards:bool=self.reward_tokens(0.into())!=zero_address();
            if (is_rewards){
            self._checkpoint_rewards(_from, total_supply, false, zero_address());
            }
            let balances: BalanceOf = BalanceOf::instance();
            let _from_balance: U256 = balances.get(&_from);
            let from_new_balance = _from_balance
                .checked_sub(_value)
                .ok_or(Error::LiquidityGaugeUnderFlow3)
                .unwrap_or_revert();
            balances.set(&_from, from_new_balance);
            self._update_liquidity_limit(_from, from_new_balance, total_supply);
            if (is_rewards){
                self._checkpoint_rewards(_to, total_supply, false, zero_address());
            }
            let balances: BalanceOf = BalanceOf::instance();
            let _to_balance: U256 = balances.get(&_to);
            let to_new_balance = _from_balance
                .checked_sub(_value)
                .ok_or(Error::LiquidityGaugeUnderFlow4)
                .unwrap_or_revert();
            balances.set(&_to, to_new_balance);
            self._update_liquidity_limit(_to, to_new_balance, total_supply);
        }
        self.emit(&LiquidityGaugeV3Event::Transfer {
            from: _from,
            to: _to,
            value: _value,
        });
    }
    fn transfer(&mut self, _to: Key, _value: U256) -> bool {
        let lock = data::get_lock();
        if lock != false {
           
            runtime::revert(Error::LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        self._transfer(self.get_caller(), _to, _value);
        data::set_lock(false);
        return true;
    }
    fn transfer_from(&mut self, _from: Key, _to: Key, _value: U256) -> bool {
        let lock = data::get_lock();
        if lock != false {
            //Locked
            runtime::revert(Error::LiquidityGaugeLocked1);
        }
        data::set_lock(true);
        let allowances = Allowance::instance();
        let _allowance: U256 = allowances.get(&_from, &self.get_caller());
        if _allowance != U256::MAX {
            let new_allowance: U256 = _allowance
                .checked_sub(_value)
                .ok_or(Error::LiquidityGaugeUnderFlow2)
                .unwrap_or_revert();
            self._approve(_from, self.get_caller(), new_allowance);
        }
        self._transfer(_from, _to, _value);
        data::set_lock(false);
        return true;
    }

    fn approve(&mut self, spender: Key, _value: U256) -> bool {
        self._approve(self.get_caller(), spender, _value)
    }

    fn _approve(&mut self, _owner: Key, _spender: Key, _value: U256) -> bool {
        Allowance::instance().set(&_owner, &_spender, _value);
        self.emit(&LiquidityGaugeV3Event::Approval {
            owner: _owner,
            spender: _spender,
            value: _value,
        });
        return true;
    }
    fn increase_allowance(&mut self, _spender: Key, _added_value: U256) -> bool {
        let allowances = Allowance::instance();
        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &_spender);
        let new_allowance: U256 = spender_allowance
            .checked_add(_added_value)
            .ok_or(Error::LiquidityGaugeV3OverFlow1)
            .unwrap_or_revert();
        self._approve(owner, _spender, new_allowance);
        return true;
    }
    fn decrease_allowance(&mut self, _spender: Key, _subtracted_value: U256) -> bool {
        let allowances = Allowance::instance();

        let owner: Key = self.get_caller();

        let spender_allowance: U256 = allowances.get(&owner, &_spender);

        let new_allowance: U256 = spender_allowance
            .checked_sub(_subtracted_value)
            .ok_or(Error::LiquidityGaugeUnderFlow1)
            .unwrap_or_revert();
        self._approve(owner, _spender, new_allowance);

        return true;
    }
   
    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Reward Only Gauge Only Admin
            runtime::revert(Error:: LiquidityGaugeOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&LiquidityGaugeV3Event::CommitOwnership { admin: addr });
    }

    fn accept_transfer_ownership(&mut self) {
        let _admin = self.future_admin();
        if self.get_caller() != _admin {
            //Reward Only Gauge Only Future Admin
            runtime::revert(Error:: LiquidityGaugeOnlyFutureAdmin);
        }
        data::set_admin(_admin);
        self.emit(&LiquidityGaugeV3Event::ApplyOwnership { admin: _admin });
    }
    

    fn emit(&self, liquidity_gauge_event: &LiquidityGaugeV3Event) {
        let mut events = Vec::new();
        let tmp = data::get_package_hash().to_formatted_string();
        let tmp: Vec<&str> = tmp.split("-").collect();
        let package_hash = tmp[1].to_string();
        match liquidity_gauge_event {
            LiquidityGaugeV3Event::Deposit { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::Withdraw { provider, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("provider", provider.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::Approval { owner,spender, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::Transfer { from,to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::UpdateLiquidityLimit {
                user,
                original_balance,
                original_supply,
                working_balance,
                working_supply,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("user", user.to_string());
                event.insert("original_balance", original_balance.to_string());
                event.insert("original_supply", original_supply.to_string());
                event.insert("working_balance", working_balance.to_string());
                event.insert("working_supply", working_supply.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
            LiquidityGaugeV3Event::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package_hash);
                event.insert("event_type", liquidity_gauge_event.type_name());
                event.insert("admin", admin.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }


    

   
}
