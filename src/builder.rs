use std::{
    ffi::{c_char, c_void, CString},
    path::{Path, PathBuf},
    str::FromStr,
};

use either::Either;
use widestring::U16String;

use crate::{
    bindings::*,
    kiwi_error,
    trampoline::{reader_trampoline, reader_w_trampoline, replacer_trampoline},
    Error, Kiwi, POSTag, Result, Typo, WordSegmentation,
};

/// [Kiwi] 인스턴스를 생성할 때 사용하는 옵션 구조체
///
/// 옵션에 대한 설명은 각 메서드의 설명 참고
///
/// # Default
///
/// 모델은 knlm, 나머지 옵션은 모두 활성화 됨
pub struct KiwiOptions {
    integrate_allomorph: bool,
    load_default_dict: bool,
    load_typo_dict: bool,
    load_multi_dict: bool,

    model_type_knlm: bool,
    model_type_sbg: bool,
}

impl KiwiOptions {
    /// 음운론적 이형태를 통합합니다.
    ///
    /// /아/와/ /어/나/ /았/과/ /었/ 같이 앞 모음의 양성/음성에 따라 형태 바뀌는 어미들을 하나로 통합합니다.
    pub fn integrate_allomorph(mut self, r: bool) -> Self {
        self.integrate_allomorph = r;
        self
    }

    /// 기본 사전을 불러옵니다.
    pub fn load_default_dict(mut self, r: bool) -> Self {
        self.load_default_dict = r;
        self
    }

    /// 내장 오타 사전을 불러옵니다.
    pub fn load_typo_dict(mut self, r: bool) -> Self {
        self.load_typo_dict = r;
        self
    }

    /// 내장 다어절 사전을 불러옵니다.
    pub fn load_multi_dict(mut self, r: bool) -> Self {
        self.load_multi_dict = r;
        self
    }

    /// sbg 모델에 비해 처리 속도가 빠르고, 짧은 거리 내의 형태소 간의 관계를 높은 정확도로 분석할 수 있습니다.
    pub fn model_type_knlm(mut self, r: bool) -> Self {
        self.model_type_knlm = r;
        self.model_type_sbg = !r;
        self
    }

    /// knlm 모델에 비해 처리 속도가 느리지만, 먼 거리의 형태소 간의 관계를 적당한 정확도로 분석할 수 있습니다.
    pub fn model_type_sbg(mut self, r: bool) -> Self {
        self.model_type_sbg = r;
        self.model_type_knlm = !r;
        self
    }

    pub(crate) fn finish(&self) -> u32 {
        let mut r = 0;

        if self.integrate_allomorph {
            r |= KIWI_BUILD_INTEGRATE_ALLOMORPH;
        }

        if self.load_default_dict {
            r |= KIWI_BUILD_LOAD_DEFAULT_DICT;
        }

        if self.load_typo_dict {
            r |= KIWI_BUILD_LOAD_TYPO_DICT;
        }

        if self.load_multi_dict {
            r |= KIWI_BUILD_LOAD_MULTI_DICT;
        }

        if self.model_type_knlm {
            r |= KIWI_BUILD_MODEL_TYPE_KNLM;
        }

        if self.model_type_sbg {
            r |= KIWI_BUILD_MODEL_TYPE_SBG;
        }

        r
    }
}

impl Default for KiwiOptions {
    fn default() -> Self {
        Self {
            integrate_allomorph: true,
            load_default_dict: true,
            load_typo_dict: true,
            load_multi_dict: true,

            model_type_knlm: true,
            model_type_sbg: false,
        }
    }
}

pub struct KiwiBuilder {
    handle: kiwi_builder_h,
    typo: Option<Typo>,
}

impl KiwiBuilder {
    /// 기본 모델을 사용하여 [KiwiBuilder]를 생성합니다.
    ///
    /// 모델 경로를 지정하려면 [Kiwi::with_model_path] 메서드를 이용해 주세요.
    ///
    /// # Parameters
    ///
    /// * `num_threads` - 사용할 스레드의 개수.
    ///                   `0` 또는 `None`으로 설정 시, 코어 개수만큼 스레드 생성함.
    ///                   `analyze`, `extract_*` 메서드에서 사용됨
    /// * `options` - [KiwiOptions] 참고
    pub fn new(num_threads: impl Into<Option<u32>>, options: KiwiOptions) -> Result<Self> {
        let model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("Kiwi")
            .join("models")
            .join("base");

        let model_path =
            CString::new(model_path.as_os_str().to_os_string().into_string().unwrap()).unwrap();

        let handle = unsafe {
            kiwi_builder_init(
                model_path.as_ptr(),
                num_threads.into().unwrap_or(0) as i32,
                options.finish() as i32,
            )
        };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(Self { handle, typo: None })
    }

