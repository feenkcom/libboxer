use std::ops::Range;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn boxer_string_create() -> *mut ValueBox<StringBox> {
    value_box!(StringBox::new()).into_raw()
}

/// I copy the data (must *not* contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub extern "C" fn boxer_string_from_byte_string(
    data: *const u8,
    length: usize,
) -> *mut ValueBox<StringBox> {
    ValueBox::new(unsafe { StringBox::from_byte_string_data(data, length) }).into_raw()
}

/// I copy the data (must *not* contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub extern "C" fn boxer_string_from_wide_string(
    data: *const u32,
    length: usize,
) -> *mut ValueBox<StringBox> {
    ValueBox::new(unsafe { StringBox::from_wide_string_data(data, length) }).into_raw()
}

/// I copy the data (must contain zero-byte).
/// length must not include the zero-byte
#[no_mangle]
pub extern "C" fn boxer_string_from_utf8_string(
    data: *const u8,
    length: usize,
) -> *mut ValueBox<StringBox> {
    ValueBox::new(unsafe { StringBox::from_utf8_string_data(data, length) }).into_raw()
}

#[no_mangle]
pub extern "C" fn boxer_string_drop(string_box: *mut ValueBox<StringBox>) {
    string_box.release();
}

#[no_mangle]
pub extern "C" fn boxer_string_get_len(string_box: *mut ValueBox<StringBox>) -> usize {
    string_box.with_ref_ok(|string| string.len()).or_log(0)
}

#[no_mangle]
pub extern "C" fn boxer_string_get_char_count(string_box: *mut ValueBox<StringBox>) -> usize {
    string_box
        .with_ref_ok(|string| string.char_count())
        .or_log(0)
}

#[no_mangle]
pub extern "C" fn boxer_string_get_ptr(string_box: *mut ValueBox<StringBox>) -> *const u8 {
    string_box
        .with_ref_ok(|string| string.as_ptr())
        .or_log(std::ptr::null())
}

#[no_mangle]
pub extern "C" fn boxer_string_char_index_to_byte_range(
    string_ptr: *mut ValueBox<StringBox>,
    index: usize,
    range_ptr: *mut ValueBox<Range<usize>>,
) {
    string_ptr
        .with_ref(|string| {
            range_ptr.with_mut_ok(|range| {
                let byte_range = string.char_index_to_byte_range(index);
                range.start = byte_range.start;
                range.end = byte_range.end;
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn boxer_string_char_index_to_utf16_range(
    string_box: *mut ValueBox<StringBox>,
    index: usize,
    range_ptr: *mut ValueBox<Range<usize>>,
) {
    string_box
        .with_ref(|string| {
            range_ptr.with_mut_ok(|range| {
                let byte_range = string.char_index_to_utf16_range(index);
                range.start = byte_range.start;
                range.end = byte_range.end;
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn boxer_string_utf16_position_to_char_index(
    string_box: *mut ValueBox<StringBox>,
    index: usize,
) -> usize {
    string_box
        .with_ref_ok(|string| string.utf16_position_to_char_index(index))
        .or_log(0)
}
