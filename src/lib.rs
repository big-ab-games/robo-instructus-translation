#[cfg(feature = "realtime")]
pub mod realtime;

// mod translated_pairs { MAP: FxHashMap<&str, FxHashMap<&str, &str>> }
include!(concat!(env!("OUT_DIR"), "/translated-pairs.rs"));

// mod company { MAP: FxHashMap<&str, FxHashMap<CompanyMessageId, &str>>, ... }
include!(concat!(env!("OUT_DIR"), "/company.rs"));

// mod primer { MAP: FxHashMap<&str, FxHashMap<PrimerId, &str>>, ... }
include!(concat!(env!("OUT_DIR"), "/primer.rs"));

// mod function_docs { MAP: FxHashMap<&str, FxHashMap<FunctionDocId, &str>>, ... }
include!(concat!(env!("OUT_DIR"), "/function_docs.rs"));

// mod colony { MAP: FxHashMap<&str, FxHashMap<FunctionDocId, &str>>, ... }
include!(concat!(env!("OUT_DIR"), "/colony.rs"));

pub use colony::ColonyMessageId;
pub use company::CompanyMessageId;
pub use function_docs::FunctionDocId;
pub use primer::PrimerId;

use once_cell::sync::Lazy;
use parking_lot::RwLock;

/// Global language, improves translation ergonomics at great cost
static LANG: Lazy<RwLock<String>> = Lazy::new(<_>::default);

/// Translations section of Robo Instructus credits
pub const CREDITS: &str = include_str!("../credits.txt");

/// Sets the global target translation language
pub fn set_language_target(lang: &str) {
    *LANG.write() = lang.to_owned();
}

#[inline]
pub fn language_target<T, F: FnOnce(&str) -> T>(fun: F) -> T {
    let lang = &*LANG.read();
    if lang.trim().is_empty() {
        fun("en")
    } else {
        fun(lang)
    }
}

/// Translates english text into the global target language
#[inline]
pub fn translate(en: &str) -> &str {
    translate_to(&LANG.read(), en)
}

/// Translates english text into the input target language
pub fn translate_to<'a>(lang: &str, en: &'a str) -> &'a str {
    if lang == "en" || en.trim().is_empty() || lang.trim().is_empty() {
        en
    } else if let Some(translated) = translated_pairs::MAP.get(lang).and_then(|l| l.get(en)) {
        translated
    } else {
        #[cfg(feature = "realtime")]
        realtime::notify_unknown(lang, en);

        #[cfg(not(feature = "realtime"))]
        log::debug!("Missing translation for {:?} in lang {:?}", en, lang);

        en
    }
}

/// Fetches global target language company message with the matching `key`, or falls back on
/// lang='en'.
#[inline]
pub fn company(c: CompanyMessageId) -> &'static str {
    company_lang(LANG.read().as_str(), c)
}

/// Fetches `lang` company message with the matching `key`, or falls back on lang='en'.
pub fn company_lang(lang: &str, c: CompanyMessageId) -> &'static str {
    company::MAP
        .get(lang)
        .and_then(|company| company.get(&c).copied())
        .unwrap_or_else(|| company::MAP["en"][&c])
}

/// Fetches global target language primer section with the matching `key`, or falls back on
/// lang='en'.
#[inline]
pub fn primer(c: PrimerId) -> &'static str {
    primer_lang(LANG.read().as_str(), c)
}

/// Fetches `lang` primer section with the matching `key`, or falls back on lang='en'.
pub fn primer_lang(lang: &str, c: PrimerId) -> &'static str {
    primer::MAP
        .get(lang)
        .and_then(|p| p.get(&c).copied())
        .unwrap_or_else(|| primer::MAP["en"][&c])
}

/// Fetches global target language function doc with the matching `key`,
/// or falls back on lang='en'.
#[inline]
pub fn function_docs(id: FunctionDocId) -> &'static str {
    function_docs_lang(LANG.read().as_str(), id)
}

/// Fetches `lang` function doc with the matching `key`, or falls back on lang='en'.
pub fn function_docs_lang(lang: &str, id: FunctionDocId) -> &'static str {
    function_docs::MAP
        .get(lang)
        .and_then(|p| p.get(&id).copied())
        .unwrap_or_else(|| function_docs::MAP["en"][&id])
}

/// Fetches global target language function doc with the matching `key`,
/// or falls back on lang='en'.
#[inline]
pub fn colony(id: ColonyMessageId) -> &'static str {
    colony_lang(LANG.read().as_str(), id)
}

/// Fetches `lang` function doc with the matching `key`, or falls back on lang='en'.
pub fn colony_lang(lang: &str, id: ColonyMessageId) -> &'static str {
    colony::MAP
        .get(lang)
        .and_then(|p| p.get(&id).copied())
        .unwrap_or_else(|| colony::MAP["en"][&id])
}

#[test]
fn translate_ru() {
    assert_eq!(translate_to("ru", "Begin"), "Начать");
}

#[test]
fn translate_pl() {
    assert_eq!(translate_to("pl", "Begin"), "Rozpocznij");
}

