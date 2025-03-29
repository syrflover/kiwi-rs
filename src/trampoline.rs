use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    str::FromStr,
};

use widestring::{U16CString, U16String};

use crate::bindings::kchar16_t;

pub(crate) extern "C" fn replacer_trampoline<F>(
    input: *const c_char,
    _len: c_int,
    ret: *mut c_char,
    replacer: *mut c_void,
) -> c_int
where
    F: FnMut(&str) -> String,
{
    // FIXME: remove unwrap
    let input = unsafe { CStr::from_ptr(input) }.to_str().unwrap();

    let replacer = unsafe { &mut *(replacer as *mut Box<F>) };

    let res = CString::from_str(&replacer(input)).unwrap();
    let len = res.as_bytes().len();

    if ret.is_null() {
        return len as c_int;
    }

    unsafe {
        // memcpy
        // src dest 메모리 영역이 겹치지 않음
        std::ptr::copy_nonoverlapping(res.as_ptr(), ret, len);
    }

    len as c_int
}

pub(crate) extern "C" fn reader_trampoline<F>(
    idx: c_int,
    ret: *mut c_char,
    reader: *mut c_void,
) -> c_int
where
    F: FnMut(i32) -> String,
{
    let reader = unsafe { &mut *(reader as *mut Box<F>) };

    let res = CString::from_str(&reader(idx)).unwrap();
    let len = res.as_bytes().len();

    if ret.is_null() {
        return len as c_int;
    }

    unsafe {
        // memcpy
        // src dest 메모리 영역이 겹치지 않음
        std::ptr::copy_nonoverlapping(res.as_ptr(), ret, len);
    }

    len as c_int
}

pub(crate) extern "C" fn reader_w_trampoline<F>(
    idx: c_int,
    ret: *mut kchar16_t,
    reader_w: *mut c_void,
) -> c_int
where
    F: FnMut(i32) -> U16String,
{
    let reader_w = unsafe { &mut *(reader_w as *mut Box<F>) };

    let res = U16CString::from_ustr(reader_w(idx)).unwrap();
    let len = res.len();

    if ret.is_null() {
        return len as c_int;
    }

    unsafe {
        // memcpy
        // src dest 메모리 영역이 겹치지 않음
        std::ptr::copy_nonoverlapping(res.as_ptr(), ret, len);
    }

    len as c_int
}
