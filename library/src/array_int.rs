use core::ffi::c_int;

use array_box::ArrayBox;
use value_box::ValueBox;

use crate::array::ArrayBoxFFI;

pub type BoxerArrayInt = ArrayBox<c_int>;

#[no_mangle]
pub extern "C" fn boxer_array_int_create() -> *mut ValueBox<BoxerArrayInt> {
    BoxerArrayInt::boxer_array_create()
}

#[no_mangle]
pub extern "C" fn boxer_array_int_create_with(
    element: c_int,
    amount: usize,
) -> *mut ValueBox<BoxerArrayInt> {
    BoxerArrayInt::boxer_array_create_with(element, amount)
}

#[no_mangle]
pub extern "C" fn boxer_array_int_create_from_data(
    _data: *mut c_int,
    amount: usize,
) -> *mut ValueBox<BoxerArrayInt> {
    BoxerArrayInt::boxer_array_create_from_data(_data, amount)
}

#[no_mangle]
pub extern "C" fn boxer_array_int_get_length(_ptr: *mut ValueBox<BoxerArrayInt>) -> usize {
    BoxerArrayInt::boxer_array_get_length(_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_array_int_get_capacity(_ptr: *mut ValueBox<BoxerArrayInt>) -> usize {
    BoxerArrayInt::boxer_array_get_capacity(_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_array_int_get_data(_ptr: *mut ValueBox<BoxerArrayInt>) -> *mut c_int {
    BoxerArrayInt::boxer_array_get_data(_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_array_int_at_put(
    _ptr: *mut ValueBox<BoxerArrayInt>,
    index: usize,
    item: c_int,
) {
    BoxerArrayInt::boxer_array_at_put(_ptr, index, item);
}

#[no_mangle]
pub extern "C" fn boxer_array_int_at(_ptr: *mut ValueBox<BoxerArrayInt>, index: usize) -> c_int {
    BoxerArrayInt::boxer_array_at(_ptr, index, 0)
}

#[no_mangle]
pub extern "C" fn boxer_array_int_drop(ptr: *mut ValueBox<BoxerArrayInt>) {
    BoxerArrayInt::boxer_array_drop(ptr);
}