    /// [KiwiBuilder]를 생성합니다.
    ///
    /// # Parameters
    ///
    /// * `model_path` - 모델 폴더의 경로.
    /// * `num_threads` - 사용할 스레드의 개수.
    ///                   `0` 또는 `None`으로 설정 시, 코어 개수만큼 스레드 생성함.
    ///                   `analyze`, `extract_*` 메서드에서 사용됨
    /// * `options` - [KiwiOptions] 참고
    pub fn with_model_path(
        model_path: impl AsRef<Path>,
        num_threads: impl Into<Option<u32>>,
        options: KiwiOptions,
    ) -> Result<Self> {
        let model_path = CString::new(
            model_path
                .as_ref()
                .as_os_str()
                .to_os_string()
                .into_string()
                .unwrap(),
        )
        .unwrap();

        let handle = unsafe {
            kiwi_builder_init(
                model_path.as_ptr(),
                num_threads.into().unwrap_or(0) as i32,
                options.finish() as i32,
            )
        };

        if handle.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(Self { handle, typo: None })
    }

    /// 사용자 형태소를 추가합니다.
    ///
    /// 이 함수로 등록한 형태소의 경우 언어 모델 내에서 UNK(사전 미등재 단어)로 처리됩니다.
    ///
    /// 특정 형태소의 변이형을 등록하려는 경우 [KiwiBuilder::add_alias_word] 메서드를 사용하는 걸 권장합니다.
    ///
    /// # Parameters
    ///
    /// * `word` - 추가할 형태소 (utf-8)
    /// * `pos` - 품사 태그 ([POSTag])
    /// * `score` - 점수
    pub fn add_word(self, word: &str, pos_tag: POSTag, score: f32) -> Result<Self> {
        let res = unsafe {
            let word = CString::from_str(word).unwrap();
            let pos_tag = CString::from_str(pos_tag.as_str()).unwrap();

            kiwi_builder_add_word(self.handle, word.as_ptr(), pos_tag.as_ptr(), score)
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(self)
    }

    /// 원본 형태소를 기반으로 하는 새 형태소를 추가합니다.
    ///
    /// [KiwiBuilder::add_word]로 등록한 형태소의 경우에는
    /// 언어 모델 내에서 UNK(사전 미등재 단어)로 처리되는 반면,
    /// 이 함수로 등록한 형태소의 경우 언어 모델 내에서 원본 형태소와 동일하게 처리됩니다.
    ///
    /// # Parameters
    ///
    /// * `alias` - 새 형태소 (utf-8)
    /// * `pos_tag` - 품사 태그 ([POSTag])
    /// * `score` - 점수
    /// * `origin_word` - 원본 형태소 (utf-8)
    ///
    /// # Errors
    ///
    /// origin_word에 pos_tag를 가진 원본 형태소가 존재하지 않는 경우 에러를 반환합니다.
    pub fn add_alias_word(
        self,
        alias: &str,
        pos_tag: POSTag,
        score: f32,
        origin_word: &str,
    ) -> Result<Self> {
        let res = unsafe {
            let alias = CString::from_str(alias).unwrap();
            let pos_tag = CString::from_str(pos_tag.as_str()).unwrap();
            let origin_word = CString::from_str(origin_word).unwrap();

            kiwi_builder_add_alias_word(
                self.handle,
                alias.as_ptr(),
                pos_tag.as_ptr(),
                score,
                origin_word.as_ptr(),
            )
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(self)
    }

    /// 기분석 형태소열을 추가합니다.
    ///
    /// 불규칙적으로 분석되어야하는 패턴을 추가하는데 용이합니다.
    ///
    /// 예) 사겼다 -> 사귀/VV + 었/EP + 다/EF
    ///
    /// # Parameters
    ///
    /// * `form` - 등록할 형태 (utf-8)
    /// * `analyzed` - 분석되어야할 각 형태소의 형태와 품사 그리고 각 형태소가 형태 내에서 차지하는 위치를 지정합니다.
    /// * `score` - 점수. 기본적으로는 0을 사용합니다. 0보다 클 경우 이 분석 결과가 더 높은 우선 순위를, 작을 경우 더 낮은 우선 순위를 갖습니다.
    ///
    /// # Errors
    ///
    /// 주어진 형태와 품사로 지정된 형태소가 사전 내에 존재하지 않는 경우에 에러를 반환합니다.
    ///
    /// # Example
    ///
    /// ## Without positions
    ///
    /// ```rust
    /// use either::Either;
    /// use kiwi::{KiwiBuilder, POSTag};
    ///
    /// let kiwi_builder = KiwiBuilder::new(None, Default::default()).unwrap();
    ///
    /// let without_positions = vec![("사귀", POSTag::VV), ("었", POSTag::EP), ("다", POSTag::EF)];
    ///
    /// kiwi_builder.add_pre_analyzed_word("사겼다", Either::Left(&*without_positions), -3.0).unwrap();
    /// ```
    ///
    /// ## With positions
    ///
    /// ```rust
    /// use either::Either;
    /// use kiwi::{KiwiBuilder, POSTag};
    ///
    /// let kiwi_builder = KiwiBuilder::new(None, Default::default()).unwrap();
    ///
    /// let with_positions = vec![("사귀", POSTag::VV, 0, 2), ("었", POSTag::EP, 1, 2), ("다", POSTag::EF, 2, 3)];
    ///
    /// kiwi_builder.add_pre_analyzed_word("사겼다", Either::Right(&*with_positions), -3.0).unwrap();
    /// ```
    pub fn add_pre_analyzed_word(
        self,
        form: &str,
        analyzed: Either<&[(&str, POSTag)], &[(&str, POSTag, usize, usize)]>,
        score: f32,
    ) -> Result<Self> {
        let (analyzed_morphs, analyzed_pos_tags, positions) = match analyzed {
            Either::Left(analyzed) => (
                analyzed.iter().map(|x| x.0).collect::<Vec<_>>(),
                analyzed.iter().map(|x| x.1).collect::<Vec<_>>(),
                None,
            ),
            Either::Right(analyzed) => {
                let mut positions = Vec::with_capacity(analyzed.len() * 2);

                for (.., start, end) in analyzed {
                    positions.push(*start as i32);
                    positions.push(*end as i32);
                }

                (
                    analyzed.iter().map(|x| x.0).collect::<Vec<_>>(),
                    analyzed.iter().map(|x| x.1).collect::<Vec<_>>(),
                    Some(positions),
                )
            }
        };

        let form = CString::from_str(form).unwrap();

        let analyzed_morphs = analyzed_morphs
            .iter()
            .map(|morph| CString::from_str(morph).unwrap())
            .collect::<Vec<_>>();
        let mut analyzed_morphs = analyzed_morphs
            .iter()
            .map(|morph| morph.as_ptr())
            .collect::<Vec<_>>();

        let analyzed_pos_tags = analyzed_pos_tags
            .iter()
            .map(|pos_tag| CString::from_str(pos_tag.as_str()).unwrap())
            .collect::<Vec<_>>();
        let mut analyzed_pos_tags = analyzed_pos_tags
            .iter()
            .map(|pos_tag| pos_tag.as_ptr())
            .collect::<Vec<_>>();

        let res = unsafe {
            kiwi_builder_add_pre_analyzed_word(
                self.handle,
                form.as_ptr(),
                analyzed_morphs.len() as i32,
                analyzed_morphs.as_mut_ptr(),
                analyzed_pos_tags.as_mut_ptr(),
                score,
                positions
                    .as_ref()
                    .map(|x| x.as_ptr())
                    .unwrap_or(std::ptr::null()),
            )
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(self)
    }

    /// 규칙에 의해 변형된 형태소 목록을 생성하여 자동 추가합니다.
    ///
    /// # Parameters
    ///
    /// * `pos_tag` - 변형할 형태소의 품사 태그
    /// * `replacer` - 변형 결과를 제공하는데 쓰일 함수
    /// * `score` - 기본적으로는 0을 사용합니다. 0보다 클 경우 이 변형 결과가 더 높은 우선 순위를, 작을 경우 더 낮은 우선 순위를 갖습니다.
    pub fn add_rule<F>(self, pos_tag: POSTag, replacer: F, score: f32) -> Result<Self>
    where
        F: FnMut(&str) -> String,
    {
        // in 64bit system
        // dyn (vtable ptr) -> 8bytes
        // Box (heap ptr) -> 8bytes
        // total -> 16bytes
        //
        // 캡처되는 변수가 없으면 0
        // println!("{}", std::mem::size_of::<Box<dyn FnMut(&str) -> String>>());
        // println!("{}", std::mem::size_of::<F>());
        // println!("{}", std::mem::size_of_val(&replacer));

        let replacer = Box::into_raw(Box::new(Box::new(replacer)));

        // println!("{}", std::mem::size_of_val(&replacer));

        let pos_tag = CString::from_str(pos_tag.as_str()).unwrap();

        let res = unsafe {
            // println!("replacer {:?}", replacer);

            let r = kiwi_builder_add_rule(
                self.handle,
                pos_tag.as_ptr(),
                Some(replacer_trampoline::<F>),
                replacer as *mut c_void,
                score,
            );

            drop(Box::from_raw(replacer));

            r
        };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(self)
    }

    pub fn load_dict(self, dict_path: &str) -> Result<Self> {
        let dict_path = CString::from_str(dict_path).unwrap();

        let res = unsafe { kiwi_builder_load_dict(self.handle, dict_path.as_ptr()) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(self)
    }

    pub fn extract_words<F>(
        &self,
        reader: F,
        min_cnt: i32,
        max_word_len: i32,
        min_score: f32,
        pos_threshold: f32,
    ) -> Result<WordSegmentation>
    where
        F: FnMut(i32) -> String,
    {
        let reader = Box::into_raw(Box::new(Box::new(reader)));

        unsafe {
            let ws = kiwi_builder_extract_words(
                self.handle,
                Some(reader_trampoline::<F>),
                reader as *mut c_void,
                min_cnt,
                max_word_len,
                min_score,
                pos_threshold,
            );

            drop(Box::from_raw(reader));

            if ws.is_null() {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(WordSegmentation { handle: ws })
        }
    }

    pub fn extract_add_words<F>(
        &self,
        reader: F,
        min_cnt: i32,
        max_word_len: i32,
        min_score: f32,
        pos_threshold: f32,
    ) -> Result<WordSegmentation>
    where
        F: FnMut(i32) -> String,
    {
        let reader = Box::into_raw(Box::new(Box::new(reader)));

        unsafe {
            let ws = kiwi_builder_extract_add_words(
                self.handle,
                Some(reader_trampoline::<F>),
                reader as *mut c_void,
                min_cnt,
                max_word_len,
                min_score,
                pos_threshold,
            );

            drop(Box::from_raw(reader));

            if ws.is_null() {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(WordSegmentation { handle: ws })
        }
    }

    pub fn extract_words_w<F>(
        &self,
        reader_w: F,
        min_cnt: i32,
        max_word_len: i32,
        min_score: f32,
        pos_threshold: f32,
    ) -> Result<WordSegmentation>
    where
        F: FnMut(i32) -> U16String,
    {
        let reader_w = Box::into_raw(Box::new(Box::new(reader_w)));

        unsafe {
            let ws = kiwi_builder_extract_words_w(
                self.handle,
                Some(reader_w_trampoline::<F>),
                reader_w as *mut c_void,
                min_cnt,
                max_word_len,
                min_score,
                pos_threshold,
            );

            drop(Box::from_raw(reader_w));

            if ws.is_null() {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(WordSegmentation { handle: ws })
        }
    }

    pub fn extract_add_words_w<F>(
        &self,
        reader_w: F,
        min_cnt: i32,
        max_word_len: i32,
        min_score: f32,
        pos_threshold: f32,
    ) -> Result<WordSegmentation>
    where
        F: FnMut(i32) -> U16String,
    {
        let reader_w = Box::into_raw(Box::new(Box::new(reader_w)));

        unsafe {
            let ws = kiwi_builder_extract_add_words_w(
                self.handle,
                Some(reader_w_trampoline::<F>),
                reader_w as *mut c_void,
                min_cnt,
                max_word_len,
                min_score,
                pos_threshold,
            );

            drop(Box::from_raw(reader_w));

            if ws.is_null() {
                let err = kiwi_error().unwrap_or_default();
                return Err(Error::Native(err));
            }

            Ok(WordSegmentation { handle: ws })
        }
    }

    pub fn typo(mut self, mut typo: Typo, typo_cost_threshold: f32) -> Self {
        typo.typo_cost_threshold = typo_cost_threshold;
        self.typo.replace(typo);
        self
    }

    pub fn build(mut self) -> Result<Kiwi> {
        let kiwi = unsafe {
            match self.typo.take() {
                Some(typo) => {
                    kiwi_builder_build(self.handle, typo.handle, typo.typo_cost_threshold)
                }
                None => kiwi_builder_build(self.handle, std::ptr::null_mut(), 0.0),
            }
        };

        if kiwi.is_null() {
            let err = kiwi_error().unwrap_or_default();
            return Err(Error::Native(err));
        }

        Ok(Kiwi { handle: kiwi })
    }
}

impl Drop for KiwiBuilder {
    fn drop(&mut self) {
        let res = unsafe { kiwi_builder_close(self.handle) };

        if res != 0 {
            let err = kiwi_error().unwrap_or_default();
            panic!("KiwiBuilder close error: {}", err);
        }
    }
}
