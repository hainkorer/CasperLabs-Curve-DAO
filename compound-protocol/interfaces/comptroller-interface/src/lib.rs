use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, U256};
use contract_utils::set_key;

pub mod constants {

    /*
        Global storage keys
    */
    /// @notice Indicator that this is a Comptroller contract (for inspection)
    /// @type bool
    pub const IS_COMPTROLLER: &str = "is_comptroller";

    /*
        Runtime Args
    */

    /// @type Vec<Key>
    pub const RUNTIME_ARG_C_TOKENS: &str = "c_tokens";

    /// @type Key
    pub const RUNTIME_ARG_C_TOKEN: &str = "c_token";

    /// @type Key
    pub const RUNTIME_ARG_MINTER: &str = "minter";

    /// @type U256
    pub const RUNTIME_ARG_MINT_AMOUNT: &str = "mint_amount";

    /// @type U256
    pub const RUNTIME_ARG_MINT_TOKENS: &str = "mint_tokens";

    /// @type Key
    pub const RUNTIME_ARG_REDEEMER: &str = "redeemer";

    /// @type U256
    pub const RUNTIME_ARG_REDEEM_TOKENS: &str = "redeemer_tokens";

    /// @type U256
    pub const RUNTIME_ARG_REDEEM_AMOUNT: &str = "redeem_amount";

    /// @type Key
    pub const RUNTIME_ARG_BORROWER: &str = "borrower";

    /// @type U256
    pub const RUNTIME_ARG_BORROW_AMOUNT: &str = "borrow_amount";

    /// @type Key
    pub const RUNTIME_ARG_PAYER: &str = "payer";

    /// @type U256
    pub const RUNTIME_ARG_REPAY_AMOUNT: &str = "repay_amount";

    /// @type U256
    pub const RUNTIME_ARG_BORROW_INDEX: &str = "borrow_index";
    /// @type Key
    pub const RUNTIME_ARG_C_TOKEN_BORROWED: &str = "c_token_borrowed";
    /// @type Key
    pub const RUNTIME_ARG_C_TOKEN_COLLATERAL: &str = "c_token_collateral";
    /// @type Key
    pub const RUNTIME_ARG_LIQUIDATOR: &str = "liquidator";
    /// @type U256
    pub const RUNTIME_ARG_SEIZE_TOKENS: &str = "seize_tokens";
    pub const RUNTIME_ARG_SOURCE: &str = "source";
    pub const RUNTIME_ARG_DESTINATION: &str = "destination";
    pub const RUNTIME_ARG_TRANSFER_TOKENS: &str = "transfer_tokens";
    /*
        Entrypoint names
    */
    pub const ENTRYPOINT_ENTER_MARKETS: &str = "enter_markets";
    pub const ENTRYPOINT_EXIT_MARKET: &str = "exit_market";
    pub const ENTRYPOINT_MINT_ALLOWED: &str = "mint_allowed";
    pub const ENTRYPOINT_MINT_VERIFY: &str = "mint_verify";
    pub const ENTRYPOINT_REDEEM_ALLOWED: &str = "redeem_allowed";
    pub const ENTRYPOINT_REDEEM_VERIFY: &str = "redeem_verify";
    pub const ENTRYPOINT_BORROW_ALLOWED: &str = "borrow_allowed";
    pub const ENTRYPOINT_BORROW_VERIFY: &str = "borrow_verify";
    pub const ENTRYPOINT_REPAY_BORROW_ALLOWED: &str = "repay_borrow_allowed";
    pub const ENTRYPOINT_REPAY_BORROW_VERIFY: &str = "repay_borrow_verify";
    pub const ENTRYPOINT_LIQUIDATE_BORROW_ALLOWED: &str = "liquidate_borrow_allowed";
    pub const ENTRYPOINT_LIQUIDATE_BORROW_VERIFY: &str = "liquidate_borrow_verify";
    pub const ENTRYPOINT_SEIZE_ALLOWED: &str = "seize_allowed";
    pub const ENTRYPOINT_SEIZE_VERIFY: &str = "seize_verify";
    pub const ENTRYPOINT_TRANSFER_ALLOWED: &str = "transfer_allowed";
    pub const ENTRYPOINT_TRANSFER_VERIFY: &str = "transfer_verify";
    pub const ENTRYPOINT_LIQUIDATE_CALCULATE_SEIZE_TOKENS: &str =
        "liquidate_calculate_seize_tokens";
}

pub trait Comptroller {
    fn initialize(&self) {
        // @notice Indicator that this is a Comptroller contract (for inspection)
        set_key(constants::IS_COMPTROLLER, true);
    }

    /// Assets You Are In

    fn enter_markets(&self, c_tokens: Vec<Key>) -> Vec<U256>;
    fn exit_market(&self, c_token: Key) -> U256;

    /// Policy Hooks

