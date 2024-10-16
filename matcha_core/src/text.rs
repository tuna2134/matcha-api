use crate::error::Result;
use jpreprocess::{
    kind::JPreprocessDictionaryKind, DefaultFetcher, JPreprocess, JPreprocessConfig,
    SystemDictionaryConfig,
};
use once_cell::sync::Lazy;

use std::collections::HashMap;

pub struct TextPreprocessor {
    pp: JPreprocess<DefaultFetcher>,
}

impl TextPreprocessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            pp: JPreprocess::from_config(JPreprocessConfig {
                dictionary: SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic),
                user_dictionary: None,
            })?,
        })
    }

    pub fn g2p(&self, text: &str) -> Result<Vec<String>> {
        let labels = self.pp.extract_fullcontext(text)?;
        let mut results: Vec<String> = Vec::new();

        for (i, label) in labels.iter().enumerate() {
            let mut p3 = label.phoneme.c.as_ref().unwrap().to_string();

            // p3 in "AIUEO"
            if "AIUEO".contains(&p3) {
                p3 = p3.to_lowercase();
            }

            if p3 == "sil" {
                assert!(i == 0 || i == labels.len() - 1);
                if i == 0 {
                    results.push("^".to_string());
                } else if i == labels.len() - 1 {
                    let e3 = label.accent_phrase_prev.clone().unwrap().is_interrogative;
                    if e3 {
                        results.push("$".to_string());
                    } else {
                        results.push("?".to_string());
                    }
                    continue;
                }
            } else if p3 == "pau" {
                results.push("_".to_string());
                continue;
            } else {
                results.push(p3);
            }

            let a1 = label
                .mora
                .as_ref()
                .map(|m| m.relative_accent_position)
                .unwrap_or(-50);
            let a2 = label
                .mora
                .as_ref()
                .map(|m| m.position_forward as i8)
                .unwrap_or(-50);
            let a3 = label
                .mora
                .as_ref()
                .map(|m| m.position_backward as i8)
                .unwrap_or(-50);

            let f1 = label
                .accent_phrase_curr
                .as_ref()
                .map(|a| a.mora_count as i8)
                .unwrap_or(-50);

            let a2_next = labels[i + 1]
                .mora
                .as_ref()
                .map(|m| m.position_forward as i8)
                .unwrap_or(-50);

            if a3 == 1 && a2_next == 1 {
                results.push("#".to_string());
            } else if a1 == 0 && a2_next == a2 + 1 && a2 != f1 {
                results.push("]".to_string());
            } else if a2 == 1 && a2_next == 2 {
                results.push("[".to_string());
            }
        }

        Ok(results)
    }
}

const SYMBOL2ID: Lazy<HashMap<String, u32>> = Lazy::new(|| {
    let mut symbol2id = HashMap::new();
    let symbols: Vec<String> = serde_json::from_str(include_str!("./symbols.json")).unwrap();
    for (i, symbol) in symbols.iter().enumerate() {
        symbol2id.insert(symbol.clone(), i as u32);
    }
    symbol2id
});

pub fn txt2seq(text: &str, pp: TextPreprocessor) -> Result<Vec<u32>> {
    let mut sequence: Vec<u32> = Vec::new();
    let clean_text = pp.g2p(text)?;
    for symbol in clean_text {
        let id = *SYMBOL2ID.get(&symbol).unwrap();
        sequence.push(id);
    }
    Ok(sequence)
}
