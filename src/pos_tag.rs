use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct POSTag(pub(crate) u8);

macro_rules! impl_pos_tag {
    ($(
        $(#[$docs:meta])*
        ($num:expr, $name:ident, $phrase:expr),
    )*) => {
        impl POSTag {
            $(
                $(#[$docs])*
                pub const $name: POSTag = POSTag($num);
            )*

            /// 분할된 동사/형용사를 나타내는데 사용됨
            pub const P: POSTag = POSTag(60);
            /// POSTag의 총 개수
            pub const MAX: u8 = 61;

            pub const fn get_num(&self) -> u8 {
                self.0
            }

            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        &POSTag::$name => $phrase,
                    )*
                    _ => "_"
                }
            }
        }


    };
}

impl Display for POSTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for POSTag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl_pos_tag![
    /// 분석 불가
    (0, UNK, "UNK"),
    /// 체언 - 일반 명사
    (1, NNG, "NNG"),
    /// 체언 - 고유 명사
    (2, NNP, "NNP"),
    /// 체언 - 의존 명사
    (3, NNB, "NNB"),
    /// 용언 - 동사
    (4, VV, "VV"),
    /// 용언 - 형용사
    (5, VA, "VA"),
    /// 부사 - 일반 부사
    (6, MAG, "MAG"),
    /// 체언 - 수사
    (7, NR, "NR"),
    /// 체언 - 대명사
    (8, NP, "NP"),
    /// 용언 - 보조 용언
    (9, VX, "VX"),
    /// 관형사
    (10, MM, "MM"),
    /// 부사 - 접속 부사
    (11, MAJ, "MAJ"),
    /// 감탄사
    (12, IC, "IC"),
    /// 체언 - 접두사
    (13, XPN, "XPN"),
    /// 접미사 - 명사 파생 접미사
    (14, XSN, "XSN"),
    /// 접미사 - 동사 파생 접미사
    (15, XSV, "XSV"),
    /// 접미사 - 형용사 파생 접미사
    (16, XSA, "XSA"),
    /// 접미사 - 부사 파생 접미사
    (17, XSM, "XSM"),
    /// 어근
    (18, XR, "XR"),
    /// 용언 - 긍정 지시사(이다)
    (19, VCP, "VCP"),
    /// 용언 - 부정 지시사(아니다)
    (20, VCN, "VCN"),
    /// 부호,외국어,특수문자 - 종결 부호(. ! ?)
    (21, SF, "SF"),
    /// 부호,외국어,특수문자 - 구분 부호(, / : ;)
    (22, SP, "SP"),
    /// 부호,외국어,특수문자 - 인용 부호 및 괄호(' " () [] <> {} - ‘ ’ “ ” ≪ ≫ 등)
    (23, SS, "SS"),
    /// 부호,외국어,특수문자 - SS 중 여는 부호
    (24, SSO, "SSO"),
    /// 부호,외국어,특수문자 - SS 중 닫는 부호
    (25, SSC, "SSC"),
    /// 부호,외국어,특수문자 - 줄임표(…)
    (26, SE, "SE"),
    /// 부호,외국어,특수문자 - 붙임표(- ~)
    (27, SO, "SO"),
    /// 부호,외국어,특수문자 - 기타 특수 문자
    (28, SW, "SW"),
    /// 부호,외국어,특수문자 - 순서 있는 글머리(가. 나. 1. 2. 가) 나) 등)
    (29, SB, "SB"),
    /// 부호,외국어,특수문자 - 알파벳(A-Z a-z)
    (30, SL, "SL"),
    /// 부호,외국어,특수문자 - 한자
    (31, SH, "SH"),
    /// 부호,외국어,특수문자 - 숫자(0-9)
    (32, SN, "SN"),
    /// 웹 - URL 주소
    (33, W_URL, "W_URL"),
    /// 웹 - 이메일 주소
    (34, W_EMAIL, "W_EMAIL"),
    /// 웹 - 멘션(@abcd)
    (35, W_MENTION, "W_MENTION"),
    /// 웹 - 해시태그(#abcd)
    (36, W_HASHTAG, "W_HASHTAG"),
    /// 웹 - 일련번호(전화번호, 통장번호, IP주소 등)
    (37, W_SERIAL, "W_SERIAL"),
    /// 웹 - 이모지
    (38, W_EMOJI, "W_EMOJI"),
    /// 조사 - 주격 조사
    (39, JKS, "JKS"),
    /// 조사 - 보격 조사
    (40, JKC, "JKC"),
    /// 조사 - 관형격 조사
    (41, JKG, "JKG"),
    /// 조사 - 목적격 조사
    (42, JKO, "JKO"),
    /// 조사 - 부사격 조사
    (43, JKB, "JKB"),
    /// 조사 - 호격 조사
    (44, JKV, "JKV"),
    /// 조사 - 인용격 조사
    (45, JKQ, "JKQ"),
    /// 조사 - 보조사
    (46, JX, "JX"),
    /// 조사 - 접속 조사
    (47, JC, "JC"),
    /// 어미 - 선어말 어미
    (48, EP, "EP"),
    /// 어미 - 종결 어미
    (49, EF, "EF"),
    /// 어미 - 연결 어미
    (50, EC, "EC"),
    /// 어미 - 명사형 전성 어미
    (51, ETN, "ETN"),
    /// 어미 - 관형형 전성 어미
    (52, ETM, "ETM"),
    /// 기타 - 덧붙은 받침
    (53, Z_CODA, "Z_CODA"),
    /// 기타 - 사이시옷
    (54, Z_SIOT, "Z_SIOT"),
    /// 사용자 정의 태그 0
    (55, USER0, "USER0"),
    /// 사용자 정의 태그 1
    (56, USER1, "USER1"),
    /// 사용자 정의 태그 2
    (57, USER2, "USER2"),
    /// 사용자 정의 태그 3
    (58, USER3, "USER3"),
    /// 사용자 정의 태그 4
    (59, USER4, "USER4"),
    (60, PV, "PV"),
    (POSTag::PV.get_num() + 1, PA, "PA"),
    (0x80, IRREGULAR, "IRREGULAR"),
    (
        POSTag::VV.get_num() | POSTag::IRREGULAR.get_num(),
        VVI,
        "VV-I"
    ),
    (
        POSTag::VA.get_num() | POSTag::IRREGULAR.get_num(),
        VAI,
        "VA-I"
    ),
    (
        POSTag::VX.get_num() | POSTag::IRREGULAR.get_num(),
        VXI,
        "VX-I"
    ),
    (
        POSTag::XSA.get_num() | POSTag::IRREGULAR.get_num(),
        XSAI,
        "XSA-I"
    ),
    (
        POSTag::PV.get_num() | POSTag::IRREGULAR.get_num(),
        PVI,
        "PV-I"
    ),
    (
        POSTag::PA.get_num() | POSTag::IRREGULAR.get_num(),
        PAI,
        "PA-I"
    ),
];