    fn mint_allowed(&self, c_token: Key, minter: Key, mint_amount: U256) -> U256;
    fn mint_verify(&self, c_token: Key, minter: Key, mint_amount: U256, mint_tokens: U256);

    fn redeem_allowed(&self, c_token: Key, redeemer: Key, redeem_tokens: U256) -> U256;
    fn redeem_verify(&self, c_token: Key, redeemer: Key, redeem_amount: U256, redeem_tokens: U256);

    fn borrow_allowed(&self, c_token: Key, borrower: Key, borrow_amount: U256) -> U256;
    fn borrow_verify(&self, c_token: Key, borrower: Key, borrow_amount: U256);

    fn repay_borrow_allowed(
        &self,
        c_token: Key,
        payer: Key,
        borrower: Key,
        repay_amount: U256,
    ) -> U256;
    fn repay_borrow_verify(
        &self,
        c_token: Key,
        payer: Key,
        borrower: Key,
        repay_amount: U256,
        borrow_index: U256,
    );

    fn liquidate_borrow_allowed(
        &self,
        c_token_borrowed: Key,
        c_token_collateral: Key,
        liquidator: Key,
        borrower: Key,
        repay_amount: U256,
    ) -> U256;
    fn liquidate_borrow_verify(
        &self,
        c_token_borrowed: Key,
        c_token_collateral: Key,
        liquidator: Key,
        borrower: Key,
        repay_amount: U256,
        seize_tokens: U256,
    );

    fn seize_allowed(
        &self,
        c_token_collateral: Key,
        c_token_borrowed: Key,
        liquidator: Key,
        borrower: Key,
        seize_tokens: U256,
    ) -> U256;
    fn seize_verify(
        &self,
        c_token_collateral: Key,
        c_token_borrowed: Key,
        liquidator: Key,
        borrower: Key,
        seize_tokens: U256,
    );

    fn transfer_allowed(&self, c_token: Key, src: Key, dst: Key, transfer_tokens: U256) -> U256;
    fn transfer_verify(&self, c_token: Key, src: Key, dst: Key, transfer_tokens: U256);

    /*** Liquidity/Liquidation Calculations ***/

    fn liquidate_calculate_seize_tokens(
        &self,
        c_token_borrowed: Key,
        c_token_collateral: Key,
        repay_amount: U256,
    ) -> (U256, U256);
}

pub struct ComptrollerInterface(ContractPackageHash);
impl Comptroller for ComptrollerInterface {
    fn initialize(&self) {}

    /// Assets You Are In

