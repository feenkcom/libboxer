use std::ops::Range;
use string_box::StringBox;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn boxer_string_create() -> *mut ValueBox<StringBox> {
    ValueBox::new(StringBox::new()).into_raw()
}

/// I copy the data (must *not* contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub fn boxer_string_from_byte_string(data: *const u8, length: usize) -> *mut ValueBox<StringBox> {
    ValueBox::new(unsafe { StringBox::from_byte_string_data(data, length) }).into_raw()
}

/// I copy the data (must *not* contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub fn boxer_string_from_wide_string(data: *const u32, length: usize) -> *mut ValueBox<StringBox> {
    ValueBox::new(unsafe { StringBox::from_wide_string_data(data, length) }).into_raw()
}

/// I copy the data (must contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub fn boxer_string_from_utf8_string(data: *const u8, length: usize) -> *mut ValueBox<StringBox> {
    ValueBox::new(unsafe { StringBox::from_utf8_string_data(data, length) }).into_raw()
}

#[no_mangle]
pub fn boxer_string_drop(ptr: *mut ValueBox<StringBox>) {
    ptr.release();
}

#[no_mangle]
pub fn boxer_string_get_len(ptr: *mut ValueBox<StringBox>) -> usize {
    ptr.with_not_null_return(0, |string| string.len())
}

#[no_mangle]
pub fn boxer_string_get_char_count(ptr: *mut ValueBox<StringBox>) -> usize {
    ptr.with_not_null_return(0, |string| string.char_count())
}

#[no_mangle]
pub fn boxer_string_get_ptr(ptr: *mut ValueBox<StringBox>) -> *const u8 {
    ptr.with_not_null_return(std::ptr::null(), |string| string.as_ptr())
}

#[no_mangle]
pub fn boxer_string_char_index_to_byte_range(
    string_ptr: *mut ValueBox<StringBox>,
    index: usize,
    range_ptr: *mut ValueBox<Range<usize>>,
) {
    string_ptr.with_not_null(|string| {
        range_ptr.with_not_null(|range| {
            let byte_range = string.char_index_to_byte_range(index);
            range.start = byte_range.start;
            range.end = byte_range.end;
        })
    })
}

#[no_mangle]
pub fn boxer_string_char_index_to_utf16_range(
    string_ptr: *mut ValueBox<StringBox>,
    index: usize,
    range_ptr: *mut ValueBox<Range<usize>>,
) {
    string_ptr.with_not_null(|string| {
        range_ptr.with_not_null(|range| {
            let byte_range = string.char_index_to_utf16_range(index);
            range.start = byte_range.start;
            range.end = byte_range.end;
        })
    })
}

#[no_mangle]
pub fn boxer_string_utf16_position_to_char_index(
    string_ptr: *mut ValueBox<StringBox>,
    index: usize,
) -> usize {
    string_ptr.with_not_null_return(0, |string| string.utf16_position_to_char_index(index))
}
