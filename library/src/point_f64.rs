use geometry_box::PointBox;
use value_box::ValueBox;

use crate::point::BoxerPointFFI;

pub type BoxerPointF64 = PointBox<f64>;

#[no_mangle]
pub extern "C" fn boxer_point_f64_default() -> *mut ValueBox<BoxerPointF64> {
    BoxerPointF64::boxer_point_default()
}

#[no_mangle]
pub extern "C" fn boxer_point_f64_create(x: f64, y: f64) -> *mut ValueBox<BoxerPointF64> {
    BoxerPointF64::boxer_point_create(x, y)
}

#[no_mangle]
pub extern "C" fn boxer_point_f64_drop(ptr: *mut ValueBox<BoxerPointF64>) {
    BoxerPointF64::boxer_point_drop(ptr);
}

#[no_mangle]
pub extern "C" fn boxer_point_f64_get_x(_point_ptr: *mut ValueBox<BoxerPointF64>) -> f64 {
    BoxerPointF64::boxer_point_get_x(_point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_f64_set_x(_point_ptr: *mut ValueBox<BoxerPointF64>, x: f64) {
    BoxerPointF64::boxer_point_set_x(_point_ptr, x);
}

#[no_mangle]
pub extern "C" fn boxer_point_f64_get_y(_point_ptr: *mut ValueBox<BoxerPointF64>) -> f64 {
    BoxerPointF64::boxer_point_get_y(_point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_f64_set_y(_point_ptr: *mut ValueBox<BoxerPointF64>, y: f64) {
    BoxerPointF64::boxer_point_set_y(_point_ptr, y);
}
