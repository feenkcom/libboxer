#![allow(non_snake_case)]
extern crate crossbeam;

#[macro_use]
extern crate value_box;
#[cfg(feature = "phlow")]
#[macro_use]
extern crate phlow_core as phlow;
#[cfg(feature = "phlow")]
extern crate phlow_extensions;

pub mod array;
pub mod array_f32;
pub mod array_int;
pub mod array_point_f32;
pub mod array_u16;
pub mod array_u8;
pub mod array_uint;
pub mod boxes;
pub mod number_uint128;
pub mod point;
pub mod point3;
pub mod point3_f32;
pub mod point_f32;
pub mod point_f64;
pub mod point_i32;
pub mod point_u64;
pub mod range_usize;
pub mod size;
pub mod size_f32;
pub mod size_f64;
pub mod size_i32;
pub mod size_u32;
pub mod size_u64;
pub mod string;

#[no_mangle]
pub extern "C" fn boxer_test() -> bool {
    return true;
}

#[cfg(feature = "phlow")]
use phlow_extensions::CoreExtensions;
#[cfg(feature = "phlow")]
import_extensions!(CoreExtensions);
