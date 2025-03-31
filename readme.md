# kiwi-rs

한글 형태소 분석기 [Kiwi](https://github.com/bab2min/Kiwi)의 C API를 바인딩한 Rust 라이브러리입니다.

작동이 확인되지 않았으니 사용하지 마세요.

- [x] Kiwi (kiwi)
- [x] KiwiBuilder (kiwi_builder)
- [x] Analyzed (kiwi_res)
- [x] Extracted (kiwi_ws)
- [x] MorphemeSet (kiwi_morphset)
- [x] Pretokenized (kiwi_pt, kiwi_pretokenized)
- [ ] Typo (kiwi_typo)
- [ ] kiwi_ss
- [ ] kiwi_joiner
- [ ] kiwi_swt

## Install with dynamic linking

1. [Kiwi Release 페이지](https://github.com/bab2min/Kiwi/releases)에서 자신의 운영체제와 아키텍처에 맞는 압축 파일을 다운로드 받고, 압축을 해제합니다.
2. 동적 라이브러리 파일들을 잘 알려진 동적 라이브러리 설치 폴더에 복사합니다.
3. crate를 추가합니다.

```toml
[dependencies]
rkiwi = { git = "https://github.com/syrflover/kiwi-rs", branch = "master" }
```

## Install with static linking

`static` feature를 활성화하면 정적 링크할 수 있습니다.

```toml
[dependencies]
rkiwi = { git = "https://github.com/syrflover/kiwi-rs", branch = "master", features = ["static"] }
```

또는

```
KIWI_STATIC_LIB_PATH=/usr/local/lib
```

위와 같이 환경 변수를 설정하고,
`static_prebuilt` feature를 활성화하여 미리 빌드된 정적 라이브러리 파일을 `KIWI_STATIC_LIB_PATH` 폴더에 복사하여 정적 링크할 수 있습니다.

```toml
[dependencies]
rkiwi = { git = "https://github.com/syrflover/kiwi-rs", branch = "master", features = ["static_prebuilt"] }
```
