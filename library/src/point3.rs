use geometry_box::Point3Box;
use std::any::Any;
use value_box::{ValueBox, ValueBoxPointer};

pub trait Point3BoxFFI<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> *mut ValueBox<Point3Box<T>>;

    fn boxer_point_create(x: T, y: T, z: T) -> *mut ValueBox<Point3Box<T>>;

    fn boxer_point_drop(ptr: *mut ValueBox<Point3Box<T>>);

    fn boxer_point_get_x(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T;

    fn boxer_point_set_x(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, x: T);

    fn boxer_point_get_y(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T;

    fn boxer_point_set_y(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, y: T);

    fn boxer_point_get_z(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T;

    fn boxer_point_set_z(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, z: T);
}

impl<T> Point3BoxFFI<T> for Point3Box<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> *mut ValueBox<Point3Box<T>> {
        ValueBox::new(Point3Box::<T>::default()).into_raw()
    }

    fn boxer_point_create(x: T, y: T, z: T) -> *mut ValueBox<Point3Box<T>> {
        ValueBox::new(Point3Box::<T>::new(x, y, z)).into_raw()
    }

    fn boxer_point_drop(ptr: *mut ValueBox<Point3Box<T>>) {
        ptr.release();
    }

    fn boxer_point_get_x(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T {
        _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.x)
    }

    fn boxer_point_set_x(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, x: T) {
        _maybe_null_ptr.with_not_null(|point| point.x = x)
    }

    fn boxer_point_get_y(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T {
        _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.y)
    }

    fn boxer_point_set_y(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, y: T) {
        _maybe_null_ptr.with_not_null(|point| point.y = y)
    }

    fn boxer_point_get_z(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T {
        _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.z)
    }

    fn boxer_point_set_z(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, z: T) {
        _maybe_null_ptr.with_not_null(|point| point.z = z)
    }
}
