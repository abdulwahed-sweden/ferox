#[cfg(feature = "memory-forensics")]
use crate::core::memory_analysis::MemoryAnalysisDB;
#[cfg(feature = "memory-forensics")]
use crate::memory_forensics::{
    AnalysisReport, CredentialExtractor, DumpParser, MalwareDetector, MitreMapper, NetworkAnalyzer,
    ProcessAnalyzer, RegistryAnalyzer,
    types::{CodeInjectionFinding, CredentialArtifact, MalwareFinding},
};
use anyhow::{Context, Result, bail};
#[cfg(feature = "memory-forensics")]
use rayon::prelude::*;
#[cfg(feature = "memory-forensics")]
use serde::Serialize;
#[cfg(feature = "memory-forensics")]
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum MemoryCommand {
    Analyze {
        dump: PathBuf,
        output: Option<PathBuf>,
        json: bool,
    },
    PsList {
        dump: PathBuf,
    },
    PsTree {
        dump: PathBuf,
    },
    Malfind {
        dump: PathBuf,
    },
    NetScan {
        dump: PathBuf,
    },
    HashDump {
        dump: PathBuf,
    },
    Hivelist {
        dump: PathBuf,
    },
    PrintKey {
        dump: PathBuf,
        key: String,
    },
    YaraScan {
        dump: PathBuf,
        rules: PathBuf,
    },
    Mitre {
        dump: PathBuf,
        output: Option<PathBuf>,
    },
}

pub struct MemoryCli {
    #[cfg(feature = "memory-forensics")]
    db_path: Option<PathBuf>,
}

impl Default for MemoryCli {
    fn default() -> Self {
        Self {
            #[cfg(feature = "memory-forensics")]
            db_path: Some(PathBuf::from("ferox_memory.db")),
        }
    }
}

impl MemoryCli {
    pub fn handle(args: &[&str]) -> Result<()> {
        let command = Self::parse_command(args)?;
        Self::default().execute(command)
    }

    pub fn run_command(command: MemoryCommand) -> Result<()> {
        Self::default().execute(command)
    }

    fn parse_command(args: &[&str]) -> Result<MemoryCommand> {
        if args.is_empty() {
            bail!("usage: memory <subcommand> <dump> [options]");
        }

        let sub = args[0].to_ascii_lowercase();
        match sub.as_str() {
            "analyze" => {
                let (dump, rest) = Self::consume_path(&args[1..])?;
                let mut output = None;
                let mut json = false;
                let mut iter = rest.iter();
                while let Some(flag) = iter.next() {
                    match (*flag).to_ascii_lowercase().as_str() {
                        "--output" => {
                            let path = iter
                                .next()
                                .map(|p| PathBuf::from(p))
                                .context("--output requires a path")?;
                            output = Some(path);
                        }
                        "--json" | "-j" => json = true,
                        _ => bail!("unknown option '{flag}' for analyze"),
                    }
                }
                Ok(MemoryCommand::Analyze { dump, output, json })
            }
            "pslist" => Ok(MemoryCommand::PsList {
                dump: Self::consume_path_only(&args[1..])?,
            }),
            "pstree" => Ok(MemoryCommand::PsTree {
                dump: Self::consume_path_only(&args[1..])?,
            }),
            "malfind" => Ok(MemoryCommand::Malfind {
                dump: Self::consume_path_only(&args[1..])?,
            }),
            "netscan" => Ok(MemoryCommand::NetScan {
                dump: Self::consume_path_only(&args[1..])?,
            }),
            "hashdump" => Ok(MemoryCommand::HashDump {
                dump: Self::consume_path_only(&args[1..])?,
            }),
            "hivelist" => Ok(MemoryCommand::Hivelist {
                dump: Self::consume_path_only(&args[1..])?,
            }),
            "printkey" => {
                let (dump, rest) = Self::consume_path(&args[1..])?;
                let mut key_value = None;
                let mut iter = rest.iter();
                while let Some(flag) = iter.next() {
                    match flag.as_str() {
                        "--key" => {
                            key_value = iter.next().cloned();
                        }
                        other => bail!("unknown option '{other}' for printkey"),
                    }
                }
                let key_value = key_value.context("printkey requires --key <registry path>")?;
                Ok(MemoryCommand::PrintKey {
                    dump,
                    key: key_value,
                })
            }
            "yarascan" => {
                let (dump, rest) = Self::consume_path(&args[1..])?;
                let mut rules_path = None;
                let mut iter = rest.iter();
                while let Some(flag) = iter.next() {
                    match flag.as_str() {
                        "--rules" => {
                            rules_path = iter.next().cloned();
                        }
                        other => bail!("unknown option '{other}' for yarascan"),
                    }
                }
                let rules = rules_path.context("yarascan requires --rules <path>")?;
                Ok(MemoryCommand::YaraScan {
                    dump,
                    rules: PathBuf::from(rules),
                })
            }
            "mitre" => {
                let (dump, rest) = Self::consume_path(&args[1..])?;
                let mut output = None;
                let mut iter = rest.iter();
                while let Some(flag) = iter.next() {
                    if (*flag).to_ascii_lowercase() == "--output" {
                        let path = iter
                            .next()
                            .map(|v| PathBuf::from(v))
                            .context("--output requires a file path")?;
                        output = Some(path);
                    }
                }
                Ok(MemoryCommand::Mitre { dump, output })
            }
            _ => bail!("unknown memory subcommand '{sub}'"),
        }
    }

