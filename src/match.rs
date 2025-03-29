use crate::bindings::*;

/// 분석 시 특수한 문자열 패턴 중 어떤 것들을 추출할 지 선택할 수 있습니다.
///
/// 옵션에 대한 설명은 각 메서드의 설명 참고
///
/// # Default
/// 모든 옵션이 비활성됨
#[derive(Debug, Clone, Copy)]
pub struct Match {
    url: bool,
    email: bool,
    hashtag: bool,
    mention: bool,
    serial: bool,
    emoji: bool,

    normalize_coda: bool,
    join_noun_prefix: bool,
    join_noun_suffix: bool,
    join_verb_suffix: bool,
    join_adj_suffix: bool,
    join_adv_suffix: bool,

    split_complex: bool,
    z_coda: bool,
    compatible_jamo: bool,
    split_saisiot: bool,
    merge_saisiot: bool,
}

impl Default for Match {
    fn default() -> Self {
        Self::new()
    }
}

impl Match {
    /// 모든 플래그가 비활성화된 [Match] 구조체를 반환합니다.
    pub fn new() -> Self {
        Self::none()
    }

    fn none() -> Self {
        Self {
            url: false,
            email: false,
            hashtag: false,
            mention: false,
            serial: false,
            emoji: false,

            normalize_coda: false,
            join_noun_prefix: false,
            join_noun_suffix: false,
            join_verb_suffix: false,
            join_adj_suffix: false,
            join_adv_suffix: false,

            split_complex: false,
            z_coda: false,
            compatible_jamo: false,
            split_saisiot: false,
            merge_saisiot: false,
        }
    }

    /// 인터넷 주소 형태의 텍스트를 w_url 태그로 추출합니다.
    pub fn url(mut self, r: bool) -> Self {
        self.url = r;
        self
    }

    /// 이메일 주소 형태의 텍스트를 w_email 태그로 추출합니다.
    pub fn email(mut self, r: bool) -> Self {
        self.email = r;
        self
    }

    /// 해시태그(#해시태그) 형태의 텍스트를 w_hashtag 태그로 추출합니다.
    pub fn hashtag(mut self, r: bool) -> Self {
        self.hashtag = r;
        self
    }

    /// 멘션(@멘션) 형태의 텍스트를 w_mention 태그로 추출합니다.
    pub fn mention(mut self, r: bool) -> Self {
        self.mention = r;
        self
    }

    /// 일련번호 형태의 텍스트를 w_serial 태그로 추출합니다.
    pub fn serial(mut self, r: bool) -> Self {
        self.serial = r;
        self
    }

    /// 이모지 문자를 w_emoji 태그로 추출합니다.
    pub fn emoji(mut self, r: bool) -> Self {
        self.emoji = r;
        self
    }

    /// url, email, hashtag, mention, serial, emoji 모두 추출합니다.
    ///
    /// 아래의 옵션들이 모두 활성화 됩니다.
    ///
    /// * [Match::url]
    /// * [Match::email]
    /// * [Match::hashtag]
    /// * [Match::mention]
    /// * [Match::serial]
    /// * [Match::emoji]
    pub fn all(self) -> Self {
        Self {
            url: true,
            email: true,
            hashtag: true,
            mention: true,
            serial: true,
            ..self
        }
    }

    /// '먹었엌ㅋㅋ'처럼 받침이 덧붙어서 분석에 실패하는 경우, 받침을 분리하여 정규화합니다.
    pub fn normalize_coda(mut self, r: bool) -> Self {
        self.normalize_coda = r;
        self
    }

    /// 명사의 접두사를 분리하지 않고 결합합니다. 풋/XPN 사과/NNG -> 풋사과/NNG
    pub fn join_noun_prefix(mut self, r: bool) -> Self {
        self.join_noun_prefix = r;
        self
    }

    /// 명사의 접미사를 분리하지 않고 결합합니다. 사과/NNG 들/XSN -> 사과들/NNG
    pub fn join_noun_suffix(mut self, r: bool) -> Self {
        self.join_noun_suffix = r;
        self
    }

    /// 동사 파생접미사를 분리하지 않고 결합합니다. 사랑/NNG 하/XSV 다/EF -> 사랑하/VV 다/EF
    pub fn join_verb_suffix(mut self, r: bool) -> Self {
        self.join_verb_suffix = r;
        self
    }

    ///  형용사 파생접미사를 분리하지 않고 결합합니다. 매콤/XR 하/XSA 다/EF -> 매콤하/VA 다/EF
    pub fn join_adj_suffix(mut self, r: bool) -> Self {
        self.join_adj_suffix = r;
        self
    }

    /// 부사 파생접미사를 분리하지 않고 결합합니다. 요란/XR 히/XSM -> 요란히/MAG
    pub fn join_adv_suffix(mut self, r: bool) -> Self {
        self.join_adv_suffix = r;
        self
    }

