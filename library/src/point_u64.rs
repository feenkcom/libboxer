use geometry_box::PointBox;
use value_box::ValueBox;

use crate::point::BoxerPointFFI;

pub type BoxerPointU64 = PointBox<u64>;

#[no_mangle]
pub extern "C" fn boxer_point_u64_default() -> *mut ValueBox<BoxerPointU64> {
    BoxerPointU64::boxer_point_default()
}

#[no_mangle]
pub extern "C" fn boxer_point_u64_create(x: u64, y: u64) -> *mut ValueBox<BoxerPointU64> {
    BoxerPointU64::boxer_point_create(x, y)
}

#[no_mangle]
pub extern "C" fn boxer_point_u64_drop(ptr: *mut ValueBox<BoxerPointU64>) {
    BoxerPointU64::boxer_point_drop(ptr);
}

#[no_mangle]
pub extern "C" fn boxer_point_u64_get_x(_point_ptr: *mut ValueBox<BoxerPointU64>) -> u64 {
    BoxerPointU64::boxer_point_get_x(_point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_u64_set_x(_point_ptr: *mut ValueBox<BoxerPointU64>, x: u64) {
    BoxerPointU64::boxer_point_set_x(_point_ptr, x);
}

#[no_mangle]
pub extern "C" fn boxer_point_u64_get_y(_point_ptr: *mut ValueBox<BoxerPointU64>) -> u64 {
    BoxerPointU64::boxer_point_get_y(_point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_u64_set_y(_point_ptr: *mut ValueBox<BoxerPointU64>, y: u64) {
    BoxerPointU64::boxer_point_set_y(_point_ptr, y);
}
