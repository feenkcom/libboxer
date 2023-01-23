use geometry_box::PointBox;
use std::any::Any;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

pub trait BoxerPointFFI<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> *mut ValueBox<PointBox<T>>;

    fn boxer_point_create(x: T, y: T) -> *mut ValueBox<PointBox<T>>;

    fn boxer_point_drop(ptr: *mut ValueBox<PointBox<T>>);

    fn boxer_point_get_x(point_box: *mut ValueBox<PointBox<T>>) -> T;

    fn boxer_point_set_x(point_box: *mut ValueBox<PointBox<T>>, x: T);

    fn boxer_point_get_y(point_box: *mut ValueBox<PointBox<T>>) -> T;

    fn boxer_point_set_y(point_box: *mut ValueBox<PointBox<T>>, y: T);
}

impl<T> BoxerPointFFI<T> for PointBox<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> *mut ValueBox<PointBox<T>> {
        value_box!(PointBox::<T>::default()).into_raw()
    }

    fn boxer_point_create(x: T, y: T) -> *mut ValueBox<PointBox<T>> {
        value_box!(PointBox::<T>::new(x, y)).into_raw()
    }

    fn boxer_point_drop(point_box: *mut ValueBox<PointBox<T>>) {
        point_box.release();
    }

    fn boxer_point_get_x(point_box: *mut ValueBox<PointBox<T>>) -> T {
        point_box.with_ref_ok(|point| point.x).or_log(0u8.into())
    }

    fn boxer_point_set_x(point_box: *mut ValueBox<PointBox<T>>, x: T) {
        point_box.with_mut_ok(|point| point.x = x).log();
    }

    fn boxer_point_get_y(point_box: *mut ValueBox<PointBox<T>>) -> T {
        point_box.with_ref_ok(|point| point.y).or_log(0u8.into())
    }

    fn boxer_point_set_y(point_box: *mut ValueBox<PointBox<T>>, y: T) {
        point_box.with_mut_ok(|point| point.y = y).log();
    }
}
