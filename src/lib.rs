#![allow(clippy::new_without_default, clippy::type_complexity)]

mod analyzed;
mod bindings;
mod builder;
mod kiwi;
mod r#match;
mod morpheme_set;
mod pos_tag;
mod pretokenized;
mod token;
mod trampoline;
mod typo;
mod word_segmentation;

pub use analyzed::*;
pub use builder::*;
pub use kiwi::*;
pub use morpheme_set::*;
pub use pos_tag::*;
pub use pretokenized::*;
pub use r#match::*;
pub use token::*;
pub use typo::*;
pub use word_segmentation::*;

use std::ffi::CStr;

pub type KiwiTokenInfo = bindings::kiwi_token_info_t;

pub fn kiwi_version() -> String {
    let cstr = unsafe { CStr::from_ptr(bindings::kiwi_version()) };

    cstr.to_owned().into_string().unwrap()
}

pub fn kiwi_error() -> Option<String> {
    unsafe {
        let err = bindings::kiwi_error();

        if err.is_null() {
            return None;
        }

        let err = CStr::from_ptr(err).to_owned().into_string().unwrap();
        bindings::kiwi_clear_error();

        Some(err)
    }
}

#[cfg(test)]
mod tests {
    use crate::kiwi_version;

    #[test]
    fn test_kiwi_version() {
        let v = kiwi_version();

        assert_eq!(v, "0.20.4");
    }

    #[test]
    fn test_kiwi_builder_add_rule() {
        use crate::{KiwiBuilder, POSTag};

        let mut a = "abcd".to_owned();

        let replacer = move |input: &str| {
            a.clear();
            input.to_owned()
        };

        let kiwi_builder = KiwiBuilder::new("./Kiwi/models/base", None, Default::default());

        kiwi_builder.add_rule(POSTag::VV, replacer, 0.0);
    }
}
