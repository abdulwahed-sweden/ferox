use crate::memory_forensics::dump_parser::DumpParser;
use crate::memory_forensics::types::{CredentialArtifact, CredentialType};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref HASH_REGEX: Regex =
        Regex::new(r"(?i)[0-9a-f]{32}:[0-9a-f]{32}").expect("valid hash pattern");
    static ref KERBEROS_REGEX: Regex =
        Regex::new(r"(?i)(krbtgt|kerberos|ticket)").expect("valid kerberos pattern");
    static ref BROWSER_REGEX: Regex =
        Regex::new(r"(?i)(password=|Cookie:|Authorization: Basic)").expect("valid browser pattern");
}

pub struct CredentialExtractor<'a> {
    dump: &'a DumpParser,
}

impl<'a> CredentialExtractor<'a> {
    pub fn new(dump: &'a DumpParser) -> Self {
        Self { dump }
    }

    pub fn extract_hashes(&self) -> Result<Vec<CredentialArtifact>> {
        self.collect_artifacts(&HASH_REGEX, CredentialType::NtlmHash)
    }

    pub fn extract_kerberos_tickets(&self) -> Result<Vec<CredentialArtifact>> {
        self.collect_artifacts(&KERBEROS_REGEX, CredentialType::KerberosTicket)
    }

    pub fn extract_browser_passwords(&self) -> Result<Vec<CredentialArtifact>> {
        self.collect_artifacts(&BROWSER_REGEX, CredentialType::BrowserCredential)
    }

    pub fn extract_lsa_secrets(&self) -> Result<Vec<CredentialArtifact>> {
        let window = self.dump.text_window(4 * 1024 * 1024);
        if window.to_ascii_lowercase().contains("lsass") {
            Ok(vec![CredentialArtifact {
                credential_type: CredentialType::LsaSecret,
                identifier: "LSASecret".to_string(),
                data: [(
                    "note".to_string(),
                    "LSA indicator present in dump".to_string(),
                )]
                .into_iter()
                .collect(),
                is_encrypted: true,
            }])
        } else {
            Ok(Vec::new())
        }
    }

    pub fn extract_tokens(&self) -> Result<Vec<CredentialArtifact>> {
        let window = self.dump.text_window(4 * 1024 * 1024);
        if window.contains("TOKEN") {
            Ok(vec![CredentialArtifact {
                credential_type: CredentialType::Token,
                identifier: "AccessToken".to_string(),
                data: [(
                    "note".to_string(),
                    "Access token string referenced".to_string(),
                )]
                .into_iter()
                .collect(),
                is_encrypted: false,
            }])
        } else {
            Ok(Vec::new())
        }
    }

    fn collect_artifacts(
        &self,
        regex: &Regex,
        typ: CredentialType,
    ) -> Result<Vec<CredentialArtifact>> {
        let window = self.dump.text_window(4 * 1024 * 1024);
        let mut artifacts = Vec::new();
        let mut seen = HashSet::new();

        for cap in regex.captures_iter(&window) {
            let matched = cap
                .get(0)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();
            if !seen.insert(matched.clone()) {
                continue;
            }
            artifacts.push(CredentialArtifact {
                credential_type: typ.clone(),
                identifier: matched.clone(),
                data: [("artifact".to_string(), matched)].into_iter().collect(),
                is_encrypted: typ == CredentialType::NtlmHash,
            });
        }

        Ok(artifacts)
    }
}
