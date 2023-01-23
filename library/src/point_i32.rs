use geometry_box::PointBox;
use value_box::ValueBox;

use crate::point::BoxerPointFFI;

pub type BoxerPointI32 = PointBox<i32>;

#[no_mangle]
pub extern "C" fn boxer_point_i32_default() -> *mut ValueBox<BoxerPointI32> {
    BoxerPointI32::boxer_point_default()
}

#[no_mangle]
pub extern "C" fn boxer_point_i32_create(x: i32, y: i32) -> *mut ValueBox<BoxerPointI32> {
    BoxerPointI32::boxer_point_create(x, y)
}

#[no_mangle]
pub extern "C" fn boxer_point_i32_drop(ptr: *mut ValueBox<BoxerPointI32>) {
    BoxerPointI32::boxer_point_drop(ptr);
}

#[no_mangle]
pub extern "C" fn boxer_point_i32_get_x(point_ptr: *mut ValueBox<BoxerPointI32>) -> i32 {
    BoxerPointI32::boxer_point_get_x(point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_i32_set_x(point_ptr: *mut ValueBox<BoxerPointI32>, x: i32) {
    BoxerPointI32::boxer_point_set_x(point_ptr, x);
}

#[no_mangle]
pub extern "C" fn boxer_point_i32_get_y(point_ptr: *mut ValueBox<BoxerPointI32>) -> i32 {
    BoxerPointI32::boxer_point_get_y(point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_i32_set_y(_point_ptr: *mut ValueBox<BoxerPointI32>, y: i32) {
    BoxerPointI32::boxer_point_set_y(_point_ptr, y);
}
