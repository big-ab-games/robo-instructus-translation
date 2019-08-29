#[cfg(feature = "realtime")]
pub mod realtime;

use lazy_static::lazy_static;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;

// REPLACE: FxHashMap<&str, FxHashMap<&str, &str>>: { "ru" -> { "yes" -> "да", .. }, .. }
include!("../target/generated/translated-pairs.rs");

// pub enum CompanyMessageId { ... }
//
// COMPANY: FxHashMap<&str, FxHashMap<CompanyMessageId, &str>>: {
//     "en" -> { CompanyMessageId::Await -> "Acknowledge", .. },
//     .. }
include!("../target/generated/company.rs");

// pub enum PrimerId { ... }
//
// PRIMER: FxHashMap<&str, FxHashMap<PrimerId, &str>>: {
//     "en" -> { PrimerId::_ -> "...", .. },
//     .. }
include!("../target/generated/primer.rs");

lazy_static! {
    static ref LANG: RwLock<String> = RwLock::new(String::new());
}

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
    translate_to(&*LANG.read(), en)
}

/// Translates english text into the input target language
pub fn translate_to<'a>(lang: &str, en: &'a str) -> &'a str {
    if lang == "en" || en.trim().is_empty() || lang.trim().is_empty() {
        en
    } else if let Some(translated) = REPLACE.get(lang).and_then(|l| l.get(en)) {
        translated
    } else {
        #[cfg(feature = "realtime")]
        realtime::notify_unknown(lang, en);

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
fn company_lang(lang: &str, c: CompanyMessageId) -> &'static str {
    COMPANY
        .get(lang)
        .and_then(|company| company.get(&c).copied())
        .unwrap_or_else(|| COMPANY["en"][&c])
}

/// Fetches global target language company message with the matching `key`, or falls back on
/// lang='en'.
#[inline]
pub fn primer(c: PrimerId) -> &'static str {
    primer_lang(LANG.read().as_str(), c)
}

/// Fetches `lang` primer section with the matching `key`, or falls back on lang='en'.
fn primer_lang(lang: &str, c: PrimerId) -> &'static str {
    PRIMER.get(lang).and_then(|p| p.get(&c).copied()).unwrap_or_else(|| PRIMER["en"][&c])
}

#[test]
fn translate_ru() {
    assert_eq!(&*translate_to("ru", "Begin"), "Начать");
}

#[test]
fn translate_pl() {
    assert_eq!(&*translate_to("pl", "Begin"), "Rozpocznij");
}

#[test]
fn company_en() {
    assert_eq!(company_lang("en", CompanyMessageId::Acknowledge), "Acknowledge");

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
    assert_eq!(company_lang("nosuch", CompanyMessageId::Receiving), "Receiving Communication");
}

#[test]
fn company_await_is_3_lines() {
    for (lang, c) in &*COMPANY {
        if let Some(text) = c.get(&CompanyMessageId::Await) {
            let new_lines = text.chars().filter(|c| *c == '\n').count();
            assert_eq!(new_lines, 2, "`{}` has invalid CompanyMessageId::Await", lang);
        }
    }
}

#[test]
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

    // en should include all primer sections
    assert!(!primer_lang("en", PrimerId::Loops).is_empty());
    assert!(!primer_lang("en", PrimerId::Comments).is_empty());
    assert!(!primer_lang("en", PrimerId::Conditionals).is_empty());
    assert!(!primer_lang("en", PrimerId::Variables).is_empty());
    assert!(!primer_lang("en", PrimerId::Conditionals2).is_empty());
    assert!(!primer_lang("en", PrimerId::Is).is_empty());
    assert!(!primer_lang("en", PrimerId::Comparison).is_empty());
    assert!(!primer_lang("en", PrimerId::Conditionals3).is_empty());
    assert!(!primer_lang("en", PrimerId::ElseIf).is_empty());
    assert!(!primer_lang("en", PrimerId::Scope).is_empty());
    assert!(!primer_lang("en", PrimerId::Loops2).is_empty());
    assert!(!primer_lang("en", PrimerId::Loops3).is_empty());
    assert!(!primer_lang("en", PrimerId::Fun).is_empty());
    assert!(!primer_lang("en", PrimerId::FunB).is_empty());
    assert!(!primer_lang("en", PrimerId::Fun2).is_empty());
    assert!(!primer_lang("en", PrimerId::Bool).is_empty());
    assert!(!primer_lang("en", PrimerId::Seq).is_empty());
    assert!(!primer_lang("en", PrimerId::SeqB).is_empty());
    assert!(!primer_lang("en", PrimerId::LoopSeq).is_empty());
    assert!(!primer_lang("en", PrimerId::Fun3).is_empty());
    assert!(!primer_lang("en", PrimerId::DotCall).is_empty());
}

#[test]
fn primer_fallback() {
    assert!(primer_lang("nosuch", PrimerId::Comments).starts_with("# Comments"));
}
