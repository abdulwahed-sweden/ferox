use crate::memory_forensics::types::{Architecture, DumpType, SystemInfo};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use memmap2::Mmap;
use std::borrow::Cow;
use std::fs::File;
use std::path::{Path, PathBuf};
use tracing::debug;

/// Wrapper around raw memory dump data with light-weight helpers
pub struct DumpParser {
    path: PathBuf,
    buffer: Vec<u8>,
    dump_type: DumpType,
    system_info: SystemInfo,
    created_at: Option<DateTime<Utc>>,
    file_size: u64,
    analysis_time: DateTime<Utc>,
}

impl DumpParser {
    /// Load a dump file into memory. Uses memory mapping for large files when available.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path).with_context(|| {
            format!(
                "unable to open memory dump '{}'. check path and permissions",
                path.display()
            )
        })?;
        let metadata = file.metadata()?;
        let file_size = metadata.len();

        // Try memory mapping first to avoid extra copies for large dumps.
        let buffer = match unsafe { Mmap::map(&file) } {
            Ok(mmap) => mmap.to_vec(),
            Err(error) => {
                debug!(
                    "falling back to buffered read for {}: {error}",
                    path.display()
                );
                std::fs::read(path).with_context(|| {
                    format!("unable to read dump file '{}' into memory", path.display())
                })?
            }
        };

        let dump_type = Self::infer_dump_type(path, &buffer);
        let system_info = Self::infer_system_info(&buffer);
        let created_at = metadata
            .modified()
            .ok()
            .map(DateTime::<Utc>::from);

        Ok(Self {
            path: path.to_path_buf(),
            buffer,
            dump_type,
            system_info,
            created_at,
            file_size,
            analysis_time: Utc::now(),
        })
    }

    /// Returns view over raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }

    /// Returns a lossy UTF-8 view over the first `limit` bytes.
    pub fn text_window(&self, limit: usize) -> Cow<'_, str> {
        let end = self.buffer.len().min(limit);
        String::from_utf8_lossy(&self.buffer[..end])
    }

    pub fn detect_type(&self) -> DumpType {
        self.dump_type
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn system_info(&self) -> SystemInfo {
        self.system_info.clone()
    }

    pub fn analysis_time(&self) -> DateTime<Utc> {
        self.analysis_time
    }

    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    fn infer_dump_type(path: &Path, buffer: &[u8]) -> DumpType {
        if buffer.len() >= 4 {
            let magic = &buffer[..4];
            if magic == b"PAGE" {
                return DumpType::Kernel;
            }
            if magic == b"MDMP" || magic == b"PMDM" {
                return DumpType::MiniDump;
            }
        }

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("dmp") | Some("mdmp") => DumpType::MiniDump,
            Some("raw") | Some("img") => DumpType::Full,
            _ => DumpType::Unknown,
        }
    }

    fn infer_system_info(buffer: &[u8]) -> SystemInfo {
        let window = String::from_utf8_lossy(&buffer[..buffer.len().min(4 * 1024 * 1024)]);

        let architecture = if window.contains("AMD64") || window.contains("x64") {
            Architecture::X64
        } else if window.contains("ARM64") {
            Architecture::Arm64
        } else if window.contains("x86") || window.contains("i386") {
            Architecture::X86
        } else {
            Architecture::Unknown
        };

        let os_version = window
            .match_indices("Windows")
            .next()
            .map(|(idx, _)| {
                let slice = &window[idx..window.len().min(idx + 64)];
                slice
                    .split_whitespace()
                    .take(5)
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_else(|| "Windows (unknown)".to_string());

        let hostname = window
            .match_indices("Hostname")
            .next()
            .map(|(idx, _)| {
                window[idx..window.len().min(idx + 48)]
                    .split(':')
                    .nth(1)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default()
            })
            .filter(|s| !s.is_empty());

        SystemInfo {
            os_version,
            architecture,
            build_number: None,
            hostname,
            uptime: None,
            memory_mb: None,
            cpu_description: None,
        }
    }
}
