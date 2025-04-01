use std::time::Instant;

use rkiwi::{kiwi_version, KiwiBuilder, KiwiOptions, Match, POSTag};
use tracing::Level;

fn tracing() {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
}

#[cfg(feature = "impl_send")]
#[test]
fn send_trait() {
    fn assert_send<T: Send>() {}

    assert_send::<rkiwi::Analyzed>();
    assert_send::<rkiwi::Extracted>();
    assert_send::<rkiwi::DefaultTypoTransformer>();
    assert_send::<rkiwi::TypoTransformer>();
    assert_send::<rkiwi::KiwiBuilder>();
    assert_send::<rkiwi::Kiwi>();
    assert_send::<rkiwi::MorphemeSet>();
    assert_send::<rkiwi::Pretokenized>();
}

#[tokio::test]
async fn test_analyze() -> anyhow::Result<()> {
    use rkiwi::{DefaultTypoSet, TypoTransformer};

    tracing();

    println!("v{}", kiwi_version());

    let builder_options = KiwiOptions::default();

    let basic_typo =
        TypoTransformer::default(DefaultTypoSet::BasicTypoSetWithContinualAndLengthening)?;

    let typo = TypoTransformer::new()?;

    typo.update(&basic_typo)?;

    let a = Instant::now();

    let kiwi_builder = KiwiBuilder::new(1, builder_options)?
        .add_word("벨리타", POSTag::NNP, 0.0)?
        .add_word("에르핀", POSTag::NNP, 0.0)?
        .add_word("에르피엔", POSTag::NNP, 0.0)?;
    // .typo(basic_typo, None)

    let kiwi = kiwi_builder.build(&typo, None)?;

    println!("initialize: {}ms", a.elapsed().as_millis());

    drop(kiwi_builder);
    drop(typo);

    let match_options = Match::new()
        .compatible_jamo(true)
        .normalize_coda(true)
        .split_complex(true)
        .z_coda(true)
        .split_saisiot(true);

    let text = "안녕하세요 저는 바보입니다. 제 핏줄 보이시나요? 만둣국 먹고 싶다. 홍개먹고싶다.";

    let analyzed = kiwi.analyze(
        //
        // U16String::from_str(text),
        text,
        1,
        match_options,
        None,
        None,
    )?;

    // let analyzed = tokio::task::spawn_blocking({
    //     let kiwi = kiwi.clone();
    //     move || {
    //         kiwi.analyze(
    //             //
    //             // U16String::from_str(text),
    //             text,
    //             1,
    //             match_options,
    //             None,
    //             None,
    //         )
    //     }
    // })
    // .await??;

    println!("{:?}", analyzed.to_vec());

    for (form, token) in analyzed.iter() {
        print!("{} {} / ", form, token.tag);
    }
    println!();

    // out of bounds
    assert!(analyzed.word_num(analyzed.size()).is_none());

    let text = "에르핀과 벨리타는 에르피엔에 위치한 세계수 교단에서 점심을 먹었습니다.";

    let analyzed = kiwi.analyze(text, 1, match_options, None, None)?;

    for (form, token) in analyzed.iter() {
        print!("{} {} / ", form, token.tag);
    }
    println!();

    Ok(())
}
