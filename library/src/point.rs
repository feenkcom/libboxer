use geometry_box::PointBox;
use value_box::{ValueBox, ValueBoxPointer};

pub trait BoxerPointFFI<T>
where
    T: From<u8> + Default + Copy,
{
    fn boxer_point_default() -> *mut ValueBox<PointBox<T>>;

    fn boxer_point_create(x: T, y: T) -> *mut ValueBox<PointBox<T>>;

    fn boxer_point_drop(ptr: *mut ValueBox<PointBox<T>>);

    fn boxer_point_get_x(_maybe_null_ptr: *mut ValueBox<PointBox<T>>) -> T;

    fn boxer_point_set_x(_maybe_null_ptr: *mut ValueBox<PointBox<T>>, x: T);

    fn boxer_point_get_y(_maybe_null_ptr: *mut ValueBox<PointBox<T>>) -> T;

    fn boxer_point_set_y(_maybe_null_ptr: *mut ValueBox<PointBox<T>>, y: T);
}

impl<T> BoxerPointFFI<T> for PointBox<T>
where
    T: From<u8> + Default + Copy,
{
    fn boxer_point_default() -> *mut ValueBox<PointBox<T>> {
        ValueBox::new(PointBox::<T>::default()).into_raw()
    }

    fn boxer_point_create(x: T, y: T) -> *mut ValueBox<PointBox<T>> {
        ValueBox::new(PointBox::<T>::new(x, y)).into_raw()
    }

    fn boxer_point_drop(ptr: *mut ValueBox<PointBox<T>>) {
        ptr.release();
    }

    fn boxer_point_get_x(_maybe_null_ptr: *mut ValueBox<PointBox<T>>) -> T {
        _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.x)
    }

    fn boxer_point_set_x(_maybe_null_ptr: *mut ValueBox<PointBox<T>>, x: T) {
        _maybe_null_ptr.with_not_null(|point| point.x = x)
    }

    fn boxer_point_get_y(_maybe_null_ptr: *mut ValueBox<PointBox<T>>) -> T {
        _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.y)
    }

    fn boxer_point_set_y(_maybe_null_ptr: *mut ValueBox<PointBox<T>>, y: T) {
        _maybe_null_ptr.with_not_null(|point| point.y = y)
    }
}