    fn enter_markets(&self, c_tokens: Vec<Key>) -> Vec<U256> {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_ENTER_MARKETS,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKENS => c_tokens
            },
        )
    }

    fn exit_market(&self, c_token: Key) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_EXIT_MARKET,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN => c_token
            },
        )
    }

    // /// Policy Hooks

    fn mint_allowed(&self, c_token: Key, minter: Key, mint_amount: U256) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_MINT_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_MINTER=>minter,
                constants::RUNTIME_ARG_MINT_AMOUNT=>mint_amount
            },
        )
    }
    fn mint_verify(&self, c_token: Key, minter: Key, mint_amount: U256, mint_tokens: U256) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_MINT_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_MINTER=>minter,
                constants::RUNTIME_ARG_MINT_AMOUNT=>mint_amount,
                constants::RUNTIME_ARG_MINT_TOKENS => mint_tokens
            },
        )
    }

    fn redeem_allowed(&self, c_token: Key, redeemer: Key, redeem_tokens: U256) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_REDEEM_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_REDEEMER=>redeemer,
                constants::RUNTIME_ARG_REDEEM_TOKENS => redeem_tokens
            },
        )
    }

    fn redeem_verify(&self, c_token: Key, redeemer: Key, redeem_amount: U256, redeem_tokens: U256) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_REDEEM_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_REDEEMER=>redeemer,
                constants::RUNTIME_ARG_REDEEM_AMOUNT => redeem_amount,
                constants::RUNTIME_ARG_REDEEM_TOKENS => redeem_tokens
            },
        )
    }

    fn borrow_allowed(&self, c_token: Key, borrower: Key, borrow_amount: U256) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_BORROW_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_BORROWER => borrower,
                constants::RUNTIME_ARG_BORROW_AMOUNT => borrow_amount
            },
        )
    }

    fn borrow_verify(&self, c_token: Key, borrower: Key, borrow_amount: U256) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_BORROW_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN => c_token,
                constants::RUNTIME_ARG_BORROWER=> borrower,
                constants::RUNTIME_ARG_BORROW_AMOUNT=> borrow_amount,
            },
        )
    }

    fn repay_borrow_allowed(
        &self,
        c_token: Key,
        payer: Key,
        borrower: Key,
        repay_amount: U256,
    ) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_REPAY_BORROW_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_PAYER=>payer,
                constants::RUNTIME_ARG_BORROWER=>borrower,
                constants::RUNTIME_ARG_REPAY_AMOUNT=>repay_amount,
            },
        )
    }
    fn repay_borrow_verify(
        &self,
        c_token: Key,
        payer: Key,
        borrower: Key,
        repay_amount: U256,
        borrow_index: U256,
    ) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_REPAY_BORROW_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_PAYER=>payer,
                constants::RUNTIME_ARG_BORROWER=>borrower,
                constants::RUNTIME_ARG_REPAY_AMOUNT=>repay_amount,
                constants::RUNTIME_ARG_BORROW_INDEX=>borrow_index,
            },
        )
    }

    fn liquidate_borrow_allowed(
        &self,
        c_token_borrowed: Key,
        c_token_collateral: Key,
        liquidator: Key,
        borrower: Key,
        repay_amount: U256,
    ) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_LIQUIDATE_BORROW_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN_BORROWED=>c_token_borrowed,
                constants::RUNTIME_ARG_C_TOKEN_COLLATERAL=>c_token_collateral,
                constants::RUNTIME_ARG_LIQUIDATOR=>liquidator,
                constants::RUNTIME_ARG_BORROWER=>borrower,
                constants::RUNTIME_ARG_REPAY_AMOUNT=>repay_amount
            },
        )
    }

    fn liquidate_borrow_verify(
        &self,
        c_token_borrowed: Key,
        c_token_collateral: Key,
        liquidator: Key,
        borrower: Key,
        repay_amount: U256,
        seize_tokens: U256,
    ) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_LIQUIDATE_BORROW_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN_BORROWED=>c_token_borrowed,
                constants::RUNTIME_ARG_C_TOKEN_COLLATERAL=>c_token_collateral,
                constants::RUNTIME_ARG_LIQUIDATOR=>liquidator,
                constants::RUNTIME_ARG_BORROWER=>borrower,
                constants::RUNTIME_ARG_REPAY_AMOUNT=>repay_amount,
                constants::RUNTIME_ARG_SEIZE_TOKENS=>seize_tokens
            },
        )
    }

    fn seize_allowed(
        &self,
        c_token_collateral: Key,
        c_token_borrowed: Key,
        liquidator: Key,
        borrower: Key,
        seize_tokens: U256,
    ) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_SEIZE_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN_BORROWED=>c_token_borrowed,
                constants::RUNTIME_ARG_C_TOKEN_COLLATERAL=>c_token_collateral,
                constants::RUNTIME_ARG_LIQUIDATOR=>liquidator,
                constants::RUNTIME_ARG_BORROWER=>borrower,
                constants::RUNTIME_ARG_SEIZE_TOKENS=>seize_tokens
            },
        )
    }

    fn seize_verify(
        &self,
        c_token_collateral: Key,
        c_token_borrowed: Key,
        liquidator: Key,
        borrower: Key,
        seize_tokens: U256,
    ) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_SEIZE_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN_BORROWED=>c_token_borrowed,
                constants::RUNTIME_ARG_C_TOKEN_COLLATERAL=>c_token_collateral,
                constants::RUNTIME_ARG_LIQUIDATOR=>liquidator,
                constants::RUNTIME_ARG_BORROWER=>borrower,
                constants::RUNTIME_ARG_SEIZE_TOKENS=>seize_tokens
            },
        )
    }

    fn transfer_allowed(
        &self,
        c_token: Key,
        source: Key,
        destination: Key,
        transfer_tokens: U256,
    ) -> U256 {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_TRANSFER_ALLOWED,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_SOURCE=>source,
                constants::RUNTIME_ARG_DESTINATION=>destination,
                constants::RUNTIME_ARG_TRANSFER_TOKENS=>transfer_tokens
            },
        )
    }

    fn transfer_verify(&self, c_token: Key, source: Key, destination: Key, transfer_tokens: U256) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_TRANSFER_VERIFY,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN=>c_token,
                constants::RUNTIME_ARG_SOURCE=>source,
                constants::RUNTIME_ARG_DESTINATION=>destination,
                constants::RUNTIME_ARG_TRANSFER_TOKENS=>transfer_tokens
            },
        )
    }

    // /*** Liquidity/Liquidation Calculations ***/
    fn liquidate_calculate_seize_tokens(
        &self,
        c_token_borrowed: Key,
        c_token_collateral: Key,
        repay_amount: U256,
    ) -> (U256, U256) {
        runtime::call_versioned_contract(
            self.0,
            None,
            constants::ENTRYPOINT_LIQUIDATE_CALCULATE_SEIZE_TOKENS,
            runtime_args! {
                constants::RUNTIME_ARG_C_TOKEN_BORROWED=>c_token_borrowed,
                constants::RUNTIME_ARG_C_TOKEN_COLLATERAL=>c_token_collateral,
                constants::RUNTIME_ARG_REPAY_AMOUNT=>repay_amount,
            },
        )
    }
}
