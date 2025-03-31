use std::{ffi::CString, str::FromStr};

use crate::{bindings::*, kiwi_error, Error, Result};

pub(crate) mod sealed {
    use crate::bindings::kiwi_typo_h;

    pub enum TypoTransformer {
        Default(super::DefaultTypoTransformer),
        Normal(super::TypoTransformer),
    }

    impl TypoTransformer {
        pub(crate) fn get_handle(&self) -> kiwi_typo_h {
            match self {
                TypoTransformer::Default(typo) => typo.handle,
                TypoTransformer::Normal(typo) => typo.handle,
            }
        }
    }

    impl From<super::DefaultTypoTransformer> for TypoTransformer {
        fn from(typo: super::DefaultTypoTransformer) -> Self {
            Self::Default(typo)
        }
    }

    impl From<super::TypoTransformer> for TypoTransformer {
        fn from(typo: super::TypoTransformer) -> Self {
            Self::Normal(typo)
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

pub enum DefaultTypoTransferOptions {
    WithoutTypo,
    BasicTypoSet,
    ContinualTypoSet,
    BasicTypoSetWithContinual,
    LengtheningTypoSet,
    BasicTypoSetWithContinualAndLengthening,
}

pub struct DefaultTypoTransformer {
    pub(crate) handle: kiwi_typo_h,
}

impl DefaultTypoTransformer {
    pub fn new(options: DefaultTypoTransferOptions) -> Result<Self> {
        TypoTransformer::default(options)
    }
}

pub struct TypoTransformer {
    pub(crate) handle: kiwi_typo_h,
}

impl TypoTransformer {
    pub fn new() -> Result<Self> {
        let handle = unsafe { kiwi_typo_init() };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(Self { handle })
    }

    pub fn basic() -> Result<DefaultTypoTransformer> {
        let handle = unsafe { kiwi_typo_get_basic() };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(DefaultTypoTransformer { handle })
    }

    pub fn default(options: DefaultTypoTransferOptions) -> Result<DefaultTypoTransformer> {
        let options = match options {
            DefaultTypoTransferOptions::WithoutTypo => KIWI_TYPO_WITHOUT_TYPO,
            DefaultTypoTransferOptions::BasicTypoSet => KIWI_TYPO_BASIC_TYPO_SET,
            DefaultTypoTransferOptions::ContinualTypoSet => KIWI_TYPO_CONTINUAL_TYPO_SET,
            DefaultTypoTransferOptions::BasicTypoSetWithContinual => {
                KIWI_TYPO_BASIC_TYPO_SET_WITH_CONTINUAL
            }
            DefaultTypoTransferOptions::LengtheningTypoSet => KIWI_TYPO_LENGTHENING_TYPO_SET,
            DefaultTypoTransferOptions::BasicTypoSetWithContinualAndLengthening => {
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
            kiwi_typo_add(
                self.handle,
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

    pub fn update(&self, other: impl Into<sealed::TypoTransformer>) -> Result<()> {
        let res = unsafe { kiwi_typo_update(self.handle, other.into().get_handle()) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn scale_cost(&self, scale: f32) -> Result<()> {
        let res = unsafe { kiwi_typo_scale_cost(self.handle, scale) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn set_continual_typo_cost(&self, threshold: f32) -> Result<()> {
        let res = unsafe { kiwi_typo_set_continual_typo_cost(self.handle, threshold) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }

    pub fn set_lengthening_typo_cost(&self, threshold: f32) -> Result<()> {
        let res = unsafe { kiwi_typo_set_lengthening_typo_cost(self.handle, threshold) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(())
    }
}

impl Drop for TypoTransformer {
    fn drop(&mut self) {
        let res = unsafe { kiwi_typo_close(self.handle) };

        if res != 0 {
            panic!("{}", kiwi_error().unwrap_or_default());
        }
    }
}
