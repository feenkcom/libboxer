use geometry_box::U128Box;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn boxer_number_uint128_create() -> *mut ValueBox<U128Box> {
    ValueBox::new(U128Box::default()).into_raw()
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_drop(ptr: *mut ValueBox<U128Box>) {
    ptr.release();
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_get_low(_number_ptr: *mut ValueBox<U128Box>) -> u64 {
    _number_ptr.with_not_null_return(0, |number| number.low)
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_low(_number_ptr: *mut ValueBox<U128Box>, low: u64) {
    _number_ptr.with_not_null(|number| number.low = low);
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_get_high(_number_ptr: *mut ValueBox<U128Box>) -> u64 {
    _number_ptr.with_not_null_return(0, |number| number.high)
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_high(_number_ptr: *mut ValueBox<U128Box>, high: u64) {
    _number_ptr.with_not_null(|number| number.high = high);
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_max(_number_ptr: *mut ValueBox<U128Box>) {
    _number_ptr.with_not_null(|number| number.set(u128::MAX));
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_min(_number_ptr: *mut ValueBox<U128Box>) {
    _number_ptr.with_not_null(|number| number.set(u128::MIN));
}
