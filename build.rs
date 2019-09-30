#![recursion_limit = "128"]

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
    build_function_docs();
    build_colony();
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
        mod translated_pairs {
            use once_cell::sync::Lazy;
            use rustc_hash::FxHashMap;

            pub static MAP: Lazy<FxHashMap<&'static str, FxHashMap<&'static str, &'static str>>> =
                Lazy::new(|| {
                    let mut store = FxHashMap::default();
                    store.reserve(#lang_replace_len);
                    #(#lang_replace)*
                    store
            });
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
                .map(|(id, text)| {
                    quote! {
                        company.insert(CompanyMessageId::try_from(#id).unwrap(), #text);
                    }
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
        mod company {
            use once_cell::sync::Lazy;
            use rustc_hash::FxHashMap;
            use std::convert::TryFrom;

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

            impl TryFrom<&str> for CompanyMessageId {
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

            pub static MAP: Lazy<FxHashMap<&'static str, FxHashMap<CompanyMessageId, &'static str>>> =
                Lazy::new(|| {
                    let mut lang_to_company = FxHashMap::default();
                    lang_to_company.reserve(#lang_company_len);
                    #(#lang_company)*
                    lang_to_company
                });
        }
    };

    write_generated("company.rs", contents);
}

fn build_primer() {
    let unlocks = std::env::current_dir().unwrap().join("primer");

    let lang_primer: Vec<_> = fs::read_dir(&unlocks)
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

            let inserts: Vec<_> = robomarkup_sections("#!unlock", file)
                .into_iter()
                .map(|(id, text)| {
                    quote! { primer.insert(PrimerId::try_from(#id).unwrap(), #text); }
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
        mod primer {
            use once_cell::sync::Lazy;
            use rustc_hash::FxHashMap;
            use std::convert::TryFrom;

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

            impl TryFrom<&str> for PrimerId {
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

            pub static MAP: Lazy<FxHashMap<&'static str, FxHashMap<PrimerId, &'static str>>> =
                Lazy::new(|| {
                    let mut lang_to_primer = FxHashMap::default();
                    lang_to_primer.reserve(#lang_primer_len);
                    #(#lang_primer)*
                    lang_to_primer
                });
        }
    };

    write_generated("primer.rs", contents);
}

fn build_function_docs() {
    let unlocks = std::env::current_dir().unwrap().join("function");

    let lang_lookup: Vec<_> = fs::read_dir(&unlocks)
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

            let inserts: Vec<_> = robomarkup_sections("#!unlock", file)
                .into_iter()
                .map(|(id, text)| {
                    quote! {
                        lookup.insert(FunctionDocId::try_from(#id).unwrap(), #text);
                    }
                })
                .collect();

            let inserts_len = inserts.len();
            quote! {
                lang_to_lookup.insert(#lang, {
                    let mut lookup = FxHashMap::default();
                    lookup.reserve(#inserts_len);
                    #(#inserts)*
                    lookup
                });
            }
        })
        .collect();

    let lang_lookup_len = lang_lookup.len();
    let contents = quote! {
        mod function_docs {
            use once_cell::sync::Lazy;
            use rustc_hash::FxHashMap;
            use std::convert::TryFrom;

            /// Function documentation id.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum FunctionDocId {
                LeftForward,
                Scan,
                ScanU1,
                ScanU2,
                ScanU3,
                ScanU4,
                Use,
                UseU1,
                UseU2,
                UseU3,
                UseU4,
                ForwardLocation,
                Location,
                DetectAdjacent,
                Detect3,
                Detect3L,
                Probo,
                ProboScanU1,
                Transmit,
                ProboUse,
                ShortLeft,
                ShortForward,
                ShortScan,
                ShortScanU1,
                ShortScanU2,
                ShortScanU3,
                ShortScanU4,
                ShortUse,
                ShortUseU2,
                ShortUseU3,
                ShortUseU4,
                ShortDetectAdjacent,
                ShortLocation,
                ShortForwardLocation,
                ShortDetect3,
                ShortDetect3L,
                ShortProboLeft,
                ShortProboForward,
                ShortProboScan,
                ShortProboLocation,
                ShortProboUse,
                ShortTransmit,
                ShortReceive,
            }

            impl TryFrom<&str> for FunctionDocId {
                type Error = String;
                fn try_from(key: &str) -> Result<Self, Self::Error> {
                    Ok(match key {
                        "left-forward" => FunctionDocId::LeftForward,
                        "scan" => FunctionDocId::Scan,
                        "scan-u1" => FunctionDocId::ScanU1,
                        "scan-u2" => FunctionDocId::ScanU2,
                        "scan-u3" => FunctionDocId::ScanU3,
                        "scan-u4" => FunctionDocId::ScanU4,
                        "use" => FunctionDocId::Use,
                        "use-u1" => FunctionDocId::UseU1,
                        "use-u2" => FunctionDocId::UseU2,
                        "use-u3" => FunctionDocId::UseU3,
                        "use-u4" => FunctionDocId::UseU4,
                        "forward-location" => FunctionDocId::ForwardLocation,
                        "location" => FunctionDocId::Location,
                        "detect-adjacent" => FunctionDocId::DetectAdjacent,
                        "detect-3" => FunctionDocId::Detect3,
                        "detect-3l" => FunctionDocId::Detect3L,
                        "probo" => FunctionDocId::Probo,
                        "probo-scan-u1" => FunctionDocId::ProboScanU1,
                        "transmit" => FunctionDocId::Transmit,
                        "probo-use" => FunctionDocId::ProboUse,
                        "s-left" => FunctionDocId::ShortLeft,
                        "s-forward" => FunctionDocId::ShortForward,
                        "s-scan" => FunctionDocId::ShortScan,
                        "s-scan-u1" => FunctionDocId::ShortScanU1,
                        "s-scan-u2" => FunctionDocId::ShortScanU2,
                        "s-scan-u3" => FunctionDocId::ShortScanU3,
                        "s-scan-u4" => FunctionDocId::ShortScanU4,
                        "s-use" => FunctionDocId::ShortUse,
                        "s-use-u2" => FunctionDocId::ShortUseU2,
                        "s-use-u3" => FunctionDocId::ShortUseU3,
                        "s-use-u4" => FunctionDocId::ShortUseU4,
                        "s-detect-adjacent" => FunctionDocId::ShortDetectAdjacent,
                        "s-location" => FunctionDocId::ShortLocation,
                        "s-forward-location" => FunctionDocId::ShortForwardLocation,
                        "s-detect-3" => FunctionDocId::ShortDetect3,
                        "s-detect-3l" => FunctionDocId::ShortDetect3L,
                        "s-probo-left" => FunctionDocId::ShortProboLeft,
                        "s-probo-forward" => FunctionDocId::ShortProboForward,
                        "s-probo-scan" => FunctionDocId::ShortProboScan,
                        "s-probo-location" => FunctionDocId::ShortProboLocation,
                        "s-probo-use" => FunctionDocId::ShortProboUse,
                        "s-transmit" => FunctionDocId::ShortTransmit,
                        "s-receive" => FunctionDocId::ShortReceive,
                        _ => return Err(format!("Unknown function doc id: `{}`", key)),
                    })
                }
            }

            pub static MAP: Lazy<FxHashMap<&'static str, FxHashMap<FunctionDocId, &'static str>>> =
                Lazy::new(|| {
                    let mut lang_to_lookup = FxHashMap::default();
                    lang_to_lookup.reserve(#lang_lookup_len);
                    #(#lang_lookup)*
                    lang_to_lookup
                });
        }
    };

    write_generated("function_docs.rs", contents);
}

fn build_colony() {
    let dir = std::env::current_dir().unwrap().join("colony");

    let lang_message: Vec<_> = fs::read_dir(&dir)
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

            let inserts: Vec<_> = robomarkup_sections("#!unlock", file)
                .into_iter()
                .map(|(id, text)| {
                    quote! {
                        colony.insert(ColonyMessageId::try_from(#id).unwrap(), #text);
                    }
                })
                .collect();

            let inserts_len = inserts.len();
            quote! {
                map.insert(#lang, {
                    let mut colony = FxHashMap::default();
                    colony.reserve(#inserts_len);
                    #(#inserts)*
                    colony
                });
            }
        })
        .collect();

    let lang_message_len = lang_message.len();
    let contents = quote! {
        mod colony {
            use once_cell::sync::Lazy;
            use rustc_hash::FxHashMap;
            use std::convert::TryFrom;

            /// Colony message id.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum ColonyMessageId {
                PrimerIntro,
                Pathfinders,
                ScansUnreliable,
                Power,
                Pause,
                DataStore,
                IHaveAPlan,
                PauseFun,
                MushroomsAgain,
                Direction,
                Direction2,
                Orientation,
                HotAir,
                FatalityReport,
                Launch,
                FallingApart,
                Distress,
                ProtoProbe,
                Vault,
            }

            impl TryFrom<&str> for ColonyMessageId {
                type Error = String;
                fn try_from(key: &str) -> Result<Self, Self::Error> {
                    Ok(match key {
                        "primer-intro" => ColonyMessageId::PrimerIntro,
                        "pathfinders" => ColonyMessageId::Pathfinders,
                        "scans-unreliable" => ColonyMessageId::ScansUnreliable,
                        "power" => ColonyMessageId::Power,
                        "pause" => ColonyMessageId::Pause,
                        "data-store" => ColonyMessageId::DataStore,
                        "i-have-a-plan" => ColonyMessageId::IHaveAPlan,
                        "pause-fun" => ColonyMessageId::PauseFun,
                        "mushrooms-again" => ColonyMessageId::MushroomsAgain,
                        "direction" => ColonyMessageId::Direction,
                        "direction-2" => ColonyMessageId::Direction2,
                        "orientation" => ColonyMessageId::Orientation,
                        "hot-air" => ColonyMessageId::HotAir,
                        "fatality-report" => ColonyMessageId::FatalityReport,
                        "launch" => ColonyMessageId::Launch,
                        "falling-apart" => ColonyMessageId::FallingApart,
                        "distress" => ColonyMessageId::Distress,
                        "proto-probe" => ColonyMessageId::ProtoProbe,
                        "vault" => ColonyMessageId::Vault,
                        _ => return Err(format!("Unknown colony id: {}", key)),
                    })
                }
            }

            pub static MAP: Lazy<FxHashMap<&'static str, FxHashMap<ColonyMessageId, &'static str>>> =
                Lazy::new(|| {
                    let mut map = FxHashMap::default();
                    map.reserve(#lang_message_len);
                    #(#lang_message)*
                    map
                });
        }
    };

    write_generated("colony.rs", contents);
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