    fn consume_path(args: &[&str]) -> Result<(PathBuf, Vec<String>)> {
        let path = Self::consume_path_only(args)?;
        let rest = args.get(1..).unwrap_or(&[]);
        Ok((path, rest.iter().map(|s| s.to_string()).collect()))
    }

    fn consume_path_only(args: &[&str]) -> Result<PathBuf> {
        let raw = args.first().context("command requires dump path")?;
        Ok(PathBuf::from(raw))
    }

    fn execute(&self, command: MemoryCommand) -> Result<()> {
        #[cfg(not(feature = "memory-forensics"))]
        {
            let _ = command;
            bail!(
                "Ferox built without memory-forensics feature. Rebuild with `--features memory-forensics`."
            );
        }

        #[cfg(feature = "memory-forensics")]
        {
            match command {
                MemoryCommand::Analyze { dump, output, json } => {
                    self.run_analyze(dump, output, json)
                }
                MemoryCommand::PsList { dump } => self.run_pslist(dump),
                MemoryCommand::PsTree { dump } => self.run_pstree(dump),
                MemoryCommand::Malfind { dump } => self.run_malfind(dump),
                MemoryCommand::NetScan { dump } => self.run_netscan(dump),
                MemoryCommand::HashDump { dump } => self.run_hashdump(dump),
                MemoryCommand::Hivelist { dump } => self.run_hivelist(dump),
                MemoryCommand::PrintKey { dump, key } => self.run_printkey(dump, &key),
                MemoryCommand::YaraScan { dump, rules } => self.run_yarascan(dump, rules),
                MemoryCommand::Mitre { dump, output } => self.run_mitre(dump, output),
            }
        }
    }

    #[cfg(feature = "memory-forensics")]
    fn run_analyze(&self, dump_path: PathBuf, output: Option<PathBuf>, json: bool) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let report = self.build_report(&dump)?;

        Self::print_report(&report);
        if let Some(path) = output {
            self.write_json(&path, &report)?;
            println!("[+] Analysis exported to {}", path.display());
        } else if json {
            let json = serde_json::to_string_pretty(&report)?;
            println!("{json}");
        }

