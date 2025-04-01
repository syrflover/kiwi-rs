use std::{ffi::CString, str::FromStr};

use parking_lot::Mutex;
use widestring::{U16CString, U16Str};

use crate::{bindings::*, kiwi_error, Error, KiwiRc, POSTag, Result};

#[derive(Clone)]
pub struct Pretokenized {
    pub(crate) handle: KiwiRc<Mutex<kiwi_pretokenized_h>>,
}

#[cfg(feature = "impl_send")]
unsafe impl Send for Pretokenized {}

impl Pretokenized {
    pub fn new() -> Self {
        let handle = unsafe { kiwi_pt_init() };

        // if handle.is_null() {
        //     let err = kiwi_error().unwrap_or_default();
        //     return Err(Error::Native(err));
        // }

        Self {
            #[allow(clippy::arc_with_non_send_sync)]
            handle: KiwiRc::new(Mutex::new(handle)),
        }
    }

    /// 새 구간을 추가합니다.
    ///
    /// # Parameters
    ///
    /// * `begin` - 구간의 시작 지점
    /// * `end` - 구간의 끝 지점
    ///
    /// begin, end로 지정하는 시작/끝 지점의 단위는
    /// [Pretokenized]를 [Kiwi::analyze](crate::Kiwi::analyze)에 사용하면 UTF-8 문자열의 바이트 단위에 따라 처리되고,
    /// [Kiwi::analyze_w](crate::Kiwi::analyze_w)에 사용하면 UTF-16 문자열의 글자 단위에 따라 처리됩니다.
    ///
    /// # Return
    ///
    /// span id를 반환합니다.
    pub fn add_span(&self, begin: usize, end: usize) -> Result<u32> {
        let span_id = unsafe {
            let handle = self.handle.lock();
            kiwi_pt_add_span(*handle, begin as i32, end as i32)
        };

        if span_id < 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(span_id as u32)
    }

    /// 구간에 새 분석 결과를 추가합니다.
    ///
    /// # Parameters
    ///
    /// * `span_id` - 구간 id
    /// * `form` - 분석 결과의 형태 (utf-8)
    /// * `tag` - 분석 결과의 품사 태그
    /// * `begin` - 분석 결과의 시작 지점
    /// * `end` - 분석 결과의 끝 지점
    ///
    /// begin, end로 지정하는 시작/끝 지점의 단위는
    /// [Pretokenized]를 [Kiwi::analyze](crate::Kiwi::analyze)에 사용하면 UTF-8 문자열의 바이트 단위에 따라 처리되고,
    /// [Kiwi::analyze_w](crate::Kiwi::analyze_w)에 사용하면 UTF-16 문자열의 글자 단위에 따라 처리됩니다.
    pub fn add_token_to_span(
        &self,
        span_id: u32,
        form: &str,
        tag: POSTag,
        begin: usize,
        end: usize,
    ) -> Result<()> {
        let form = CString::from_str(form).unwrap();
        let tag = CString::from_str(tag.as_str()).unwrap();

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_pt_add_token_to_span(
                *handle,
                span_id as i32,
                form.as_ptr(),
                tag.as_ptr(),
                begin as i32,
                end as i32,
            )
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    /// 구간에 새 분석 결과를 추가합니다.
    ///
    /// # Parameters
    ///
    /// * `span_id` - 구간 id
    /// * `form` - 분석 결과의 형태 (utf-16)
    /// * `tag` - 분석 결과의 품사 태그
    /// * `begin` - 분석 결과의 시작 지점
    /// * `end` - 분석 결과의 끝 지점
    ///
    /// begin, end로 지정하는 시작/끝 지점의 단위는
    /// [Pretokenized]를 [Kiwi::analyze](crate::Kiwi::analyze)에 사용하면 UTF-8 문자열의 바이트 단위에 따라 처리되고,
    /// [Kiwi::analyze_w](crate::Kiwi::analyze_w)에 사용하면 UTF-16 문자열의 글자 단위에 따라 처리됩니다.
    pub fn add_token_to_span_w(
        &self,
        span_id: u32,
        form: impl AsRef<U16Str>,
        tag: POSTag,
        begin: usize,
        end: usize,
    ) -> Result<()> {
        let form = U16CString::from_ustr(form).unwrap();
        let tag = CString::from_str(tag.as_str()).unwrap();

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_pt_add_token_to_span_w(
                *handle,
                span_id as i32,
                form.as_ptr(),
                tag.as_ptr(),
                begin as i32,
                end as i32,
            )
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }
}

impl Drop for Pretokenized {
    fn drop(&mut self) {
        if KiwiRc::strong_count(&self.handle) > 1 {
            return;
        }

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_pt_close(*handle)
        };

        if res != 0 {
            let err = kiwi_error();
            panic!("Pretokenized close error: {:?}", err);
        }

        tracing::trace!("closed `Pretokenized`");
    }
}
