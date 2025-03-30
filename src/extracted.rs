use std::{ffi::CStr, fmt::Debug};

use widestring::{U16CStr, U16String};

use crate::{bindings::*, kiwi_error};

#[derive(Debug, Clone)]
pub struct Word<S>
where
    S: Debug + Clone,
{
    pub form: S,
    pub score: f32,
    pub pos_score: f32,
    pub freq: usize,
}

pub struct Extracted {
    pub(crate) handle: kiwi_ws_h,
    size: usize,
}

#[inline]
fn size(handle: kiwi_ws_h) -> usize {
    unsafe {
        let size = kiwi_ws_size(handle);

        if size < 0 {
            let err = kiwi_error();
            panic!("word_segments.size() -> {:?}", err);
        }

        size as usize
    }
}

impl Extracted {
    pub(crate) fn new(handle: kiwi_ws_h) -> Self {
        let size = size(handle);

        Self { handle, size }
    }

    #[inline]
    fn check_index(&self, index: usize) -> Option<()> {
        (self.size > index).then_some(())
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn form(&self, index: usize) -> Option<String> {
        self.check_index(index)?;
        Some(self.form_unchecked(index))
    }

    #[inline]
    fn form_unchecked(&self, index: usize) -> String {
        unsafe {
            let form = kiwi_ws_form(self.handle, index as i32);

            if form.is_null() {
                let err = kiwi_error();
                panic!("word_segments.form({}) -> {:?}", index, err);
            }

            let form = CStr::from_ptr(form);

            form.to_owned().into_string().unwrap()
        }
    }

    pub fn form_w(&self, index: usize) -> Option<U16String> {
        self.check_index(index)?;
        Some(self.form_w_unchecked(index))
    }

    #[inline]
    fn form_w_unchecked(&self, index: usize) -> U16String {
        unsafe {
            let form_w = kiwi_ws_form_w(self.handle, index as i32);

            if form_w.is_null() {
                let err = kiwi_error();
                panic!("word_segments.form_w({}) -> {:?}", index, err);
            }

            let form_w = U16CStr::from_ptr_str(form_w);

            form_w.to_owned().into_ustring()
        }
    }

    pub fn score(&self, index: usize) -> Option<f32> {
        self.check_index(index)?;
        Some(self.score_unchecked(index))
    }

    #[inline]
    fn score_unchecked(&self, index: usize) -> f32 {
        unsafe {
            let score = kiwi_ws_score(self.handle, index as i32);

            if score == 0.0 {
                let err = kiwi_error();
                panic!("word_segments.score({}) -> {:?}", index, err);
            }

            score
        }
    }

    pub fn freq(&self, index: usize) -> Option<usize> {
        self.check_index(index)?;
        Some(self.freq_unchecked(index))
    }

    #[inline]
    fn freq_unchecked(&self, index: usize) -> usize {
        unsafe {
            let freq = kiwi_ws_freq(self.handle, index as i32);

            if freq < 0 {
                let err = kiwi_error();
                panic!("word_segments.freq({}) -> {:?}", index, err);
            }

            freq as usize
        }
    }

    pub fn pos_score(&self, index: usize) -> Option<f32> {
        self.check_index(index)?;
        Some(self.pos_score_unchecked(index))
    }

    #[inline]
    fn pos_score_unchecked(&self, index: usize) -> f32 {
        unsafe {
            let pos_score = kiwi_ws_pos_score(self.handle, index as i32);

            if pos_score == 0.0 {
                let err = kiwi_error();
                panic!("word_segments.pos_score({}) -> {:?}", index, err);
            }

            pos_score
        }
    }

    pub fn to_vec(&self) -> Vec<Word<String>> {
        let ws_size = self.size();
        let mut words = Vec::with_capacity(ws_size);

        for i in 0..ws_size {
            let form = self.form_unchecked(i);
            let freq = self.freq_unchecked(i);
            let score = self.score_unchecked(i);
            let pos_score = self.pos_score_unchecked(i);

            let word_info = Word {
                form,
                freq,
                score,
                pos_score,
            };

            words.push(word_info);
        }

        words
    }

    pub fn to_vec_w(&self) -> Vec<Word<U16String>> {
        let ws_size = self.size();
        let mut words = Vec::with_capacity(ws_size);

        for i in 0..ws_size {
            let form = self.form_w_unchecked(i);
            let freq = self.freq_unchecked(i);
            let score = self.score_unchecked(i);
            let pos_score = self.pos_score_unchecked(i);

            let word_info = Word {
                form,
                freq,
                score,
                pos_score,
            };

            words.push(word_info);
        }

        words
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_w(&self) -> IterW {
        IterW::new(self)
    }
}

impl Drop for Extracted {
    fn drop(&mut self) {
        let res = unsafe { kiwi_ws_close(self.handle) };

        if res != 0 {
            panic!(
                "WordSegments close error: {}",
                kiwi_error().unwrap_or_default()
            );
        }
    }
}

macro_rules! impl_iterator {
    ($(
        (
            $struct_name:ident,
            $form_fn:ident,
            $item_ty:ty $(,)?
            $(=> $item_mapper:ident)? $(,)?
        )
        $(,)?
    )*) => {
        $(
            pub struct $struct_name<'a> {
                word_segments: &'a Extracted,
                i: usize,
            }

            impl<'a> $struct_name<'a> {
                pub(crate) fn new(word_segments: &'a Extracted) -> Self {
                    Self {
                        word_segments,
                        i: 0,
                    }
                }
            }

            impl Iterator for $struct_name<'_> {
                type Item = $item_ty;

                fn next(&mut self) -> Option<Self::Item> {
                    self.word_segments.check_index(self.i)?;

                    let form = self.word_segments.$form_fn(self.i);
                    let freq = self.word_segments.freq_unchecked(self.i);
                    let score = self.word_segments.score_unchecked(self.i);
                    let pos_score = self.word_segments.pos_score_unchecked(self.i);

                    self.i += 1;

                    let item = Word {
                        form,
                        freq,
                        score,
                        pos_score,
                    };

                    Some(item)
                }
            }
        )*
    };
}

impl_iterator![
    (Iter, form_unchecked, Word<String>),
    (IterW, form_w_unchecked, Word<U16String>),
];
