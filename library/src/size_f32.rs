use geometry_box::SizeBox;
use value_box::ValueBox;

use crate::size::SizeBoxFFI;

pub type BoxerSizeF32 = SizeBox<f32>;

#[no_mangle]
pub extern "C" fn boxer_size_f32_create() -> *mut ValueBox<BoxerSizeF32> {
    BoxerSizeF32::boxer_size_create()
}

#[no_mangle]
pub extern "C" fn boxer_size_f32_drop(ptr: *mut ValueBox<BoxerSizeF32>) {
    BoxerSizeF32::boxer_size_drop(ptr);
}

#[no_mangle]
pub extern "C" fn boxer_size_f32_get_width(ptr: *mut ValueBox<BoxerSizeF32>) -> f32 {
    BoxerSizeF32::boxer_size_get_width(ptr)
}

#[no_mangle]
pub extern "C" fn boxer_size_f32_set_width(ptr: *mut ValueBox<BoxerSizeF32>, width: f32) {
    BoxerSizeF32::boxer_size_set_width(ptr, width);
}

#[no_mangle]
pub extern "C" fn boxer_size_f32_get_height(ptr: *mut ValueBox<BoxerSizeF32>) -> f32 {
    BoxerSizeF32::boxer_size_get_height(ptr)
}

#[no_mangle]
pub extern "C" fn boxer_size_f32_set_height(ptr: *mut ValueBox<BoxerSizeF32>, height: f32) {
    BoxerSizeF32::boxer_size_set_height(ptr, height);
}