        if let Some(db_path) = &self.db_path {
            let db_path_string = db_path.to_string_lossy();
            if let Ok(db) = MemoryAnalysisDB::connect(&db_path_string) {
                let _ = db.store_analysis(&report);
            }
        }

        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_pslist(&self, dump_path: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let analyzer = ProcessAnalyzer::new(&dump);
        let procs = analyzer.list_processes()?;
        println!("PID\tPPID\tThreads\tProcess");
        for proc in procs {
            println!(
                "{}\t{}\t{}\t{}",
                proc.pid,
                proc.ppid
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                proc.thread_count,
                proc.name
            );
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_pstree(&self, dump_path: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let analyzer = ProcessAnalyzer::new(&dump);
        let tree = analyzer.process_tree()?;
        tree.print_hierarchical();
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_malfind(&self, dump_path: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let detector = MalwareDetector::new(&dump);
        let injections = detector.find_code_injections()?;
        let malware = detector.detect_malware_strings()?;

        for inj in injections {
            println!("[!] Possible injection: {}", inj.description);
        }
        for finding in malware {
            println!(
                "[!] Malware indicator: {} - {}",
                finding.indicator, finding.description
            );
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_netscan(&self, dump_path: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let analyzer = NetworkAnalyzer::new(&dump);
        let connections = analyzer.list_connections()?;
        for conn in connections {
            let remote = conn
                .remote_addr
                .map(|addr| format!("{}:{}", addr, conn.remote_port.unwrap_or(0)))
                .unwrap_or_else(|| "-".to_string());
            println!(
                "{}:{} -> {} [{}]",
                conn.local_addr,
                conn.local_port,
                remote,
                conn.state.unwrap_or_else(|| "Unknown".to_string())
            );
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_hashdump(&self, dump_path: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let extractor = CredentialExtractor::new(&dump);
        let hashes = extractor.extract_hashes()?;
        for hash in hashes {
            println!("[hash] {}", hash.identifier);
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_hivelist(&self, dump_path: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let registry = RegistryAnalyzer::new(&dump);
        for key in registry.list_registry_keys()? {
            println!("{}", key.path);
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_printkey(&self, dump_path: PathBuf, key: &str) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let registry = RegistryAnalyzer::new(&dump);
        if let Some(reg_key) = registry.get_key(key)? {
            println!("[{}]", reg_key.path);
            for (name, value) in reg_key.values {
                println!("  {} = {}", name, value);
            }
        } else {
            println!("[-] Registry key not found: {}", key);
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_yarascan(&self, dump_path: PathBuf, rules: PathBuf) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let detector = MalwareDetector::new(&dump);
        match detector.scan_yara(rules.to_str().unwrap_or("")) {
            Ok(results) => {
                for finding in results {
                    println!("[YARA] {}", finding.indicator);
                }
            }
            Err(err) => {
                println!("[!] YARA scan unavailable: {err}");
            }
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn run_mitre(&self, dump_path: PathBuf, output: Option<PathBuf>) -> Result<()> {
        let dump = Self::load_dump(&dump_path)?;
        let analyzer = ProcessAnalyzer::new(&dump);
        let processes = analyzer.list_processes()?;
        let detector = MalwareDetector::new(&dump);
        let malware = detector.detect_malware_strings()?;
        let network = NetworkAnalyzer::new(&dump).list_connections()?;
        let techniques = MitreMapper::map(&processes, &malware, &network);

        if let Some(path) = output {
            self.write_json(&path, &techniques)?;
            println!("[+] MITRE techniques exported to {}", path.display());
        } else {
            for tech in techniques {
                println!("{} - {} ({})", tech.id, tech.name, tech.tactic);
            }
        }
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn load_dump(path: &Path) -> Result<DumpParser> {
        DumpParser::from_file(path)
    }

    #[cfg(feature = "memory-forensics")]
    fn build_report(&self, dump: &DumpParser) -> Result<AnalysisReport> {
        let processes = ProcessAnalyzer::new(dump).list_processes()?;
        let detector = MalwareDetector::new(dump);
        let code_injections = detector.find_code_injections()?;
        let malware_findings = detector.detect_malware_strings()?;
        let network = NetworkAnalyzer::new(dump);
        let connections = network.list_connections()?;
        let mut artifacts = network.extract_dns_cache()?;
        artifacts.extend(network.extract_urls()?);
        let registry = RegistryAnalyzer::new(dump).list_registry_keys()?;
        let credentials = {
            let extractor = CredentialExtractor::new(dump);
            let mut creds = extractor.extract_hashes()?;
            creds.extend(extractor.extract_kerberos_tickets()?);
            creds.extend(extractor.extract_browser_passwords()?);
            creds.extend(extractor.extract_lsa_secrets()?);
            creds.extend(extractor.extract_tokens()?);
            creds
        };

        let mitre = MitreMapper::map(&processes, &malware_findings, &connections);
        let risk_score = Self::calculate_risk(&code_injections, &malware_findings, &credentials);
        let suspicious = processes
            .par_iter()
            .filter(|proc| proc.is_suspicious())
            .count();

        Ok(AnalysisReport {
            dump_path: dump.path().display().to_string(),
            analysis_time: dump.analysis_time(),
            dump_type: dump.detect_type(),
            system_info: dump.system_info(),
            processes,
            code_injections,
            malware_findings,
            network_connections: connections,
            network_artifacts: artifacts,
            registry_keys: registry,
            credentials,
            mitre_techniques: mitre,
            risk_score: Some((risk_score + suspicious as u8).min(100)),
        })
    }

    #[cfg(feature = "memory-forensics")]
    fn calculate_risk(
        injections: &[CodeInjectionFinding],
        malware: &[MalwareFinding],
        credentials: &[CredentialArtifact],
    ) -> u8 {
        let mut score = 10;
        score += (injections.len() as u8) * 10;
        score += (malware.len() as u8) * 8;
        score += (credentials.len() as u8) * 5;
        score.min(100)
    }

    #[cfg(feature = "memory-forensics")]
    fn print_report(report: &AnalysisReport) {
        println!("=== Ferox Memory Analysis ===");
        println!("Dump: {}", report.dump_path);
        println!("Type: {:?}", report.dump_type);
        println!("Analyzed: {}", report.analysis_time);
        println!("OS: {}", report.system_info.os_version);
        println!("Processes: {}", report.processes.len());
        println!(
            "Code injections: {} | Malware indicators: {} | Credentials: {}",
            report.code_injections.len(),
            report.malware_findings.len(),
            report.credentials.len()
        );
        if let Some(score) = report.risk_score {
            println!("Risk score: {}", score);
        }
    }

    #[cfg(feature = "memory-forensics")]
    fn write_json<T: Serialize>(&self, path: &Path, data: &T) -> Result<()> {
        let mut file = File::create(path)?;
        let json = serde_json::to_string_pretty(data)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
