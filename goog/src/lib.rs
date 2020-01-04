use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde_json::Value;

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'#');

pub fn translate_to(lang: &str, en: &str) -> Option<String> {
    // handle newlines
    let text = en.replace('\n', "|");

    let client = reqwest::blocking::Client::new();
    // let url = format!(
    //     "https://translate.googleapis.com/translate_a/single?client=gtx&sl=en&tl={lang}&dt=t&q={text}",
    //     lang = lang,
    //     text = utf8_percent_encode(&text, QUERY_ENCODE_SET));
    let api_key =
        std::env::var("GOOGLE_TRANSLATE_API_KEY").expect("Missing env GOOGLE_TRANSLATE_API_KEY");
    let url = format!(
        "https://translation.googleapis.com/language/translate/v2?q={text}&source=en&target={lang}&key={key}",
        lang = lang,
        key = api_key,
        text = utf8_percent_encode(&text, QUERY_ENCODE_SET)
    );

    let mut json: Value = client.get(&url).send().ok()?.json().ok()?;
    if json["data"]["translations"].is_null() {
        return None;
    }
    match json["data"]["translations"][0]["translatedText"].take() {
        Value::String(translated) => Some(translated.replace("| ", "\n").replace("|", "\n")),
        _ => None,
    }
}

#[test]
fn translate_with_newlines_into_ru() {
    if std::env::var_os("GOOGLE_TRANSLATE_API_KEY").is_none() {
        eprintln!("Missing env GOOGLE_TRANSLATE_API_KEY, skipping test");
        return;
    }

    let t = translate_to("ru", "Modify the commands and \nre-run to reach the exit");
    assert_eq!(t, Some("Измените команды и \nповторно запустите, чтобы достичь выхода".into()));
}
