use crate::POSTag;

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
