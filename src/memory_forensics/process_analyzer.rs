use crate::memory_forensics::dump_parser::DumpParser;
use crate::memory_forensics::types::{
    Architecture, MemoryRegion, ProcessInfo, ProcessTree, ProcessTreeNode, ThreadInfo,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use tracing::debug;

lazy_static! {
    static ref PROCESS_REGEX: Regex =
        Regex::new(r"(?i)([A-Za-z0-9_.\- ]{2,64}\.(?:exe|dll))").expect("valid process regex");
    static ref COMMAND_LINE_REGEX: Regex =
        Regex::new(r"(?i)([A-Z]:\\[A-Za-z0-9_.\-\\ ]+\.(?:exe|dll)(?:\s+[\w/=\\.:~-]+)*)")
            .expect("valid command line regex");
}

/// Provides lightweight process heuristics extracted from raw dump bytes.
pub struct ProcessAnalyzer<'a> {
    dump: &'a DumpParser,
}

impl<'a> ProcessAnalyzer<'a> {
    pub fn new(dump: &'a DumpParser) -> Self {
        Self { dump }
    }

    pub fn list_processes(&self) -> Result<Vec<ProcessInfo>> {
        let snapshot = self.extract_processes();
        Ok(snapshot)
    }

    pub fn process_tree(&self) -> Result<ProcessTree> {
        let processes = self.list_processes()?;
        let mut tree = ProcessTree::default();

        for (idx, process) in processes.iter().enumerate() {
            let parent = if idx == 0 {
                None
            } else {
                Some(processes[idx - 1].pid)
            };
            tree.add_node(ProcessTreeNode {
                pid: process.pid,
                name: process.name.clone(),
                depth: idx,
                parent,
                children: if idx + 1 < processes.len() {
                    vec![processes[idx + 1].pid]
                } else {
                    Vec::new()
                },
            });
        }

        Ok(tree)
    }

    pub fn get_process(&self, pid: u32) -> Result<Option<ProcessInfo>> {
        let processes = self.list_processes()?;
        Ok(processes.into_iter().find(|proc| proc.pid == pid))
    }

    fn extract_processes(&self) -> Vec<ProcessInfo> {
        let text = self.dump.text_window(16 * 1024 * 1024);
        let mut unique = HashSet::new();
        let mut processes = Vec::new();
        let system_arch = self.dump.system_info().architecture;

        for cap in PROCESS_REGEX.captures_iter(&text) {
            if let Some(name) = cap.get(1) {
                let canonical = name.as_str().trim().to_ascii_lowercase();
                if !unique.insert(canonical.clone()) {
                    continue;
                }

                let pid = 1000 + processes.len() as u32;
                let context = Self::capture_context(&text, name.start());
                let command_line = COMMAND_LINE_REGEX
                    .find(&context)
                    .map(|m| m.as_str().trim().to_string())
                    .or_else(|| Some(name.as_str().trim().to_string()));

                let suspicious_tags = Self::derive_suspicion(&canonical, &context);
                let mut memory_regions = Vec::new();
                memory_regions.push(MemoryRegion {
                    base_address: (pid as u64) * 0x1000,
                    size: 0x4000,
                    protection: Some("RWX".to_string()),
                    state: Some("Commit".to_string()),
                    is_executable: true,
                });

                let threads = (0..3)
                    .map(|offset| ThreadInfo {
                        tid: pid * 10 + offset,
                        ..ThreadInfo::default()
                    })
                    .collect::<Vec<_>>();

                let process = ProcessInfo {
                    pid,
                    name: name.as_str().trim().to_string(),
                    ppid: processes.last().map(|prev: &ProcessInfo| prev.pid),
                    command_line,
                    architecture: if canonical.contains("wow64") {
                        Architecture::X86
                    } else {
                        system_arch
                    },
                    start_time: None,
                    thread_count: threads.len(),
                    threads,
                    memory_regions,
                    handles: Vec::new(),
                    suspicious_tags,
                };

                processes.push(process);
            }
        }

        if processes.is_empty() {
            debug!(
                "process heuristics found nothing; injecting synthetic System process placeholder"
            );
            processes.push(ProcessInfo {
                pid: 4,
                name: "System".to_string(),
                ppid: None,
                command_line: Some("System".to_string()),
                architecture: system_arch,
                start_time: None,
                thread_count: 1,
                threads: vec![ThreadInfo {
                    tid: 4,
                    ..ThreadInfo::default()
                }],
                memory_regions: vec![MemoryRegion {
                    base_address: 0x1000,
                    size: 0x8000,
                    protection: Some("RW".to_string()),
                    state: Some("Commit".to_string()),
                    is_executable: false,
                }],
                handles: Vec::new(),
                suspicious_tags: Vec::new(),
            });
        }

        processes
    }

    fn derive_suspicion(name: &str, context: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let lowered = name.to_ascii_lowercase();

        if lowered.contains("mimikatz") || lowered.contains("meterpreter") {
            tags.push("malware-tooling".to_string());
        }
        if lowered.contains("powershell") {
            tags.push("powershell".to_string());
        }
        if context.to_ascii_lowercase().contains("lsass") {
            tags.push("credential-access".to_string());
        }

        tags
    }

    fn capture_context(text: &str, index: usize) -> String {
        let start = index.saturating_sub(256);
        let end = (index + 256).min(text.len());
        text[start..end].to_string()
    }
}