#[test]
fn company_en() {
    assert_eq!(
        company_lang("en", CompanyMessageId::Acknowledge),
        "Acknowledge"
    );

    // en should include every CompanyMessageId
    assert!(!company_lang("en", CompanyMessageId::Receiving).is_empty());
    assert!(!company_lang("en", CompanyMessageId::Await).is_empty());
    assert!(!company_lang("en", CompanyMessageId::Arrive).is_empty());
    assert!(!company_lang("en", CompanyMessageId::Underground).is_empty());
    assert!(!company_lang("en", CompanyMessageId::Lower).is_empty());
    assert!(!company_lang("en", CompanyMessageId::Final).is_empty());
    assert!(!company_lang("en", CompanyMessageId::Promotion).is_empty());
}

#[test]
fn company_fallback() {
    assert_eq!(
        company_lang("nosuch", CompanyMessageId::Receiving),
        "Receiving Communication"
    );
}

#[test]
fn company_es() {
    assert_eq!(
        company_lang("es", CompanyMessageId::Acknowledge),
        "Recibido"
    );
}

#[test]
fn company_zh_cn() {
    assert_eq!(company_lang("zh-CN", CompanyMessageId::Acknowledge), "认可");
}

#[test]
fn company_await_is_3_lines() {
    for (lang, c) in &*company::MAP {
        if let Some(text) = c.get(&CompanyMessageId::Await) {
            let new_lines = text.chars().filter(|c| *c == '\n').count();
            assert_eq!(
                new_lines, 2,
                "`{}` has invalid CompanyMessageId::Await",
                lang
            );
        }
    }
}

#[test]
#[rustfmt::skip]
fn primer_en() {
    const EXPECTED_COMMENTS_PRIMER: &str = "# Comments\n\
                                            \n\
                                            Text after a '`#`' symbol will not be used as code, \
                                            these are just comments used to make notes.\n\
                                            \n\
                                            ```no_run\n\
                                            robo_left()  # I think I'm starting to get this\n\
                                            ```";

    assert_eq!(primer_lang("en", PrimerId::Comments), EXPECTED_COMMENTS_PRIMER);

    // en should include all
    use PrimerId::*;
    for id in [
        Loops, Comments, Conditionals, Variables, Conditionals2, Is, Comparison, Conditionals3,
        ElseIf, Scope, Loops2, Loops3, Fun, FunB, Fun2, Bool, Seq, SeqB, LoopSeq, Fun3, DotCall
    ].iter() {
        assert!(!primer_lang("en", *id).trim().is_empty(), "{:?} empty", id);
    }
}

#[test]
fn primer_fallback() {
    assert!(primer_lang("nosuch", PrimerId::Comments).starts_with("# Comments"));
}

#[test]
fn primer_es() {
    let text = primer_lang("es", PrimerId::Comments);
    assert!(text.starts_with("# Comentarios"), "`{}`", text);
}

#[test]
fn primer_zh_cn() {
    let text = primer_lang("zh-CN", PrimerId::Comments);
    assert!(text.starts_with("# 注释"), "`{}`", text);
}

#[test]
#[rustfmt::skip]
fn function_docs_en() {
    const EXPECTED: &str = "`robo_use()` Operates on the current tile returning tile specific \
                            data, or `0` otherwise. Runtime $tu{robo_use()} ms\n\
                            $render{robo_use}";
    assert_eq!(function_docs_lang("en", FunctionDocId::Use), EXPECTED);

    // en should include all
    use FunctionDocId::*;
    for id in [
        LeftForward, Scan, ScanU1, ScanU2, ScanU3, ScanU4, Use, UseU1, UseU2, UseU3, UseU4,
        ForwardLocation, Location, DetectAdjacent, Detect3, Detect3L, Probo, ProboScanU1, Transmit,
        ProboUse, ShortLeft, ShortForward, ShortScan, ShortScanU1, ShortScanU2, ShortScanU3,
        ShortScanU4, ShortUse, ShortUseU2, ShortUseU3, ShortUseU4, ShortDetectAdjacent,
        ShortLocation, ShortForwardLocation, ShortDetect3, ShortDetect3L, ShortProboLeft,
        ShortProboForward, ShortProboScan, ShortProboLocation, ShortProboUse, ShortTransmit,
        ShortReceive,
    ].iter() {
        assert!(!function_docs_lang("en", *id).trim().is_empty(), "{:?} empty", id);
    }
}

#[test]
#[rustfmt::skip]
fn colony_en() {
    const EXPECTED_TEXT: &str = "Jacob, what can we do about the doc? Things are falling apart. \
                                 Why doesn't she respond? We've all lost people but it's been \
                                 17 cycles since Jack passed. She's been locked away working on \
                                 some project (we ASSUME) but we really need her out here.\n\n\
                                 Hackett";

    assert_eq!(colony_lang("en", ColonyMessageId::FallingApart), EXPECTED_TEXT);

    // en should include all
    use ColonyMessageId::*;
    for id in [
        PrimerIntro, Pathfinders, ScansUnreliable, Power, Pause, DataStore, IHaveAPlan, PauseFun,
        MushroomsAgain, Direction, Direction2, Orientation, HotAir, FatalityReport, Launch,
        FallingApart, Distress, ProtoProbe, Vault,
    ].iter() {
        assert!(!colony_lang("en", *id).trim().is_empty(), "{:?} empty", id);
    }
}
