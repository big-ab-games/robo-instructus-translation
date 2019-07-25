#![recursion_limit="128"]

use quote::quote;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::*,
};

fn main() {
    build_translated_pairs();
    build_company();
}

fn build_translated_pairs() {
    let translations = std::env::current_dir().unwrap().join("translated-pairs");

    let lang_replace: Vec<_> = fs::read_dir(&translations)
        .unwrap()
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

    let dest_path = generated.join("translated-pairs.rs");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dest_path)
        .unwrap();

    file.write_all(contents.to_string().as_bytes()).unwrap();
}

fn build_company() {
    let company = std::env::current_dir().unwrap().join("company");

    let lang_company: Vec<_> = fs::read_dir(&company)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok().unwrap();
            let path = entry.file_name().to_str().unwrap().to_owned();
            Some((entry, path))
        })
        .filter(|(_, path)| path.ends_with(".robomarkup"))
        .map(|(entry, path)| {
            let lang = path.split('.').nth(1).unwrap();
            let file = fs::OpenOptions::new().read(true).open(entry.path()).unwrap();

            let mut inserts = vec![];
            let mut message: Option<(String, String)> = None;

            for line in BufReader::new(file).lines() {
                let line = line.unwrap_or_else(|_| String::new());
                if line.starts_with("//") {
                    continue;
                } else if line.starts_with("#!company") {
                    if let Some((id, text)) = message.take() {
                        let text = text.trim();
                        inserts.push(quote! {
                            company.insert(CompanyMessageId::from_str(#id), #text);
                        });
                    }

                    let id_start = line.find("id{").expect("missing id{...}");
                    let id: String =
                        line[id_start + "id{".len()..].chars().take_while(|c| *c != '}').collect();

                    message = Some((id, String::new()));
                } else if let Some((_, text)) = message.as_mut() {
                    text.push_str(&line);
                    text.push('\n');
                }
            }

            if let Some((id, text)) = message.take() {
                let text = text.trim();
                inserts.push(quote! {
                    company.insert(CompanyMessageId::from_str(#id), #text);
                });
            }

            let inserts_len = inserts.len();
            quote! {
                lang_to_company.insert(#lang, {
                    let mut company = FxHashMap::default();
                    company.reserve(#inserts_len);
                    #(#inserts)*
                    company
                });
            }
        })
        .collect();

    let lang_company_len = lang_company.len();
    let contents = quote! {
        /// Company message id.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum CompanyMessageId {
            Await,
            Receiving,
            Acknowledge,
            Arrive,
            Underground,
            Lower,
            Final,
            Promotion,
        }

        impl CompanyMessageId {
            /// Panics if unknown.
            pub fn from_str(key: &str) -> Self {
                match key {
                    "c-await" => CompanyMessageId::Await,
                    "c-receiving" => CompanyMessageId::Receiving,
                    "c-ack" => CompanyMessageId::Acknowledge,
                    "c-arrive" => CompanyMessageId::Arrive,
                    "c-underground" => CompanyMessageId::Underground,
                    "c-lower" => CompanyMessageId::Lower,
                    "c-final" => CompanyMessageId::Final,
                    "c-promotion" => CompanyMessageId::Promotion,
                    key => panic!("Unknown company key: {}", key),
                }
            }
        }

        lazy_static! {
            pub static ref COMPANY: FxHashMap<&'static str, FxHashMap<CompanyMessageId, &'static str>> = {
                let mut lang_to_company = FxHashMap::default();
                lang_to_company.reserve(#lang_company_len);
                #(#lang_company)*
                lang_to_company
            };
        }
    };

    let generated = Path::new("target").join("generated");
    fs::create_dir_all(&generated).unwrap();

    let dest_path = generated.join("company.rs");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dest_path)
        .unwrap();

    file.write_all(contents.to_string().as_bytes()).unwrap();
}
