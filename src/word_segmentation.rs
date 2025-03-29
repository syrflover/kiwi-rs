use crate::{bindings::*, kiwi_error};

pub struct WordSegmentation {
    pub(crate) handle: kiwi_ws_h,
}

impl WordSegmentation {}

impl Drop for WordSegmentation {
    fn drop(&mut self) {
        let res = unsafe { kiwi_ws_close(self.handle) };

        if res != 0 {
            panic!("{}", kiwi_error().unwrap_or_default());
        }
    }
}
