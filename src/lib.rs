#![allow(
    clippy::new_without_default,
    clippy::type_complexity,
    clippy::derivable_impls
)]

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
pub use typo::{DefaultTypoSet, DefaultTypoTransformer, TypoTransformer};

use std::ffi::CStr;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "impl_send")]
type KiwiRc<T> = std::sync::Arc<T>;
#[cfg(not(feature = "impl_send"))]
type KiwiRc<T> = std::rc::Rc<T>;

pub fn kiwi_version() -> String {
    let cstr = unsafe { CStr::from_ptr(bindings::kiwi_version()) };

    cstr.to_owned().into_string().unwrap()
}

pub fn get_script_name(script: u8) -> Option<String> {
    unsafe {
        let script_name = CStr::from_ptr(bindings::kiwi_get_script_name(script));
        let script_name = script_name.to_owned().into_string().unwrap();

        if script_name == "Unknown" {
            return None;
        }

        Some(script_name)
    }
}

pub(crate) fn kiwi_error() -> Option<String> {
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
    use crate::{get_script_name, kiwi_version};

    #[test]
    fn test_kiwi_version() {
        let v = kiwi_version();

        assert_eq!(v, "0.20.4", "{}", v);
    }

    #[test]
    fn test_kiwi_script_name() {
        let script_name = get_script_name(255);

        assert!(script_name.is_none(), "{:?}", script_name);

        let script_name = get_script_name(2).unwrap();

        assert_eq!("IPA Extensions", script_name, "{:?}", script_name);
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
