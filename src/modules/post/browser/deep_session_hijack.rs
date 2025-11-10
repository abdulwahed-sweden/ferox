//! Mock deep session hijack module (safe, non-destructive).
//! This does NOT perform real browser session theft; it reads a local sample profile file
//! for test-only demonstration of parsing capabilities.
//!
//! TODO: Replace with real session token extraction logic gated behind explicit user consent.

use anyhow::Result;
use async_trait::async_trait;
use std::fs;
use std::path::Path;

use crate::core::module::{Module, ModuleInfo, ModuleOption, ModuleResult, ModuleType};

pub struct DeepSessionHijackMock {
    profile_path: Option<String>,
}

impl DeepSessionHijackMock {
    pub fn new() -> Self { Self { profile_path: None } }
}

impl Default for DeepSessionHijackMock {
    fn default() -> Self { Self::new() }
}

#[async_trait]
impl Module for DeepSessionHijackMock {
    fn info(&self) -> ModuleInfo {
        ModuleInfo {
            name: "deep_session_hijack_mock".into(),
            version: "0.1.0".into(),
            author: "Ferox".into(),
            description: "Mock browser session extractor (reads local sample profile file).".into(),
            module_type: ModuleType::PostExploit,
            category: "post/browser".into(),
        }
    }

    fn options(&self) -> Vec<ModuleOption> {
        vec![ModuleOption {
            name: "profile_path".into(),
            description: "Path to local sample browser profile (test-only).".into(),
            required: false,
            default_value: None,
            current_value: self.profile_path.clone(),
        }]
    }

    fn set_option(&mut self, name: &str, value: &str) -> Result<()> {
        if name == "profile_path" { self.profile_path = Some(value.to_string()); }
        Ok(())
    }

    fn get_option(&self, name: &str) -> Option<String> {
        if name == "profile_path" { return self.profile_path.clone(); }
        None
    }

    fn validate(&self) -> Result<()> { Ok(()) }

    async fn run(&mut self) -> Result<ModuleResult> {
        let path = match &self.profile_path { Some(p) => p, None => "tests/data/sample_profile.txt" };
        let p = Path::new(path);
        if p.exists() {
            let content = fs::read_to_string(p)?;
            Ok(ModuleResult::success("mock profile read").with_data("lines", serde_json::json!(content.lines().count())))
        } else {
            Ok(ModuleResult::error("profile file not found"))
        }
    }
}
