use casper_types::{Key, U128};
use core::convert::TryInto;

pub fn zero_address() -> Key {
    Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
}

pub fn account_zero_address() -> Key {
    Key::from_formatted_str(
        "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap()
}

// ---- TUPLE USAGE FOR int128 ----
// As primtive i128 cannot be handled in structs and entrypoints
// so changing it to tuple (sign:bool {true:(-ve) | false:(+ve)}, value: U128)
// ---- TUPLE USAGE FOR int128 ----

pub fn tuple_to_i128(value: (bool, U128)) -> i128 {
    if value.0 {
        let val: i128 = value.1.as_u128().try_into().unwrap();
        -val
    } else {
        value.1.as_u128().try_into().unwrap()
    }
}

pub fn i128_to_tuple(value: i128) -> (bool, U128) {
    let mut val: (bool, U128) = (false, 0.into());
    if value < 0 {
        val.0 = true;
        val.1 = (-value).into();
    } else {
        val.0 = false;
        val.1 = value.into();
    }
    val
}
