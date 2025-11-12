use crate::memory_forensics::dump_parser::DumpParser;
use crate::memory_forensics::types::RegistryKey;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref REG_KEY_REGEX: Regex = Regex::new(
        r"(?i)(HKEY_(?:LOCAL_MACHINE|CURRENT_USER|USERS|CLASSES_ROOT)\\[\\A-Za-z0-9_\-\\s]+)"
    )
    .expect("valid registry regex");
    static ref REG_VALUE_REGEX: Regex =
        Regex::new(r"(?m)^\s*([A-Za-z0-9_\- ]+)\s*REG_[A-Z_]+\s*(.+)$").expect("valid value regex");
}

pub struct RegistryAnalyzer<'a> {
    dump: &'a DumpParser,
}

impl<'a> RegistryAnalyzer<'a> {
    pub fn new(dump: &'a DumpParser) -> Self {
        Self { dump }
    }

    pub fn list_registry_keys(&self) -> Result<Vec<RegistryKey>> {
        let window = self.dump.text_window(4 * 1024 * 1024);
        let mut keys = Vec::new();
        for cap in REG_KEY_REGEX.captures_iter(&window) {
            if let Some(key) = cap.get(1) {
                let context = Self::extract_context(&window, key.start(), 768);
                let values = Self::parse_values(&context);
                keys.push(RegistryKey {
                    path: key.as_str().to_string(),
                    values,
                    last_write_time: None,
                });
            }
        }
        Ok(keys)
    }

    pub fn get_key(&self, key_path: &str) -> Result<Option<RegistryKey>> {
        let window = self.dump.text_window(4 * 1024 * 1024);
        if let Some(pos) = window
            .to_ascii_uppercase()
            .find(&key_path.to_ascii_uppercase())
        {
            let context = Self::extract_context(&window, pos, 1024);
            let values = Self::parse_values(&context);
            return Ok(Some(RegistryKey {
                path: key_path.to_string(),
                values,
                last_write_time: None,
            }));
        }
        Ok(None)
    }

    fn extract_context(window: &str, index: usize, span: usize) -> String {
        let start = index.saturating_sub(span / 2);
        let end = (index + span).min(window.len());
        window[start..end].to_string()
    }

    fn parse_values(context: &str) -> HashMap<String, String> {
        let mut values = HashMap::new();
        for cap in REG_VALUE_REGEX.captures_iter(context) {
            let name = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let value = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            if !name.is_empty() && !value.is_empty() {
                values.insert(name.to_string(), value.to_string());
            }
        }
        values
    }
}
