use crate::memory_forensics::types::{
    MalwareFinding, MitreTechnique, NetworkConnection, ProcessInfo,
};

/// Lightweight helper that maps heuristics to MITRE ATT&CK techniques.
pub struct MitreMapper;

impl MitreMapper {
    pub fn map(
        processes: &[ProcessInfo],
        malware: &[MalwareFinding],
        connections: &[NetworkConnection],
    ) -> Vec<MitreTechnique> {
        let mut techniques = Vec::new();

        for process in processes {
            if process
                .suspicious_tags
                .iter()
                .any(|tag| tag == "malware-tooling")
            {
                techniques.push(MitreTechnique {
                    id: "T1059".to_string(),
                    name: "Command and Scripting Interpreter".to_string(),
                    tactic: "Execution".to_string(),
                    confidence: 0.6,
                });
            }
            if process
                .suspicious_tags
                .iter()
                .any(|tag| tag == "credential-access")
            {
                techniques.push(MitreTechnique {
                    id: "T1003".to_string(),
                    name: "OS Credential Dumping".to_string(),
                    tactic: "Credential Access".to_string(),
                    confidence: 0.7,
                });
            }
        }

        for finding in malware {
            let (id, tactic) = match finding.indicator.as_str() {
                "mimikatz" => ("T1003", "Credential Access"),
                "meterpreter" => ("T1059", "Execution"),
                _ => ("T1204", "Execution"),
            };

            techniques.push(MitreTechnique {
                id: id.to_string(),
                name: finding.description.clone(),
                tactic: tactic.to_string(),
                confidence: 0.8,
            });
        }

        if connections.iter().any(|conn| conn.remote_addr.is_some()) {
            techniques.push(MitreTechnique {
                id: "T1090".to_string(),
                name: "Proxy".to_string(),
                tactic: "Command and Control".to_string(),
                confidence: 0.5,
            });
        }

        techniques
    }
}
