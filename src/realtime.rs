use crossbeam_channel::{self as channel, Receiver, Sender};
use indexmap::IndexMap;
use log::*;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{
    fs,
    io::{self, BufRead, Write},
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

pub type FxIndexMap<K, V> = IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
type UnknownChannel = (
    Sender<(Language, EnglishPhase)>,
    Receiver<(Language, EnglishPhase)>,
);
type Language = String;
type EnglishPhase = String;
type Translated = Arc<String>;
type ReplaceMap = FxIndexMap<Language, FxIndexMap<EnglishPhase, Translated>>;

static THREAD_STARTED: AtomicBool = AtomicBool::new(false);
static REPLACE: Lazy<RwLock<ReplaceMap>> = Lazy::new(<_>::default);

pub(crate) fn notify_unknown(lang: &str, en: &str) {
    static CHANNEL: Lazy<UnknownChannel> = Lazy::new(channel::unbounded);

    CHANNEL.0.send((lang.into(), en.into())).unwrap();

    if !THREAD_STARTED.swap(true, SeqCst) {
        std::thread::Builder::new()
            .name("google-translator".into())
            .spawn(|| {
                info!("Starting realtime google translator");
                read_persisted().unwrap();

                while let Ok((lang, en)) = CHANNEL.1.recv() {
                    if translated_to(&lang, &en).is_none() {
                        google_translate(&lang, &en).unwrap();
                    }
                }
            })
            .unwrap();
    }
}

pub(crate) fn translated_to(lang: &str, en: &str) -> Option<Translated> {
    REPLACE.read().get(lang).and_then(|l| l.get(en)).cloned()
}

pub fn external_init() {
    if !THREAD_STARTED.swap(true, SeqCst) {
        read_persisted().unwrap();
    }
}

pub fn google_translate(lang: &str, en: &str) -> Result<(), String> {
    if let Some(translated) = google_translate::translate_to(lang, en) {
        info!("Fetched google translation {:?} -> {:?}", en, translated);
        let translated = translated.into();
        REPLACE
            .write()
            .entry(lang.to_owned())
            .or_default()
            .insert(en.into(), translated);
        write_replace().unwrap();
        Ok(())
    } else {
        Err(format!(
            "Google translate ({:?}, {:?}) didn't work",
            lang, en
        ))
    }
}

fn read_persisted() -> io::Result<()> {
    let persisted = fs::read_dir(std::env::current_dir()?)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.file_name().to_str()?.to_owned();
            Some((entry, path))
        })
        .filter(|(_, path)| path.starts_with("en-replace.") && path.ends_with(".google.pairs"));

    let mut replace: ReplaceMap = <_>::default();

    for (entry, path) in persisted {
        let lang = path.split('.').nth(1).unwrap();
        let lang_replace = replace.entry(lang.into()).or_default();

        let file = fs::OpenOptions::new().read(true).open(entry.path())?;
        let mut last_line = String::new();
        for line in io::BufReader::new(file).lines() {
            let line = line.unwrap_or_else(|_| String::new()).replace(r"\n", "\n");
            if !line.trim().is_empty() && !last_line.trim().is_empty() {
                let en = last_line.trim();
                let t = line.trim();
                lang_replace.insert(en.into(), t.to_owned().into());
            }
            last_line = line;
        }
    }

    *REPLACE.write() = replace;

    Ok(())
}

fn write_replace() -> io::Result<()> {
    for (lang, vals) in &*REPLACE.read() {
        let filename = format!("en-replace.{}.google.pairs", lang);
        let file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&filename)?;
        let mut file = io::BufWriter::new(file);

        for (en, translated) in vals {
            write!(
                file,
                "{}\n{}\n\n",
                en.replace('\n', r"\n"),
                translated.replace('\n', r"\n")
            )?;
        }
    }

    Ok(())
}
