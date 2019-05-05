use quote::quote;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::*,
};

fn main() {
    generate_translations();
}

fn generate_translations() {
    let translations = std::env::current_dir().unwrap();

    let lang_replace: Vec<_> = fs::read_dir(&translations).unwrap()
        .filter_map(|entry| {
            let entry = entry.ok().unwrap();
            let path = entry.file_name().to_str().unwrap().to_owned();
            Some((entry, path))
        })
        .filter(|(_, path)| path.ends_with(".pairs"))
        .map(|(entry, path)| {
            let lang = path.split('.').nth(1).unwrap();
            let file = fs::OpenOptions::new().read(true).open(entry.path()).unwrap();

            let mut inserts = vec![];

            let mut last_line = String::new();
            for line in BufReader::new(file).lines() {
                let line = line.unwrap_or_else(|_| String::new()).replace(r"\n", "\n");
                if line.starts_with('#') {
                    continue;
                }
                if !line.trim().is_empty() && !last_line.trim().is_empty() {
                    let en = last_line.trim();
                    let t = line.trim();
                    inserts.push(quote! {
                        replace.insert(#en, #t);
                    });
                }
                last_line = line;
            }

            let inserts_len = inserts.len();
            quote! {
                store.insert(#lang, {
                    let mut replace = FxHashMap::default();
                    replace.reserve(#inserts_len);
                    #(#inserts)*
                    replace
                });
            }
        })
        .collect();

    let lang_replace_len = lang_replace.len();
    let contents = quote! {
        lazy_static! {
            pub static ref REPLACE: FxHashMap<&'static str, FxHashMap<&'static str, &'static str>> = {
                let mut store = FxHashMap::default();
                store.reserve(#lang_replace_len);
                #(#lang_replace)*
                store
            };
        }
    };

    let generated = Path::new("target").join("generated");
    fs::create_dir_all(&generated).unwrap();

    let dest_path = generated.join("translations.rs");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dest_path).unwrap();

    file.write_all(contents.to_string().as_bytes()).unwrap();
}