    /// 더 잘게 분할 가능한 형태소를 모두 분할합니다. 고마움/NNG -> 고맙/VA-I 음/ETN
    pub fn split_complex(mut self, r: bool) -> Self {
        self.split_complex = r;
        self
    }

    /// 조사/어미에 덧붙은 받침을 Z_CODA 태그로 분리합니다. 했어욗 -> 하/VV 었/EP 어요/EF ㄳ/Z_CODA
    pub fn z_coda(mut self, r: bool) -> Self {
        self.z_coda = r;
        self
    }

    /// 형태소 분석 결과 출력 시 첫가끝 자모를 호환용 자모로 변환합니다.
    pub fn compatible_jamo(mut self, r: bool) -> Self {
        self.compatible_jamo = r;
        self
    }

    /// 사이시옷이 포함된 합성명사를 분리합니다. 만둣국 -> 만두/NNG ᆺ/Z_SIOT 국/NNG
    pub fn split_saisiot(mut self, r: bool) -> Self {
        self.split_saisiot = r;
        self
    }

    /// 사이시옷이 포함된 것으로 추정되는 명사를 결합합니다. 만둣국 -> 만둣국/NNG
    pub fn merge_saisiot(mut self, r: bool) -> Self {
        self.merge_saisiot = r;
        self
    }

    /// 아래의 옵션들이 모두 활성화 됩니다.
    ///
    /// * [Match::all]
    /// * [Match::normalize_coda]
    pub fn all_with_normailize_coda(self) -> Self {
        Self {
            normalize_coda: true,
            ..self.all()
        }
    }

    /// 동사/형용사형 파생접미사를 분리하지 않고 결합합니다.
    ///
    /// 아래의 옵션들이 모두 활성화 됩니다.
    ///
    /// * [Match::join_verb_suffix]
    /// * [Match::join_adj_suffix]
    pub fn join_v_suffix(self) -> Self {
        Self {
            join_verb_suffix: true,
            join_adj_suffix: true,
            ..self
        }
    }

    /// 모든 접두사/접미사를 분리하지 않고 결합합니다.
    ///
    /// 아래의 옵션들이 모두 활성화 됩니다.
    ///
    /// * [Match::join_noun_prefix]
    /// * [Match::join_noun_suffix]
    /// * [Match::join_adv_suffix]
    /// * [Match::join_v_suffix]
    pub fn join_affix(self) -> Self {
        Self {
            join_noun_prefix: true,
            join_noun_suffix: true,
            join_adv_suffix: true,
            ..self.join_v_suffix()
        }
    }

    pub(crate) fn finish(&self) -> i32 {
        let Match {
            url,
            email,
            hashtag,
            mention,
            serial,
            emoji,
            normalize_coda,
            join_noun_prefix,
            join_noun_suffix,
            join_verb_suffix,
            join_adj_suffix,
            join_adv_suffix,
            split_complex,
            z_coda,
            compatible_jamo,
            split_saisiot,
            merge_saisiot,
        } = *self;

        let mut flag = 0;

        if url {
            flag |= KIWI_MATCH_URL;
        }
        if email {
            flag |= KIWI_MATCH_EMAIL;
        }
        if hashtag {
            flag |= KIWI_MATCH_HASHTAG;
        }
        if mention {
            flag |= KIWI_MATCH_MENTION;
        }
        if serial {
            flag |= KIWI_MATCH_SERIAL;
        }
        if emoji {
            flag |= 1 << 5;
        }
        if normalize_coda {
            flag |= KIWI_MATCH_NORMALIZE_CODA;
        }
        if join_noun_prefix {
            flag |= KIWI_MATCH_JOIN_NOUN_PREFIX;
        }
        if join_noun_suffix {
            flag |= KIWI_MATCH_JOIN_NOUN_SUFFIX;
        }
        if join_verb_suffix {
            flag |= KIWI_MATCH_JOIN_VERB_SUFFIX;
        }
        if join_adj_suffix {
            flag |= KIWI_MATCH_JOIN_ADJ_SUFFIX;
        }
        if join_adv_suffix {
            flag |= KIWI_MATCH_JOIN_ADV_SUFFIX;
        }
        if split_complex {
            flag |= KIWI_MATCH_SPLIT_COMPLEX;
        }
        if z_coda {
            flag |= KIWI_MATCH_Z_CODA;
        }
        if compatible_jamo {
            flag |= KIWI_MATCH_COMPATIBLE_JAMO;
        }
        if split_saisiot {
            flag |= KIWI_MATCH_SPLIT_SAISIOT;
        }
        if merge_saisiot {
            flag |= KIWI_MATCH_MERGE_SAISIOT;
        }

        flag as i32
    }
}
