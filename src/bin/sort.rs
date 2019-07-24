//! Sort .pairs files in the same way as one in particular
//!
//! cargo run --bin sort --features sort -- ru
#![cfg(feature = "sort")]

use lazy_static::lazy_static;
use log::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    env, fs,
    io::{self, BufRead, Write},
};

// REPLACE: FxHashMap<&str, FxHashMap<&str, &str>>: { "ru" -> { "yes" -> "да", .. }, .. }
include!("../../target/generated/translations.rs");

fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let master: String = env::args().nth(1).unwrap();

    {
        if !REPLACE.contains_key(master.as_str()) {
            eprintln!("Unknown language {}", master);
            std::process::exit(1);
        }
        let translation_counts = REPLACE.values().map(|t| t.len()).collect::<FxHashSet<_>>();
        if translation_counts.len() != 1 {
            eprintln!(
                "All translation must have equal coverage/size: {:?}",
                REPLACE.iter().map(|(l, t)| (l, t.len())).collect::<FxHashMap<_, _>>()
            );
            std::process::exit(2);
        }
    }

    let ordered_ens = {
        let filename = format!("./translated-pairs/en-replace.{}.pairs", master);
        let file = fs::OpenOptions::new().read(true).open(filename).unwrap();

        let mut ens = vec![];

        let mut last_line = String::new();
        for line in io::BufReader::new(file).lines() {
            let line = line.unwrap_or_else(|_| String::new()).replace(r"\n", "\n");;
            if line.starts_with('#') {
                ens.push(Line::Comment(line));
                continue;
            }
            if !line.trim().is_empty() && !last_line.trim().is_empty() {
                ens.push(Line::English(last_line.trim().to_owned()));
            }
            last_line = line;
        }
        ens
    };

    // rewrite all .pairs files except master
    for (lang, vals) in REPLACE.iter().filter(|(lang, _)| *lang != &master) {
        let filename = format!("./translated-pairs/en-replace.{}.pairs", lang);
        let file =
            fs::OpenOptions::new().write(true).truncate(true).create(true).open(&filename).unwrap();
        let mut file = io::BufWriter::new(file);

        info!("sorting {}", filename);
        for en in &ordered_ens {
            match en {
                Line::Comment(s) => writeln!(file, "{}", s).unwrap(),
                Line::English(en) => {
                    let translated = &vals[en.as_str()];
                    write!(
                        file,
                        "{}\n{}\n\n",
                        en.replace('\n', r"\n"),
                        translated.replace('\n', r"\n")
                    )
                    .unwrap();
                }
            }
        }
    }
}

enum Line {
    Comment(String),
    English(String),
}
