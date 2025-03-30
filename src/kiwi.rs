use std::{ffi::CString, str::FromStr};

use widestring::{U16CString, U16Str};

use crate::{bindings::*, kiwi_error, Analyzed, Error, Match, MorphSet, Pretokenized, Result};

pub struct Kiwi {
    pub(crate) handle: kiwi_h,
}

impl Kiwi {
    // pub(crate) fn new<P>(
    //     model_path: P,
    //     num_threads: Option<u32>,
    //     options: KiwiBuilderOptions,
    // ) -> Self
    // where
    //     P: AsRef<Path>,
    // {
    //     let model_path = CString::new(
    //         model_path
    //             .as_ref()
    //             .as_os_str()
    //             .to_os_string()
    //             .into_string()
    //             .unwrap(),
    //     )
    //     .unwrap();

    //     let handle = unsafe {
    //         kiwi_init(
    //             model_path.as_ptr() as *const c_char,
    //             num_threads.unwrap_or(0) as i32,
    //             options.finish() as i32,
    //         )
    //     };

    //     Self {
    //         handle: ManuallyDrop::new(handle),
    //         model_path,
    //         typo: None,
    //     }
    // }

    pub fn set_integrate_allomorph(&self, r: bool) {
        let value = if r { 1 } else { 0 };

        unsafe {
            kiwi_set_option(self.handle, KIWI_BUILD_INTEGRATE_ALLOMORPH as i32, value);
        }
    }

    pub fn get_integrate_allomorph(&self) -> bool {
        unsafe {
            let r = kiwi_get_option(self.handle, KIWI_BUILD_INTEGRATE_ALLOMORPH as i32);
            r != 0
        }
    }

    pub fn set_max_unk_form_size(&self, r: u32) {
        unsafe {
            kiwi_set_option(self.handle, KIWI_MAX_UNK_FORM_SIZE as i32, r as i32);
        }
    }

    pub fn get_max_unk_form_size(&self) -> u32 {
        unsafe { kiwi_get_option(self.handle, KIWI_MAX_UNK_FORM_SIZE as i32) as u32 }
    }

    pub fn set_space_tolerance(&self, r: u32) {
        unsafe {
            kiwi_set_option(self.handle, KIWI_SPACE_TOLERANCE as i32, r as i32);
        }
    }

    pub fn get_space_tolerance(&self) -> u32 {
        unsafe { kiwi_get_option(self.handle, KIWI_SPACE_TOLERANCE as i32) as u32 }
    }

    pub fn set_cut_off_threshold(&self, r: f32) {
        unsafe {
            kiwi_set_option_f(self.handle, KIWI_CUT_OFF_THRESHOLD as i32, r);
        }
    }

    pub fn get_cut_off_threshold(&self) -> f32 {
        unsafe { kiwi_get_option_f(self.handle, KIWI_CUT_OFF_THRESHOLD as i32) }
    }

    pub fn set_unk_form_score_scale(&self, r: f32) {
        unsafe {
            kiwi_set_option_f(self.handle, KIWI_UNK_FORM_SCORE_SCALE as i32, r);
        }
    }

    pub fn get_unk_form_score_scale(&self) -> f32 {
        unsafe { kiwi_get_option_f(self.handle, KIWI_UNK_FORM_SCORE_SCALE as i32) }
    }

    pub fn set_unk_form_score_bias(&self, r: f32) {
        unsafe {
            kiwi_set_option_f(self.handle, KIWI_UNK_FORM_SCORE_BIAS as i32, r);
        }
    }

    pub fn get_unk_form_score_bias(&self) -> f32 {
        unsafe { kiwi_get_option_f(self.handle, KIWI_UNK_FORM_SCORE_BIAS as i32) }
    }

    pub fn set_space_penalty(&self, r: f32) {
        unsafe {
            kiwi_set_option_f(self.handle, KIWI_SPACE_PENALTY as i32, r);
        }
    }

    pub fn get_space_penalty(&self) -> f32 {
        unsafe { kiwi_get_option_f(self.handle, KIWI_SPACE_PENALTY as i32) }
    }

    /// 텍스트를 분석해 형태소 결과를 반환합니다.
    ///
    /// # Parameters
    /// * `text` - 분석할 텍스트 (utf-8)
    /// * `top_n` - 분석 결과 후보를 상위 몇개까지 생성할지 설정합니다.
    /// * `match_options` - [Match] 참고
    /// * `blocklist` - 분석 후보 탐색 과정에서 blocklist에 포함된 형태소들은 배제됩니다.
    /// * `pretokenized` - 입력 텍스트 중 특정 영역의 분석 방법을 강제로 지정합니다.
    ///
    /// # Return
    /// [Analyzed] 참고
    pub fn analyze<'a>(
        &self,
        text: &str,
        top_n: i32,
        match_options: Match,
        blocklist: impl Into<Option<&'a MorphSet>>,
        pretokenized: impl Into<Option<&'a Pretokenized>>,
    ) -> Result<Analyzed> {
        let blocklist: Option<&MorphSet> = blocklist.into();
        let pretokenized: Option<&Pretokenized> = pretokenized.into();

        unsafe {
            let text = CString::from_str(text).unwrap();
            let blocklist = match blocklist {
                Some(blocklist) => blocklist.handle,
                None => std::ptr::null::<kiwi_morphset>() as *mut _,
            };
            let pretokenized = match pretokenized {
                Some(pretokenized) => pretokenized.handle,
                None => std::ptr::null::<kiwi_pretokenized>() as *mut _,
            };

            let res = kiwi_analyze(
                self.handle,
                text.as_ptr(),
                top_n,
                match_options.finish(),
                blocklist,
                pretokenized,
            );

            if res.is_null() {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(Analyzed::new(res))
        }
    }

    /// 텍스트를 분석해 형태소 결과를 반환합니다.
    ///
    /// # Parameters
    /// * `text` - 분석할 텍스트 (utf-16)
    /// * `top_n` - 분석 결과 후보를 상위 몇개까지 생성할지 설정합니다.
    /// * `match_options` - [Match] 참고
    /// * `blocklist` - 분석 후보 탐색 과정에서 blocklist에 포함된 형태소들은 배제됩니다.
    /// * `pretokenized` - 입력 텍스트 중 특정 영역의 분석 방법을 강제로 지정합니다.
    ///
    /// # Return
    /// [Analyzed] 참고
    pub fn analyze_w<'a>(
        &self,
        text: impl AsRef<U16Str>,
        top_n: i32,
        match_options: Match,
        blocklist: impl Into<Option<&'a MorphSet>>,
        pretokenized: impl Into<Option<&'a Pretokenized>>,
    ) -> Result<Analyzed> {
        let blocklist: Option<&MorphSet> = blocklist.into();
        let pretokenized: Option<&Pretokenized> = pretokenized.into();

        unsafe {
            let text = U16CString::from_ustr(text).unwrap();
            let blocklist = match blocklist {
                Some(blocklist) => blocklist.handle,
                None => std::ptr::null::<kiwi_morphset>() as *mut _,
            };
            let pretokenized = match pretokenized {
                Some(pretokenized) => pretokenized.handle,
                None => std::ptr::null::<kiwi_pretokenized>() as *mut _,
            };

            let res = kiwi_analyze_w(
                self.handle,
                text.as_ptr(),
                top_n,
                match_options.finish(),
                blocklist,
                pretokenized,
            );

            if res.is_null() {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(Analyzed::new(res))
        }
    }
}

impl Drop for Kiwi {
    fn drop(&mut self) {
        let res = unsafe { kiwi_close(self.handle) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            panic!("Kiwi close error: {}", err);
        }
    }
}
