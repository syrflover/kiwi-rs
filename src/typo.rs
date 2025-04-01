use std::{ffi::CString, str::FromStr};

use parking_lot::Mutex;

use crate::{bindings::*, kiwi_error, Error, KiwiRc, Result};

pub(crate) mod sealed {
    use std::borrow::Cow;

    use parking_lot::MutexGuard;

    use super::*;

    pub enum TypoTransformer<'a> {
        Default(Cow<'a, super::DefaultTypoTransformer>),
        Normal(Cow<'a, super::TypoTransformer>),
    }

    impl TypoTransformer<'_> {
        pub(crate) fn get_handle(
            &self,
        ) -> (Option<kiwi_typo_h>, Option<MutexGuard<'_, *mut kiwi_typo>>) {
            match self {
                TypoTransformer::Default(t) => (Some(t.handle), None),
                TypoTransformer::Normal(t) => (None, Some(t.handle.lock())),
            }
        }
    }

    impl From<super::DefaultTypoTransformer> for TypoTransformer<'_> {
        fn from(typo: super::DefaultTypoTransformer) -> Self {
            Self::Default(Cow::Owned(typo))
        }
    }

    impl<'a> From<&'a super::DefaultTypoTransformer> for TypoTransformer<'a> {
        fn from(typo: &'a super::DefaultTypoTransformer) -> Self {
            Self::Default(Cow::Borrowed(typo))
        }
    }

    impl From<super::DefaultTypoTransformer> for Option<TypoTransformer<'_>> {
        fn from(typo: super::DefaultTypoTransformer) -> Self {
            Some(TypoTransformer::Default(Cow::Owned(typo)))
        }
    }

    impl<'a> From<&'a super::DefaultTypoTransformer> for Option<TypoTransformer<'a>> {
        fn from(typo: &'a super::DefaultTypoTransformer) -> Self {
            Some(TypoTransformer::Default(Cow::Borrowed(typo)))
        }
    }

    impl From<super::TypoTransformer> for TypoTransformer<'_> {
        fn from(typo: super::TypoTransformer) -> Self {
            Self::Normal(Cow::Owned(typo))
        }
    }

    impl<'a> From<&'a super::TypoTransformer> for TypoTransformer<'a> {
        fn from(typo: &'a super::TypoTransformer) -> Self {
            Self::Normal(Cow::Borrowed(typo))
        }
    }

    impl From<super::TypoTransformer> for Option<TypoTransformer<'_>> {
        fn from(typo: super::TypoTransformer) -> Self {
            Some(TypoTransformer::Normal(Cow::Owned(typo)))
        }
    }

    impl<'a> From<&'a super::TypoTransformer> for Option<TypoTransformer<'a>> {
        fn from(typo: &'a super::TypoTransformer) -> Self {
            Some(TypoTransformer::Normal(Cow::Borrowed(typo)))
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum CondVowel {
    /// 조건 설정되지 않음
    None,
    /// 저음, 모음 여부와 상관 없이 등장 가능
    Any,
    /// 선행 형태소가 받침이 없는 경우만 등장 가능
    Vowel,
    /// 선행 형태소가 받침이 없거나 ㄹ받침인 경우만 등장 가능
    Vocalic,
    /// 선행 형태소가 받침이 없거나 ㄹ, ㅎ 받침인 경우만 등장 가능
    VocalicH,
    /// [CondVowel::Vowel]의 부정
    NonVowel,
    /// [CondVowel::Vocalic]의 부정
    NonVocalic,
    /// [CondVowel::VocalicH]의 부정
    NonVocalicH,
    /// 불파음 받침(ㅁㄴㅇㄹ을 제외한 모든 받침)
    Applosive,
}

impl Default for CondVowel {
    fn default() -> Self {
        CondVowel::None
    }
}

pub enum DefaultTypoSet {
    WithoutTypo,
    BasicTypoSet,
    ContinualTypoSet,
    BasicTypoSetWithContinual,
    LengtheningTypoSet,
    BasicTypoSetWithContinualAndLengthening,
}

#[derive(Clone)]
pub struct DefaultTypoTransformer {
    pub(crate) handle: kiwi_typo_h,
}

#[cfg(feature = "impl_send")]
unsafe impl Send for DefaultTypoTransformer {}

impl DefaultTypoTransformer {
    pub fn new(options: DefaultTypoSet) -> Result<Self> {
        TypoTransformer::default(options)
    }
}

#[derive(Clone)]
pub struct TypoTransformer {
    pub(crate) handle: KiwiRc<Mutex<kiwi_typo_h>>,
}

#[cfg(feature = "impl_send")]
unsafe impl Send for TypoTransformer {}

impl TypoTransformer {
    pub fn new() -> Result<Self> {
        let handle = unsafe { kiwi_typo_init() };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(Self {
            #[allow(clippy::arc_with_non_send_sync)]
            handle: KiwiRc::new(Mutex::new(handle)),
        })
    }

    pub fn basic() -> Result<DefaultTypoTransformer> {
        let handle = unsafe { kiwi_typo_get_basic() };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(DefaultTypoTransformer { handle })
    }

    pub fn default(options: DefaultTypoSet) -> Result<DefaultTypoTransformer> {
        let options = match options {
            DefaultTypoSet::WithoutTypo => KIWI_TYPO_WITHOUT_TYPO,
            DefaultTypoSet::BasicTypoSet => KIWI_TYPO_BASIC_TYPO_SET,
            DefaultTypoSet::ContinualTypoSet => KIWI_TYPO_CONTINUAL_TYPO_SET,
            DefaultTypoSet::BasicTypoSetWithContinual => KIWI_TYPO_BASIC_TYPO_SET_WITH_CONTINUAL,
            DefaultTypoSet::LengtheningTypoSet => KIWI_TYPO_LENGTHENING_TYPO_SET,
            DefaultTypoSet::BasicTypoSetWithContinualAndLengthening => {
                KIWI_TYPO_BASIC_TYPO_SET_WITH_CONTINUAL_AND_LENGTHENING
            }
        };

        let handle = unsafe { kiwi_typo_get_default(options as i32) };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(DefaultTypoTransformer { handle })
    }

    pub fn add<'a>(
        &mut self,
        originals: impl Iterator<Item = &'a str>,
        errors: impl Iterator<Item = &'a str>,
        cost: f32,
        condition: impl Into<Option<CondVowel>>,
    ) -> Result<()> {
        let originals = originals
            .map(|s| CString::from_str(s).unwrap())
            .collect::<Vec<_>>();
        let mut originals = originals.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
        let errors = errors
            .map(|s| CString::from_str(s).unwrap())
            .collect::<Vec<_>>();
        let mut errors = errors.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();

        let condition = match condition.into() {
            Some(CondVowel::None) | None => CondVowel::None,
            Some(condition) => condition,
        };

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_typo_add(
                *handle,
                originals.as_mut_ptr(),
                originals.len() as i32,
                errors.as_mut_ptr(),
                errors.len() as i32,
                cost,
                condition as i32,
            )
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn update<'other>(&self, other: impl Into<sealed::TypoTransformer<'other>>) -> Result<()> {
        let other: sealed::TypoTransformer = other.into();

        let res = unsafe {
            let handle = self.handle.lock();
            let (t1, t2) = other.get_handle();
            let other = t1.or(t2.as_ref().map(|x| **x)).unwrap();
            kiwi_typo_update(*handle, other)
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn scale_cost(&self, scale: f32) -> Result<()> {
        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_typo_scale_cost(*handle, scale)
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn set_continual_typo_cost(&self, threshold: f32) -> Result<()> {
        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_typo_set_continual_typo_cost(*handle, threshold)
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn set_lengthening_typo_cost(&self, threshold: f32) -> Result<()> {
        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_typo_set_lengthening_typo_cost(*handle, threshold)
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }
}

impl Drop for TypoTransformer {
    fn drop(&mut self) {
        if KiwiRc::strong_count(&self.handle) > 1 {
            return;
        }

        let res = unsafe {
            let handle = self.handle.lock();
            kiwi_typo_close(*handle)
        };

        if res != 0 {
            panic!("{}", kiwi_error().unwrap_or_default());
        }

        tracing::trace!("closed `TypoTransformer`");
    }
}
