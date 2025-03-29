use crate::{bindings::*, kiwi_error};

pub enum TypoOptions {
    WithoutTypo,
    BasicTypoSet,
    ContinualTypoSet,
    BasicTypoSetWithContinual,
    LengtheningTypoSet,
    BasicTypoSetWithContinualAndLengthening,
}

pub struct Typo {
    pub(crate) handle: kiwi_typo_h,
    pub(crate) typo_cost_threshold: f32,

    pub(crate) use_close: bool,
    pub(crate) is_defualt: bool,
}

impl Typo {
    pub fn new() -> Self {
        let handle = unsafe { kiwi_typo_init() };

        Self {
            handle,
            typo_cost_threshold: 0.0,
            use_close: true,
            is_defualt: false,
        }
    }

    pub fn basic() -> Self {
        let handle = unsafe { kiwi_typo_get_basic() };

        Self {
            handle,
            typo_cost_threshold: 0.0,
            use_close: false,
            is_defualt: false,
        }
    }

    pub fn standard(options: TypoOptions) -> Self {
        let options = match options {
            TypoOptions::WithoutTypo => KIWI_TYPO_WITHOUT_TYPO,
            TypoOptions::BasicTypoSet => KIWI_TYPO_BASIC_TYPO_SET,
            TypoOptions::ContinualTypoSet => KIWI_TYPO_CONTINUAL_TYPO_SET,
            TypoOptions::BasicTypoSetWithContinual => KIWI_TYPO_BASIC_TYPO_SET_WITH_CONTINUAL,
            TypoOptions::LengtheningTypoSet => KIWI_TYPO_LENGTHENING_TYPO_SET,
            TypoOptions::BasicTypoSetWithContinualAndLengthening => {
                KIWI_TYPO_BASIC_TYPO_SET_WITH_CONTINUAL_AND_LENGTHENING
            }
        };

        let handle = unsafe { kiwi_typo_get_default(options as i32) };

        Self {
            handle,
            typo_cost_threshold: 0.0,
            use_close: false,
            is_defualt: true,
        }
    }

    // pub fn add(
    //     &mut self,
    //     original: &str,
    //     original_size: i32,
    //     error: &str,
    //     error_size: i32,
    //     cost: f32,
    //     condition: i32,
    // ) {
    //     if self.is_defualt {
    //         return;
    //     }

    //     let res = unsafe {
    //         kiwi_typo_add(handle, orig, orig_size, error, error_size, cost, condition)
    //     };
    // }
}

impl Drop for Typo {
    fn drop(&mut self) {
        if !self.use_close {
            return;
        }

        let res = unsafe { kiwi_typo_close(self.handle) };

        if res != 0 {
            panic!("{}", kiwi_error().unwrap_or_default());
        }
    }
}
