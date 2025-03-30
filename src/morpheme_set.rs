use std::{ffi::CString, str::FromStr};

use widestring::{U16CString, U16Str};

use crate::{bindings::*, kiwi_error, Error, Kiwi, POSTag, Result};

pub struct MorphemeSet {
    pub(crate) handle: kiwi_morphset_h,
}

impl MorphemeSet {
    pub fn new(kiwi: &Kiwi) -> Self {
        let handle = unsafe { kiwi_new_morphset(kiwi.handle) };

        Self { handle }
    }

    pub fn add(&self, form: &str, tag: POSTag) -> Result<bool> {
        let form = CString::from_str(form).unwrap();
        let tag = CString::from_str(tag.as_str()).unwrap();

        unsafe {
            let res = kiwi_morphset_add(self.handle, form.as_ptr(), tag.as_ptr());

            if res < 0 {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(res > 0)
        }
    }

    pub fn add_w(&self, form: impl AsRef<U16Str>, tag: POSTag) -> Result<bool> {
        let form = U16CString::from_ustr(form).unwrap();
        let tag = CString::from_str(tag.as_str()).unwrap();

        unsafe {
            let res = kiwi_morphset_add_w(self.handle, form.as_ptr(), tag.as_ptr());

            if res < 0 {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(res > 0)
        }
    }
}

impl Drop for MorphemeSet {
    fn drop(&mut self) {
        let res = unsafe { kiwi_morphset_close(self.handle) };

        if res != 0 {
            let err = kiwi_error();
            panic!("MorhphemeSet close error: {:?}", err);
        }
    }
}
