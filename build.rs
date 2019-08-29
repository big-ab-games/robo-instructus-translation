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
    build_primer();
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

    write_generated("translated-pairs.rs", contents);
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

            let inserts: Vec<_> = robomarkup_sections("#!company", file)
                .into_iter()
                .map(|(id, text)| quote! {
                    company.insert(CompanyMessageId::try_from(#id).unwrap(), #text);
                })
                .collect();

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

        impl std::convert::TryFrom<&str> for CompanyMessageId {
            type Error = &'static str;
            fn try_from(key: &str) -> Result<Self, Self::Error> {
                Ok(match key {
                    "c-await" => CompanyMessageId::Await,
                    "c-receiving" => CompanyMessageId::Receiving,
                    "c-ack" => CompanyMessageId::Acknowledge,
                    "c-arrive" => CompanyMessageId::Arrive,
                    "c-underground" => CompanyMessageId::Underground,
                    "c-lower" => CompanyMessageId::Lower,
                    "c-final" => CompanyMessageId::Final,
                    "c-promotion" => CompanyMessageId::Promotion,
                    _ => return Err("Unknown company id"),
                })
            }
        }

        lazy_static! {
            pub static ref COMPANY: FxHashMap<&'static str, FxHashMap<CompanyMessageId, &'static str>> = {
                use std::convert::TryFrom;
                let mut lang_to_company = FxHashMap::default();
                lang_to_company.reserve(#lang_company_len);
                #(#lang_company)*
                lang_to_company
            };
        }
    };

    write_generated("company.rs", contents);
}

fn build_primer() {
    let unlocks = std::env::current_dir().unwrap().join("unlocks");

    let lang_primer: Vec<_> = fs::read_dir(&unlocks)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok().unwrap();
            let path = entry.file_name().to_str().unwrap().to_owned();
            Some((entry, path))
        })
        .filter(|(_, path)| path.starts_with("primer.") && path.ends_with(".robomarkup"))
        .map(|(entry, path)| {
            let lang = path.split('.').nth(1).unwrap();
            let file = fs::OpenOptions::new().read(true).open(entry.path()).unwrap();

            let inserts: Vec<_> = robomarkup_sections("#!unlock", file)
                .into_iter()
                .map(|(id, text)| quote! {
                    primer.insert(PrimerId::try_from(#id).unwrap(), #text);
                })
                .collect();

            let inserts_len = inserts.len();
            quote! {
                lang_to_primer.insert(#lang, {
                    let mut primer = FxHashMap::default();
                    primer.reserve(#inserts_len);
                    #(#inserts)*
                    primer
                });
            }
        })
        .collect();

    let lang_primer_len = lang_primer.len();
    let contents = quote! {
        /// Primer section id.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PrimerId {
            Loops,
            Comments,
            Conditionals,
            Variables,
            Conditionals2,
            Is,
            Comparison,
            Conditionals3,
            ElseIf,
            Scope,
            Loops2,
            Loops3,
            Fun,
            FunB,
            Fun2,
            Bool,
            Seq,
            SeqB,
            LoopSeq,
            Fun3,
            DotCall,
        }

        impl std::convert::TryFrom<&str> for PrimerId {
            type Error = &'static str;
            fn try_from(key: &str) -> Result<Self, Self::Error> {
                Ok(match key {
                    "loops" => PrimerId::Loops,
                    "comments" => PrimerId::Comments,
                    "conditionals" => PrimerId::Conditionals,
                    "variables" => PrimerId::Variables,
                    "conditionals-2" => PrimerId::Conditionals2,
                    "is" => PrimerId::Is,
                    "comparison" => PrimerId::Comparison,
                    "conditionals-3" => PrimerId::Conditionals3,
                    "else-if" => PrimerId::ElseIf,
                    "scope" => PrimerId::Scope,
                    "loops-2" => PrimerId::Loops2,
                    "loops-3" => PrimerId::Loops3,
                    "fun" => PrimerId::Fun,
                    "fun-b" => PrimerId::FunB,
                    "fun-2" => PrimerId::Fun2,
                    "bool" => PrimerId::Bool,
                    "seq" => PrimerId::Seq,
                    "seq-b" => PrimerId::SeqB,
                    "loop-seq" => PrimerId::LoopSeq,
                    "fun-3" => PrimerId::Fun3,
                    "dot-call" => PrimerId::DotCall,
                    _ => return Err("Unknown primer id"),
                })
            }
        }

        lazy_static! {
            pub static ref PRIMER: FxHashMap<&'static str, FxHashMap<PrimerId, &'static str>> = {
                use std::convert::TryFrom;
                let mut lang_to_primer = FxHashMap::default();
                lang_to_primer.reserve(#lang_primer_len);
                #(#lang_primer)*
                lang_to_primer
            };
        }
    };

    write_generated("primer.rs", contents);
}

fn write_generated<T: std::fmt::Display>(file_name: &str, contents: T) {
    let generated = Path::new("target").join("generated");
    fs::create_dir_all(&generated).unwrap();

    let dest_path = generated.join(file_name);
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&dest_path)
        .unwrap();

    write!(file, "{}", contents).unwrap();
}

fn extract_id(line: &str) -> String {
    let id_start = line.find("id{").expect("missing id{...}");
    line[id_start + "id{".len()..].chars().take_while(|c| *c != '}').collect()
}

/// Parses robomarkup files -> `[(id, contents), ...]`.
///
/// Tokens must have `id{...}` declared.
fn robomarkup_sections(start_token: &str, file: fs::File) -> Vec<(String, String)> {
    let mut message: Option<(String, String)> = None;
    let mut out = vec![];

    for line in BufReader::new(file).lines() {
        let line = line.unwrap_or_else(|_| String::new());
        if line.starts_with("//") {
            continue;
        } else if line.starts_with(start_token) {
            if let Some((id, text)) = message.take() {
                out.push((id, text.trim().to_owned()));
            }
            message = Some((extract_id(&line), String::new()));
        } else if let Some((_, text)) = message.as_mut() {
            text.push_str(&line);
            text.push('\n');
        }
    }
    if let Some((id, text)) = message.take() {
        out.push((id, text.trim().to_owned()));
    }

    out
}
