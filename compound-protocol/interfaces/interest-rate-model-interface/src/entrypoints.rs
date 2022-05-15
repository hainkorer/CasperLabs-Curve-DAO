use crate::constants;
extern crate alloc;
use alloc::vec;
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

/// Returns the entrypoints for Interest Rate Model contracts.  
pub fn entry_points_install() -> EntryPoints {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        constants::ENTRYPOINT_GET_BORROW_RATE,
        vec![
            Parameter::new(constants::RUNTIME_ARG_CASH, CLType::U256),
            Parameter::new(constants::RUNTIME_ARG_BORROWS, CLType::U256),
            Parameter::new(constants::RUNTIME_ARG_RESERVES, CLType::U256),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        constants::ENTRYPOINT_GET_SUPPLY_RATE,
        vec![
            Parameter::new(constants::RUNTIME_ARG_CASH, CLType::U256),
            Parameter::new(constants::RUNTIME_ARG_BORROWS, CLType::U256),
            Parameter::new(constants::RUNTIME_ARG_RESERVES, CLType::U256),
            Parameter::new(constants::RUNTIME_ARG_RESERVE_FACTOR_MANTISSA, CLType::U256),
        ],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    return entry_points;
}
