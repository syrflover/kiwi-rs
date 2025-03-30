use std::ffi::CStr;

use widestring::{U16CStr, U16String};

use crate::{bindings::*, kiwi_error, POSTag};

#[derive(Debug, Clone, Copy)]
pub struct Token {
    /// 시작 위치 (UTF16 문자 기준)
    pub chr_position: u32,
    /// 어절 번호 (공백 기준)
    pub word_position: u32,
    /// 문장 번호
    pub sent_position: u32,
    /// 줄 번호
    pub line_number: u32,
    /// 길이 (UTF16 문자 기준)
    pub length: u16,
    /// 품사 태그
    pub tag: POSTag,
    /// 해당 형태소의 언어모델 점수
    pub score: f32,
    /// 오타가 교정한 오타 비용. 그렇지 않는 경우 0
    pub typo_cost: f32,
    /// 교정 전 오타의 형태에 대한 정보 (typo_cost가 0인 경우 PreTokenizedSpan의 ID값)
    pub typo_form_id: u32,
    /// SSO, SSC 태그에 속하는 형태소의 경우 쌍을 이루는 반대쪽 형태소의 위치 (-1인 경우 해당하는 형태소가 없다는 것)
    pub paired_token: u32,
    /// 인용 부호나 괄호로 둘러싸인 하위 문장의 번호. 1부터 시작. 0인 경우 하위 문장이 아님
    pub sub_sent_position: u32,

    /// 의미 번호
    pub sense_id: u8,
    /// 유니코드 영역에 기반한 문자 타입
    pub script: u8,
}

/// [Kiwi::analyze] 또는 [Kiwi::analyze_w]의 반환 값
///
/// [Kiwi::analyze]: crate::Kiwi::analyze
/// [Kiwi::analyze_w]: crate::Kiwi::analyze_w
pub struct Analyzed {
    pub(crate) handle: kiwi_res_h,
    size: usize,
    /// words\[n\] = word_num
    words: Box<[usize]>,
}

#[inline]
fn size(handle: kiwi_res_h) -> usize {
    unsafe {
        let size = kiwi_res_size(handle);

        if size < 0 {
            let err = kiwi_error();
            panic!("analyzed.size() -> {:?}", err);
        }

        size as usize
    }
}

#[inline]
fn word_num(handle: kiwi_res_h, index: usize) -> usize {
    unsafe {
        let word_num = kiwi_res_word_num(handle, index as i32);

        if word_num < 0 {
            let err = kiwi_error();
            panic!("analyzed.word_num({}) -> {:?}", index, err);
        }

        word_num as usize
    }
}

impl Analyzed {
    pub(crate) fn new(handle: kiwi_res_h) -> Self {
        let size = size(handle);
        let mut words = Vec::with_capacity(size);

        for i in 0..size {
            words.push(word_num(handle, i));
        }

        Self {
            handle,
            size,
            words: words.into_boxed_slice(),
        }
    }

    #[inline]
    fn check_index(&self, i: usize, j: impl Into<Option<usize>>) -> Option<()> {
        match j.into() {
            Some(j) => (self.word_num(i)? > j).then_some(()),
            None => (self.size > i).then_some(()),
        }
    }

    /// 분석 결과 내에 포함된 리스트의 개수를 반환합니다.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// index번째 분석 결과의 확률 점수를 반환합니다.
    pub fn prob(&self, index: usize) -> Option<f32> {
        self.check_index(index, None)?;

        unsafe {
            let prob = kiwi_res_prob(self.handle, index as i32);

            if prob == 0.0 {
                let err = kiwi_error();
                panic!("analyzed.prob({}) -> {:?}", index, err);
            }

            Some(prob)
        }
    }

    /// index번째 분석 결과 내에 포함된 형태소의 개수를 반환합니다.
    #[inline]
    pub fn word_num(&self, index: usize) -> Option<usize> {
        self.words.get(index).copied()
    }

    /// index번째 분석 결과의 word_num번째 형태소의 정보를 반환합니다.
    ///
    /// # Return
    /// [Token] 참고
    pub fn token(&self, index: usize, word_num: usize) -> Option<Token> {
        self.check_index(index, word_num)?;
        Some(self.token_unchecked(index, word_num))
    }

