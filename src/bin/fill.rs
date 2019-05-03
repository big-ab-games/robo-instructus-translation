//! Generate missing translation files use `features "fill"`
#![cfg(feature = "fill")]

use lazy_static::lazy_static;
use rustc_hash::{FxHashMap, FxHashSet};
use std::env;

// REPLACE: FxHashMap<&str, FxHashMap<&str, &str>>: { "ru" -> { "yes" -> "да", .. }, .. }
include!("../../target/generated/translations.rs");

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "robo_instructus=debug");
    }
    env_logger::init();

    robo_instructus_translation::realtime::external_init();

    let all_en: FxHashSet<_> = REPLACE.values().flat_map(FxHashMap::keys).collect();

    // run all keys through all known languages to produce missing tranlations
    for (lang, translations) in REPLACE.iter() {
        for en in &all_en {
            if translations.get(*en).is_none() {
                robo_instructus_translation::realtime::google_translate(lang, en).unwrap();
            }
        }
    }
}
