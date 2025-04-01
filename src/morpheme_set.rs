use std::{ffi::CString, str::FromStr};

use parking_lot::Mutex;
use widestring::{U16CString, U16Str};

use crate::{bindings::*, kiwi_error, Error, Kiwi, KiwiRc, POSTag, Result};

#[derive(Clone)]
pub struct MorphemeSet {
    pub(crate) handle: KiwiRc<Mutex<kiwi_morphset_h>>,
}

#[cfg(feature = "impl_send")]
unsafe impl Send for MorphemeSet {}

impl MorphemeSet {
    pub fn new(kiwi: &Kiwi) -> Self {
        let handle = unsafe {
            let kiwi_handle = kiwi.handle.read();
            kiwi_new_morphset(*kiwi_handle)
        };

        Self {
            #[allow(clippy::arc_with_non_send_sync)]
            handle: KiwiRc::new(Mutex::new(handle)),
        }
    }

    pub fn add(&self, form: &str, tag: POSTag) -> Result<bool> {
        let form = CString::from_str(form).unwrap();
        let tag = CString::from_str(tag.as_str()).unwrap();

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_morphset_add(*handle, form.as_ptr(), tag.as_ptr())
        };

        if res < 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(res > 0)
    }

    pub fn add_w(&self, form: impl AsRef<U16Str>, tag: POSTag) -> Result<bool> {
        let form = U16CString::from_ustr(form).unwrap();
        let tag = CString::from_str(tag.as_str()).unwrap();

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_morphset_add_w(*handle, form.as_ptr(), tag.as_ptr())
        };

        if res < 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(res > 0)
    }
}

impl Drop for MorphemeSet {
    fn drop(&mut self) {
        if KiwiRc::strong_count(&self.handle) > 1 {
            return;
        }

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_morphset_close(*handle)
        };

        if res != 0 {
            let err = kiwi_error();
            panic!("MorhphemeSet close error: {:?}", err);
        }

        tracing::trace!("closed `MorphemeSet`");
    }
}