    #[inline]
    fn token_unchecked(&self, index: usize, word_num: usize) -> Token {
        unsafe {
            let token = kiwi_res_token_info(self.handle, index as i32, word_num as i32);

            if token.is_null() {
                let err = kiwi_error();
                panic!("analyzed.token({}, {}) -> {:?}", index, word_num, err);
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
            } = *token as kiwi_token_info_t;

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
    pub fn form(&self, index: usize, word_num: usize) -> Option<String> {
        self.check_index(index, word_num)?;
        Some(self.form_unchecked(index, word_num))
    }

    #[inline]
    fn form_unchecked(&self, index: usize, word_num: usize) -> String {
        unsafe {
            let form = kiwi_res_form(self.handle, index as i32, word_num as i32);

            if form.is_null() {
                let err = kiwi_error();
                panic!("analyzed.form({}, {}) -> {:?}", index, word_num, err);
            }

            let form = CStr::from_ptr(form);

            form.to_owned().into_string().unwrap()
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 형태를 반환합니다.
    ///
    /// # Return
    /// UTF-16 인코딩된 문자열
    pub fn form_w(&self, index: usize, word_num: usize) -> Option<U16String> {
        self.check_index(index, word_num)?;
        Some(self.form_w_unchecked(index, word_num))
    }

    #[inline]
    fn form_w_unchecked(&self, index: usize, word_num: usize) -> U16String {
        unsafe {
            let form = kiwi_res_form_w(self.handle, index as i32, word_num as i32);

            if form.is_null() {
                let err = kiwi_error();
                panic!("analyzed.form_w({}, {}) -> {:?}", index, word_num, err);
            }

            let form = U16CStr::from_ptr_str(form);

            form.to_owned().into_ustring()
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 품사 태그를 반환합니다.
    ///
    /// # Return
    /// UTF-8 인코딩된 문자열
    pub fn tag(&self, index: usize, word_num: usize) -> Option<String> {
        self.check_index(index, word_num)?;

        unsafe {
            let tag = kiwi_res_tag(self.handle, index as i32, word_num as i32);

            if tag.is_null() {
                let err = kiwi_error();
                panic!("analyzed.tag({}, {}) -> {:?}", index, word_num, err);
            }

            let tag = CStr::from_ptr(tag);

            Some(tag.to_owned().into_string().unwrap())
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 품사 태그를 반환합니다.
    ///
    /// # Return
    /// UTF-16 인코딩된 문자열
    pub fn tag_w(&self, index: usize, word_num: usize) -> Option<U16String> {
        self.check_index(index, word_num)?;

        unsafe {
            let tag = kiwi_res_tag_w(self.handle, index as i32, word_num as i32);

            if tag.is_null() {
                let err = kiwi_error();
                panic!("analyzed.tag_w({}, {}) -> {:?}", index, word_num, err);
            }

            let tag = U16CStr::from_ptr_str(tag);

            Some(tag.to_owned().into_ustring())
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 길이(utf-16 문자열 기준)를 반환합니다.
    pub fn length(&self, index: usize, word_num: usize) -> Option<usize> {
        self.check_index(index, word_num)?;

        unsafe {
            let length = kiwi_res_length(self.handle, index as i32, word_num as i32);

            if length < 0 {
                let err = kiwi_error();
                panic!("analyzed.length({}, {}) -> {:?}", index, word_num, err);
            }

            Some(length as usize)
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 문장 내 어절 번호를 반환합니다.
    pub fn word_position(&self, index: usize, word_num: usize) -> Option<usize> {
        self.check_index(index, word_num)?;

        unsafe {
            let word_position = kiwi_res_word_position(self.handle, index as i32, word_num as i32);

            if word_position < 0 {
                let err = kiwi_error();
                panic!(
                    "analyzed.word_position({}, {}) -> {:?}",
                    index, word_num, err
                );
            }

            Some(word_position as usize)
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 문장 번호를 반환합니다.
    pub fn sent_position(&self, index: usize, word_num: usize) -> Option<usize> {
        self.check_index(index, word_num)?;

        unsafe {
            let sent_position = kiwi_res_sent_position(self.handle, index as i32, word_num as i32);

            if sent_position < 0 {
                let err = kiwi_error();
                panic!(
                    "analyzed.sent_position({}, {}) -> {:?}",
                    index, word_num, err
                );
            }

            Some(sent_position as usize)
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 언어 모델 점수를 반환합니다.
    pub fn score(&self, index: usize, word_num: usize) -> Option<f32> {
        self.check_index(index, word_num)?;

        unsafe {
            let score = kiwi_res_score(self.handle, index as i32, word_num as i32);

            if score == 0.0 {
                let err = kiwi_error();
                panic!("analyzed.score({}, {}) -> {:?}", index, word_num, err);
            }

            Some(score)
        }
    }

    /// index번째 분석 결과의 word_num번째 형태소의 오타 교정 비용을 반환합니다.
    pub fn typo_cost(&self, index: usize, word_num: usize) -> Option<f32> {
        self.check_index(index, word_num)?;

        unsafe {
            let typo_cost = kiwi_res_typo_cost(self.handle, index as i32, word_num as i32);

            if typo_cost < 0.0 {
                let err = kiwi_error();
                panic!("analyzed.typo_cost({}, {}) -> {:?}", index, word_num, err);
            }

            Some(typo_cost)
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

            let word_num = self.word_num(i).unwrap();

            for j in 0..word_num {
                let form = self.form_unchecked(i, j);
                let token = self.token_unchecked(i, j);

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
            let word_num = self.word_num(i).unwrap();

            for j in 0..word_num {
                let form = self.form_w_unchecked(i, j);
                let token = self.token_unchecked(i, j);

                // println!("{:?} {}", form, token.tag);

                tokens.push((form, token));
            }
        }

        tokens
    }

    /// 모든 분석 결과의 [토큰 정보](Analyzed::token_info)를 리스트로 반환합니다.
    pub fn to_vec_t(&self) -> Vec<Token> {
        let res_size = self.size();
        let mut tokens = Vec::with_capacity(res_size);

        for i in 0..res_size {
            let word_num = self.word_num(i).unwrap();

            for j in 0..word_num {
                let token = self.token_unchecked(i, j);

                // println!("{} {}", form, token.tag);

                tokens.push(token);
            }
        }

        tokens
    }

    /// 모든 분석 결과를 [형태소(UTF-8)](Analyzed::form)와 [토큰 정보](Analyzed::token_info)를 묶은 이터레이터 구조체를 반환합니다.
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    /// 모든 분석 결과를 [형태소(UTF-16)](Analyzed::form_w)와 [토큰 정보](Analyzed::token_info)를 묶은 이터레이터 구조체를 반환합니다.
    pub fn iter_w(&self) -> IterW {
        IterW::new(self)
    }

    /// 모든 분석 결과의 [토큰 정보](Analyzed::token_info)의 이터레이터 구조체를 반환합니다.
    pub fn iter_t(&self) -> IterT {
        IterT::new(self)
    }
}

impl Drop for Analyzed {
    fn drop(&mut self) {
        let res = unsafe { kiwi_res_close(self.handle) };

        if res != 0 {
            let err = kiwi_error();
            panic!("Analyzed close error: {:?}", err);
        }
    }
}

macro_rules! impl_iterator {
    ($(
        (
            $struct_name:ident,
            ($($item_fn:ident $(,)?)+),
            $item_ty:ty $(,)?
            $(=> $item_mapper:ident)? $(,)?
        )
        $(,)?
    )*) => {
        $(
            pub struct $struct_name<'a> {
                analyzed: &'a Analyzed,
                i: usize,
                j: usize,
            }

            impl<'a> $struct_name<'a> {
                pub(crate) fn new(analyzed: &'a Analyzed) -> Self {
                    Self {
                        analyzed,
                        i: 0,
                        j: 0,
                    }
                }
            }

            impl Iterator for $struct_name<'_> {
                type Item = $item_ty;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.analyzed.check_index(self.i, self.j).is_none() {
                        self.i += 1;
                        self.j = 0;
                        self.analyzed.check_index(self.i, None)?;
                    }

                    let item = ($(self.analyzed.$item_fn(self.i, self.j),)+);
                    $(let item = $item_mapper(item);)?

                    self.j += 1;

                    Some(item)
                }
            }
        )*
    };
}

#[inline]
fn flat(item: (Token,)) -> Token {
    item.0
}

impl_iterator![
    (Iter, (form_unchecked, token_unchecked), (String, Token)),
    (IterW, (form_w_unchecked, token_unchecked), (U16String, Token)),
    (IterT, (token_unchecked), Token => flat),
];
