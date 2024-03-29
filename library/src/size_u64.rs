use geometry_box::SizeBox;
use value_box::ValueBox;

use crate::size::SizeBoxFFI;

pub type BoxerSizeU64 = SizeBox<u64>;

#[no_mangle]
pub extern "C" fn boxer_size_u64_create() -> *mut ValueBox<BoxerSizeU64> {
    BoxerSizeU64::boxer_size_create()
}

#[no_mangle]
pub extern "C" fn boxer_size_u64_drop(ptr: *mut ValueBox<BoxerSizeU64>) {
    BoxerSizeU64::boxer_size_drop(ptr);
}

#[no_mangle]
pub extern "C" fn boxer_size_u64_get_width(ptr: *mut ValueBox<BoxerSizeU64>) -> u64 {
    BoxerSizeU64::boxer_size_get_width(ptr)
}

#[no_mangle]
pub extern "C" fn boxer_size_u64_set_width(ptr: *mut ValueBox<BoxerSizeU64>, width: u64) {
    BoxerSizeU64::boxer_size_set_width(ptr, width);
}

#[no_mangle]
pub extern "C" fn boxer_size_u64_get_height(ptr: *mut ValueBox<BoxerSizeU64>) -> u64 {
    BoxerSizeU64::boxer_size_get_height(ptr)
}

#[no_mangle]
pub extern "C" fn boxer_size_u64_set_height(ptr: *mut ValueBox<BoxerSizeU64>, height: u64) {
    BoxerSizeU64::boxer_size_set_height(ptr, height);
}
