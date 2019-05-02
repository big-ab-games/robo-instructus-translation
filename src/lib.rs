#[cfg(debug_assertions)]
pub mod realtime;

use lazy_static::lazy_static;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;

// REPLACE: FxHashMap<&str, FxHashMap<&str, &str>>: { "ru" -> { "yes" -> "да", .. }, .. }
include!("../target/generated/translations.rs");

lazy_static! {
    static ref LANG: RwLock<String> = RwLock::new(String::new());
}

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
#[inline]
pub fn translate_to<'a>(lang: &str, en: &'a str) -> &'a str {
    if lang == "en" || en.trim().is_empty() || lang.trim().is_empty() {
        en
    } else if let Some(translated) = REPLACE.get(lang).and_then(|l| l.get(en)) {
        translated
    } else {
        #[cfg(debug_assertions)]
        realtime::notify_unknown(lang, en);

        en
    }
}

#[test]
fn translate_to_ru() {
    assert_eq!(&*translate_to("ru", "Exit Level"), "Выход");
}
