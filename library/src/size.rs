use geometry_box::SizeBox;
use value_box::{ValueBox, ValueBoxPointer};

pub trait SizeBoxFFI<T>
where
    T: From<u8> + Default + Copy,
{
    fn boxer_size_create() -> *mut ValueBox<SizeBox<T>>;

    fn boxer_size_drop(ptr: *mut ValueBox<SizeBox<T>>);

    fn boxer_size_get_width(_ptr: *mut ValueBox<SizeBox<T>>) -> T;

    fn boxer_size_set_width(_ptr: *mut ValueBox<SizeBox<T>>, width: T);

    fn boxer_size_get_height(_ptr: *mut ValueBox<SizeBox<T>>) -> T;

    fn boxer_size_set_height(_ptr: *mut ValueBox<SizeBox<T>>, height: T);
}

impl<T> SizeBoxFFI<T> for SizeBox<T>
where
    T: From<u8> + Default + Copy,
{
    fn boxer_size_create() -> *mut ValueBox<SizeBox<T>> {
        ValueBox::new(SizeBox::<T>::default()).into_raw()
    }

    fn boxer_size_drop(ptr: *mut ValueBox<SizeBox<T>>) {
        ptr.release();
    }

    fn boxer_size_get_width(_ptr: *mut ValueBox<SizeBox<T>>) -> T {
        _ptr.with_not_null_return(0u8.into(), |size| size.width)
    }

    fn boxer_size_set_width(_ptr: *mut ValueBox<SizeBox<T>>, width: T) {
        _ptr.with_not_null(|size| size.width = width);
    }

    fn boxer_size_get_height(_ptr: *mut ValueBox<SizeBox<T>>) -> T {
        _ptr.with_not_null_return(0u8.into(), |size| size.height)
    }

    fn boxer_size_set_height(_ptr: *mut ValueBox<SizeBox<T>>, height: T) {
        _ptr.with_not_null(|size| size.height = height);
    }
}
