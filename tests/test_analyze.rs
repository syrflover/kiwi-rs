use kiwi::{kiwi_version, KiwiBuilder, KiwiOptions, Match};

#[test]
fn test_analyze() -> anyhow::Result<()> {
    println!("v{}", kiwi_version());

    let builder_options = KiwiOptions::default();

    let kiwi = KiwiBuilder::new(2, builder_options)?.build()?;

    let match_options = Match::new()
        .compatible_jamo(true)
        .normalize_coda(true)
        .split_complex(true)
        .z_coda(true)
        .split_saisiot(true);

    let text = "안녕하세요 저는 바보입니다. 제 핏줄 보이시나요? 만둣국 먹고 싶다.";

    let analyzed = kiwi.analyze(
        //
        // U16String::from_str(text),
        text,
        1,
        match_options,
        None,
        None,
    )?;

    println!("{:?}", analyzed.to_vec());

    for (form, token) in analyzed.iter() {
        print!("{} {} / ", form, token.tag);
    }
    println!();

    // out of bounds
    assert!(analyzed.word_num(analyzed.size()).is_none());

    Ok(())
}
