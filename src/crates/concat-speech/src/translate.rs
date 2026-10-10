// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer and Concat contributors

//! Caption lines into Spanish, through Google Translate's public web
//! endpoint. Free, no key and no account; the one part of captions that is
//! not on this machine, so it is asked for and never assumed.
//!
//! Lines are sent in batches of neighbours rather than one by one, so the
//! translator sees the sentence around each line. When Google hands back a
//! different number of lines than were sent, the batch is redone line by
//! line so a caption never ends up under the wrong words.

use std::time::Duration;

/// Tried in turn; the second is the web page's own host.
const ENDPOINTS: [&str; 2] = [
    "https://translate.googleapis.com/translate_a/single",
    "https://translate.google.com/translate_a/single",
];
/// Most characters and most lines in one request.
const BATCH_CHARS: usize = 3500;
const BATCH_LINES: usize = 25;
const AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                     (KHTML, like Gecko) Chrome/124.0 Safari/537.36";

/// `lines` in Spanish, one for one and in the same order. `progress` is
/// told how far along it is, in percent.
pub fn to_spanish(lines: &[String], progress: impl Fn(i32)) -> Result<Vec<String>, String> {
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut start = 0;
    while start < lines.len() {
        let mut end = start;
        let mut chars = 0;
        while end < lines.len()
            && end - start < BATCH_LINES
            && (end == start || chars + lines[end].len() < BATCH_CHARS)
        {
            chars += lines[end].len() + 1;
            end += 1;
        }
        out.extend(translate_batch(&lines[start..end])?);
        start = end;
        progress((start * 100 / lines.len()) as i32);
    }
    Ok(out)
}

fn translate_batch(batch: &[String]) -> Result<Vec<String>, String> {
    let flat: Vec<String> = batch.iter().map(|line| line.replace('\n', " ")).collect();
    let translated = ask(&flat.join("\n"))?;
    let lines: Vec<String> = translated
        .split('\n')
        .map(|line| line.trim().to_owned())
        .collect();
    if lines.len() == batch.len() {
        return Ok(lines);
    }
    let mut one_by_one = Vec::with_capacity(flat.len());
    for line in &flat {
        if line.trim().is_empty() {
            one_by_one.push(String::new());
        } else {
            one_by_one.push(ask(line)?.replace('\n', " ").trim().to_owned());
        }
    }
    Ok(one_by_one)
}

/// One request, retried and moved between hosts before it gives up.
fn ask(text: &str) -> Result<String, String> {
    let mut last = String::from("no reply");
    for attempt in 0..3u64 {
        for endpoint in ENDPOINTS {
            match request(endpoint, text) {
                Ok(translated) => return Ok(translated),
                Err(error) => last = error,
            }
        }
        std::thread::sleep(Duration::from_secs(3 * (attempt + 1)));
    }
    Err(format!(
        "could not reach Google Translate: {last}. Check the internet connection, \
         the firewall or a VPN"
    ))
}

fn request(endpoint: &str, text: &str) -> Result<String, String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(30))
        .user_agent(AGENT)
        .build();
    let body = agent
        .post(endpoint)
        .send_form(&[
            ("client", "gtx"),
            ("sl", "auto"),
            ("tl", "es"),
            ("dt", "t"),
            ("q", text),
        ])
        .map_err(|error| error.to_string())?
        .into_string()
        .map_err(|error| error.to_string())?;
    parse(&body)
}

/// The translation out of Google's reply: `[[["Hola\n","Hello\n",..],..],..]`.
fn parse(body: &str) -> Result<String, String> {
    let value: serde_json::Value = serde_json::from_str(body).map_err(|error| error.to_string())?;
    let parts = value
        .get(0)
        .and_then(|part| part.as_array())
        .ok_or_else(|| "unexpected reply".to_owned())?;
    let text: String = parts
        .iter()
        .filter_map(|part| part.get(0).and_then(|piece| piece.as_str()))
        .collect();
    if text.is_empty() {
        Err("empty reply".to_owned())
    } else {
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    /// The reply's pieces are joined, line breaks and all.
    #[test]
    fn a_reply_is_read_into_its_lines() {
        let body = r#"[[["Hola\n","Hello\n",null,null,1],["mundo","world",null,null,1]],null,"en"]"#;
        assert_eq!(parse(body).unwrap(), "Hola\nmundo");
        assert!(parse("[]").is_err());
        assert!(parse("not json").is_err());
    }
}
