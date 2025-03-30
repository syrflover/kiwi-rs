#![allow(clippy::new_without_default, clippy::type_complexity)]

pub mod analyzed;
mod bindings;
mod builder;
pub mod error;
pub mod extracted;
mod kiwi;
mod r#match;
mod morpheme_set;
mod pos_tag;
mod pretokenized;
mod trampoline;
mod typo;

pub use analyzed::Analyzed;
pub use builder::*;
pub use error::*;
pub use extracted::Extracted;
pub use kiwi::*;
pub use morpheme_set::*;
pub use pos_tag::*;
pub use pretokenized::*;
pub use r#match::*;
pub use typo::*;

use std::ffi::CStr;

pub type Result<T> = std::result::Result<T, Error>;

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

        if err.is_empty() {
            return None;
        }

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
    fn test_kiwi_builder_add_rule() -> anyhow::Result<()> {
        use crate::{KiwiBuilder, POSTag};

        let mut a = "abcd".to_owned();

        let replacer = move |input: &str| {
            a.clear();
            input.to_owned()
        };

        let kiwi_builder = KiwiBuilder::new(None, Default::default())?;

        kiwi_builder.add_rule(POSTag::VV, replacer, 0.0)?;

        Ok(())
    }
}
