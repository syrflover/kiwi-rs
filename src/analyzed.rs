use std::ffi::CStr;

use widestring::{U16CStr, U16String};

use crate::{bindings::*, kiwi_error, POSTag, Token};

/// [Kiwi::analyze] 또는 [Kiwi::analyze_w]의 반환 값
///
/// [Kiwi::analyze]: crate::Kiwi::analyze
/// [Kiwi::analyze_w]: crate::Kiwi::analyze_w
pub struct Analyzed {
    pub(crate) handle: kiwi_res_h,
}

impl Analyzed {
    /// 분석 결과 내에 포함된 리스트의 개수를 반환합니다.
    pub fn size(&self) -> usize {
        unsafe {
            let size = kiwi_res_size(self.handle);

            if size < 0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            size as usize
        }
    }

    /// index번째 분석 결과의 확률 점수를 반환합니다.
    pub fn prob(&self, index: usize) -> f32 {
        unsafe {
            let prob = kiwi_res_prob(self.handle, index as i32);

            if prob == 0.0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            prob
        }
    }

    /// index번째 분석 결과 내에 포함된 형태소의 개수를 반환합니다.
    pub fn word_num(&self, index: usize) -> usize {
        unsafe {
            let word_num = kiwi_res_word_num(self.handle, index as i32);

            if word_num < 0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            word_num as usize
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 정보를 반환합니다.
    ///
    /// # Return
    /// [Token] 참고
    pub fn token_info(&self, index: usize, word_num: usize) -> Token {
        unsafe {
            let token_info = kiwi_res_token_info(self.handle, index as i32, word_num as i32);

            if token_info.is_null() {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            let kiwi_token_info_t {
                chr_position,
                word_position,
                sent_position,
                line_number,
                length,
                tag,
                __bindgen_anon_1: u,
                score,
                typo_cost,
                typo_form_id,
                paired_token,
                sub_sent_position,
            } = *token_info as kiwi_token_info_t;

            Token {
                chr_position,
                word_position,
                sent_position,
                line_number,
                length,
                tag: POSTag(tag),
                sense_id: u.sense_id,
                script: u.script,
                score,
                typo_cost,
                typo_form_id,
                paired_token,
                sub_sent_position,
            }
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 형태를 반환합니다.
    ///
    /// # Return
    /// UTF-8 인코딩된 문자열
    pub fn form(&self, index: usize, word_num: usize) -> String {
        unsafe {
            let form = kiwi_res_form(self.handle, index as i32, word_num as i32);

            if form.is_null() {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            let form = CStr::from_ptr(form);

            form.to_owned().into_string().unwrap()
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 형태를 반환합니다.
    ///
    /// # Return
    /// UTF-16 인코딩된 문자열
    pub fn form_w(&self, index: usize, word_num: usize) -> U16String {
        unsafe {
            let form = kiwi_res_form_w(self.handle, index as i32, word_num as i32);

            if form.is_null() {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            let form = U16CStr::from_ptr_str(form);

            form.to_owned().into_ustring()
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 품사 태그를 반환합니다.
    ///
    /// # Return
    /// UTF-8 인코딩된 문자열
    pub fn tag(&self, index: usize, word_num: usize) -> String {
        unsafe {
            let tag = kiwi_res_tag(self.handle, index as i32, word_num as i32);

            if tag.is_null() {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            let tag = CStr::from_ptr(tag);

            tag.to_owned().into_string().unwrap()
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 품사 태그를 반환합니다.
    ///
    /// # Return
    /// UTF-16 인코딩된 문자열
    pub fn tag_w(&self, index: usize, word_num: usize) -> U16String {
        unsafe {
            let tag = kiwi_res_tag_w(self.handle, index as i32, word_num as i32);

            if tag.is_null() {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            let tag = U16CStr::from_ptr_str(tag);

            tag.to_owned().into_ustring()
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 길이(utf-16 문자열 기준)를 반환합니다.
    pub fn length(&self, index: usize, word_num: usize) -> usize {
        unsafe {
            let length = kiwi_res_length(self.handle, index as i32, word_num as i32);

            if length < 0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            length as usize
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 문장 내 어절 번호를 반환합니다.
    pub fn word_position(&self, index: usize, word_num: usize) -> usize {
        unsafe {
            let word_position = kiwi_res_word_position(self.handle, index as i32, word_num as i32);

            if word_position < 0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            word_position as usize
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 문장 번호를 반환합니다.
    pub fn sent_position(&self, index: usize, word_num: usize) -> usize {
        unsafe {
            let sent_position = kiwi_res_sent_position(self.handle, index as i32, word_num as i32);

            if sent_position < 0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            sent_position as usize
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 언어 모델 점수를 반환합니다.
    pub fn score(&self, index: usize, word_num: usize) -> f32 {
        unsafe {
            let score = kiwi_res_score(self.handle, index as i32, word_num as i32);

            if score == 0.0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            score
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 오타 교정 비용을 반환합니다.
    pub fn typo_cost(&self, index: usize, word_num: usize) -> f32 {
        unsafe {
            let typo_cost = kiwi_res_typo_cost(self.handle, index as i32, word_num as i32);

            if typo_cost < 0.0 {
                let err = kiwi_error().unwrap_or_default();
                panic!("{}", err);
            }

            typo_cost
        }
    }

    /// 모든 분석 결과를 [형태소(UTF-8)](Analyzed::form)와 [토큰 정보](Analyzed::token_info)를 묶어 리스트로 반환합니다.
    pub fn to_vec(&self) -> Vec<(String, Token)> {
        let res_size = self.size();
        let mut tokens = Vec::with_capacity(res_size);

        for i in 0..res_size {
            // ressult = (vector<token_result>, result_buffer)
            // result_buffer = ResultBuffer { vector<string> stringBuf }
            //
            // token_result = (vector<token_info>, score: float)
            // token_info

            let word_num = self.word_num(i);

            for j in 0..word_num {
                let form = self.form(i, j);
                let token = self.token_info(i, j);

                // println!("{} {}", form, token.tag);

                tokens.push((form, token));
            }
        }

        tokens
    }

    /// 모든 분석 결과를 [형태소(UTF-16)](Analyzed::form_w)와 [토큰 정보](Analyzed::token_info)를 묶어 리스트로 반환합니다.
    pub fn to_vec_w(&self) -> Vec<(U16String, Token)> {
        let res_size = self.size();
        let mut tokens = Vec::with_capacity(res_size);

        for i in 0..res_size {
            let word_num = self.word_num(i);

            for j in 0..word_num {
                let form = self.form_w(i, j);
                let token = self.token_info(i, j);

                // println!("{:?} {}", form, token.tag);

                tokens.push((form, token));
            }
        }

        tokens
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_w(&self) -> IterW {
        IterW::new(self)
    }
}

impl Drop for Analyzed {
    fn drop(&mut self) {
        let res = unsafe { kiwi_res_close(self.handle) };

        if res != 0 {
            panic!("{}", kiwi_error().unwrap_or_default());
        }
    }
}

macro_rules! impl_iterator {
    ($(
        ($struct_name:ident, $form_fn:ident, $form_ty:ty) $(,)?
    )*) => {
        $(
            pub struct $struct_name<'a> {
                analyzed: &'a Analyzed,

                i: usize,
                size: usize,

                j: usize,
                word_num: usize,
            }

            impl<'a> $struct_name<'a> {
                pub(crate) fn new(analyzed: &'a Analyzed) -> Self {
                    let size = analyzed.size();
                    let word_num = analyzed.word_num(0);

                    Self {
                        analyzed,

                        i: 0,
                        size,

                        j: 0,
                        word_num,
                    }
                }
            }

            impl Iterator for $struct_name<'_> {
                type Item = ($form_ty, Token);

                fn next(&mut self) -> Option<Self::Item> {
                    if self.i >= self.size {
                        return None;
                    }

                    if self.j >= self.word_num {
                        self.i += 1;

                        if self.i >= self.size {
                            return None;
                        }

                        self.j = 0;
                        self.word_num = self.analyzed.word_num(self.i);
                    }

                    let form = self.analyzed.$form_fn(self.i, self.j);
                    let token = self.analyzed.token_info(self.i, self.j);

                    self.j += 1;

                    Some((form, token))
                }
            }
        )*
    };
}

impl_iterator![(Iter, form, String), (IterW, form_w, U16String)];
