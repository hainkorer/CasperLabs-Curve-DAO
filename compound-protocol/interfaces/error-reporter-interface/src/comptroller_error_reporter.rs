use std::collections::BTreeMap;

use casper_contract::contract_api::storage;
use casper_types::{ApiError, ContractPackageHash, URef, U256};

#[repr(u16)]
pub enum Error {
    NoError,
    Unauthorized,
    ComptrollerMismatch,
    InsufficientShortfall,
    InsufficientLiquidity,
    InvalidCloseFactor,
    InvalidCollateralFactor,
    InvalidLiquidationIncentive,
    MarketNotEnteredNoLongerPossible,
    MarketNotListed,
    MarketAlreadyListed,
    MathError,
    NonzeroBorrowBalance,
    PriceError,
    Rejection,
    SnapshotError,
    TooManyAssets,
    TooMuchRepay,
}
impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[repr(u16)]
pub enum FailureInfo {
    AcceptAdminPendingAdminCheck,
    AcceptPendingImplementationAddressCheck,
    ExitMarketBalanceOwed,
    ExitMarketRejection,
    SetCloseFactorOwnerCheck,
    SetCloseFactorValidation,
    SetCollateralFactorOwnerCheck,
    SetCollateralFactorNoExists,
    SetCollateralFactorValidation,
    SetCollateralFactorWithoutPrice,
    SetImplementationOwnerCheck,
    SetLiquidationIncentiveOwnerCheck,
    SetLiquidationIncentiveValidation,
    SetMaxAssetsOwnerCheck,
    SetPendingAdminOwnerCheck,
    SetPendingImplementationOwnerCheck,
    SetPriceOracleOwnerCheck,
    SupportMarketExists,
    SupportMarketOwnerCheck,
    SetPauseGuardianOwnerCheck,
}
impl From<FailureInfo> for ApiError {
    fn from(error: FailureInfo) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum ComptrollerErrorReporterEvent {
    ///  @dev `error` corresponds to enum Error; `info` corresponds to enum FailureInfo, and `detail` is an arbitrary
    ///  contract-specific code that enables us to report opaque error codes from upgradeable contracts.
    ///
    Failure {
        error: U256,
        info: U256,
        detail: U256,
    },
}

impl ComptrollerErrorReporterEvent {
    pub fn type_name(&self) -> String {
        match self {
            ComptrollerErrorReporterEvent::Failure {
                error: _,
                info: _,
                detail: _,
            } => "ComptrollerErrorReporter::Failure",
        }
        .to_string()
    }
}

fn emit(
    comptroller_error_reporter_event: &ComptrollerErrorReporterEvent,
    package_hash: ContractPackageHash,
) {
    let mut events = Vec::new();
    let tmp = package_hash.to_formatted_string();
    let tmp: Vec<&str> = tmp.split("-").collect();
    let package_hash = tmp[1].to_string();
    match comptroller_error_reporter_event {
        ComptrollerErrorReporterEvent::Failure {
            error,
            info,
            detail,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package_hash);
            event.insert("event_type", comptroller_error_reporter_event.type_name());
            event.insert("error", error.to_string());
            event.insert("info", info.to_string());
            event.insert("detail", detail.to_string());
            events.push(event);
        }
    };
    for event in events {
        let _: URef = storage::new_uref(event);
    }
}

pub trait ComptrollerErrorReporter {
    ///
    /// @dev use this when reporting a known error from the money market or a non-upgradeable collaborator
    /// @returns Error variant as u16
    ///
    fn fail(&self, error: Error, info: FailureInfo, package_hash: ContractPackageHash) -> u16 {
        let error_variant: u16 = error as u16;
        let failure_info_variant: u16 = info as u16;
        emit(
            &ComptrollerErrorReporterEvent::Failure {
                error: error_variant.into(),
                info: failure_info_variant.into(),
                detail: U256::zero(),
            },
            package_hash,
        );
        error_variant
    }

    /// @dev use this when reporting an opaque error from an upgradeable collaborator contract
    /// @returns Error variant as u16
    // /
    fn fail_opaque(
        &self,
        error: Error,
        info: FailureInfo,
        opaque_error: U256,
        package_hash: ContractPackageHash,
    ) -> u16 {
        let error_variant: u16 = error as u16;
        let failure_info_variant: u16 = info as u16;
        emit(
            &ComptrollerErrorReporterEvent::Failure {
                error: error_variant.into(),
                info: failure_info_variant.into(),
                detail: opaque_error,
            },
            package_hash,
        );
        error_variant
    }
}
