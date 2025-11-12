#![cfg(feature = "volatility-bridge")]

use crate::memory_forensics::types::ProcessInfo;
use anyhow::{Result, anyhow};
use pyo3::prelude::*;
use serde::de::DeserializeOwned;

/// Thin wrapper around Volatility plugins using PyO3.
pub struct VolatilityBridge;

impl VolatilityBridge {
    pub fn new() -> Result<Self> {
        // PyO3 auto-initialize is enabled via feature flag.
        Python::with_gil(|_py| Ok(()))?;
        Ok(Self)
    }

    pub fn run_plugin(&self, plugin: &str, dump_path: &str) -> Result<String> {
        Python::with_gil(|py| {
            let volatility = py.import("volatility3.cli")?;
            let run_cli = volatility.getattr("main")?;
            let args = vec!["volatility", "-f", dump_path, plugin];
            run_cli.call1((args,))?;
            Ok(String::from("Plugin executed"))
        })
    }

    pub fn parse_result<T>(&self, data: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(data).map_err(|err| anyhow!(err))
    }

    pub fn list_processes(&self, dump_path: &str) -> Result<Vec<ProcessInfo>> {
        let json = self.run_plugin("windows.pslist", dump_path)?;
        self.parse_result(&json)
    }
}
