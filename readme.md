# kiwi-rs

한글 형태소 분석기 [Kiwi](https://github.com/bab2min/Kiwi)의 C API를 바인딩한 Rust 라이브러리입니다.

작동이 확인되지 않았으니 사용하지 마세요.

- [x] Kiwi (kiwi)
- [x] KiwiBuilder (kiwi_builder)
- [x] Analyzed (kiwi_res)
- [x] Extracted (kiwi_ws)
- [x] MorphemeSet (kiwi_morphset)
- [ ] Typo (kiwi_typo)
- [ ] Pretokenized (kiwi_pt, kiwi_pretokenized)
- [ ] kiwi_ss
- [ ] kiwi_joiner
- [ ] kiwi_swt

## Install with dynamic linking

1. [Kiwi Release 페이지](https://github.com/bab2min/Kiwi/releases)에서 `.tgz` 파일을 다운로드 받고, 압축을 해제합니다.
2. `*.dylib` 파일들을 `/usr/local/lib/` 폴더에 복사합니다.
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

또는 `static_prebuilt` feature를 활성화하면 미리 빌드된 `libkiwi_static.a` 파일을 `/usr/local/lib` 폴더에 복사하여 정적 링크할 수 있습니다.

```toml
[dependencies]
rkiwi = { git = "https://github.com/syrflover/kiwi-rs", branch = "master", features = ["static_prebuilt"] }
```
