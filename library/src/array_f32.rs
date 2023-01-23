use array_box::ArrayBox;
use value_box::ValueBox;

use crate::array::ArrayBoxFFI;

pub type BoxerArrayF32 = ArrayBox<f32>;

#[no_mangle]
pub extern "C" fn boxer_array_f32_create() -> *mut ValueBox<BoxerArrayF32> {
    BoxerArrayF32::boxer_array_create()
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_create_with(
    element: f32,
    amount: usize,
) -> *mut ValueBox<BoxerArrayF32> {
    BoxerArrayF32::boxer_array_create_with(element, amount)
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_create_from_data(
    _data: *mut f32,
    amount: usize,
) -> *mut ValueBox<BoxerArrayF32> {
    BoxerArrayF32::boxer_array_create_from_data(_data, amount)
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_get_length(_ptr: *mut ValueBox<BoxerArrayF32>) -> usize {
    BoxerArrayF32::boxer_array_get_length(_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_get_capacity(_ptr: *mut ValueBox<BoxerArrayF32>) -> usize {
    BoxerArrayF32::boxer_array_get_capacity(_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_get_data(_ptr: *mut ValueBox<BoxerArrayF32>) -> *mut f32 {
    BoxerArrayF32::boxer_array_get_data(_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_at_put(
    _ptr: *mut ValueBox<BoxerArrayF32>,
    index: usize,
    item: f32,
) {
    BoxerArrayF32::boxer_array_at_put(_ptr, index, item);
}

#[no_mangle]
pub extern "C" fn boxer_array_f32_drop(ptr: *mut ValueBox<BoxerArrayF32>) {
    BoxerArrayF32::boxer_array_drop(ptr);
}
