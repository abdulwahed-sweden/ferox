use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};

#[cfg(feature = "memory-forensics")]
use crate::memory_forensics::types::{
    AnalysisReport, CodeInjectionFinding, MalwareFinding, MitreTechnique, NetworkConnection,
    ProcessInfo,
};

pub struct MemoryAnalysisDB {
    conn: Connection,
}

impl MemoryAnalysisDB {
    pub fn connect(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS memory_dumps (
                id INTEGER PRIMARY KEY,
                dump_path TEXT NOT NULL,
                analysis_time TEXT NOT NULL,
                dump_type TEXT NOT NULL,
                os_version TEXT,
                architecture TEXT,
                risk_score INTEGER
            );

            CREATE TABLE IF NOT EXISTS processes (
                id INTEGER PRIMARY KEY,
                dump_id INTEGER NOT NULL,
                pid INTEGER NOT NULL,
                name TEXT NOT NULL,
                ppid INTEGER,
                suspicious INTEGER,
                suspicious_tags TEXT,
                FOREIGN KEY(dump_id) REFERENCES memory_dumps(id)
            );

            CREATE TABLE IF NOT EXISTS code_injections (
                id INTEGER PRIMARY KEY,
                dump_id INTEGER NOT NULL,
                pid INTEGER,
                description TEXT NOT NULL,
                severity TEXT NOT NULL,
                FOREIGN KEY(dump_id) REFERENCES memory_dumps(id)
            );

            CREATE TABLE IF NOT EXISTS mitre_techniques (
                id INTEGER PRIMARY KEY,
                dump_id INTEGER NOT NULL,
                technique_id TEXT NOT NULL,
                technique_name TEXT NOT NULL,
                tactic TEXT NOT NULL,
                confidence REAL,
                FOREIGN KEY(dump_id) REFERENCES memory_dumps(id)
            );

            CREATE TABLE IF NOT EXISTS malware_findings (
                id INTEGER PRIMARY KEY,
                dump_id INTEGER NOT NULL,
                indicator TEXT NOT NULL,
                description TEXT,
                severity TEXT,
                FOREIGN KEY(dump_id) REFERENCES memory_dumps(id)
            );

            CREATE TABLE IF NOT EXISTS network_connections (
                id INTEGER PRIMARY KEY,
                dump_id INTEGER NOT NULL,
                local_addr TEXT NOT NULL,
                local_port INTEGER,
                remote_addr TEXT,
                remote_port INTEGER,
                protocol TEXT,
                state TEXT,
                FOREIGN KEY(dump_id) REFERENCES memory_dumps(id)
            );
            "#,
        )?;
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    pub fn store_analysis(&self, report: &AnalysisReport) -> Result<i64> {
        let architecture = format!("{:?}", report.system_info.architecture);
        let risk_score = report.risk_score.map(|v| v as i64);
        let analysis_time = Self::format_time(report.analysis_time);

        self.conn.execute(
            "INSERT INTO memory_dumps \
             (dump_path, analysis_time, dump_type, os_version, architecture, risk_score) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                report.dump_path,
                analysis_time,
                format!("{:?}", report.dump_type),
                report.system_info.os_version,
                architecture,
                risk_score
            ],
        )?;
        let dump_id = self.conn.last_insert_rowid();

        for process in &report.processes {
            self.store_process(dump_id, process)?;
        }
        for injection in &report.code_injections {
            self.store_injection(dump_id, injection)?;
        }
        for technique in &report.mitre_techniques {
            self.store_mitre(dump_id, technique)?;
        }
        for finding in &report.malware_findings {
            self.store_malware(dump_id, finding)?;
        }
        for connection in &report.network_connections {
            self.store_connection(dump_id, connection)?;
        }

        Ok(dump_id)
    }

    #[cfg(feature = "memory-forensics")]
    fn store_process(&self, dump_id: i64, process: &ProcessInfo) -> Result<()> {
        let tags = serde_json::to_string(&process.suspicious_tags)?;
        self.conn.execute(
            "INSERT INTO processes (dump_id, pid, name, ppid, suspicious, suspicious_tags) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                dump_id,
                process.pid,
                &process.name,
                process.ppid,
                process.is_suspicious() as i64,
                tags
            ],
        )?;
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn store_injection(&self, dump_id: i64, injection: &CodeInjectionFinding) -> Result<()> {
        self.conn.execute(
            "INSERT INTO code_injections (dump_id, pid, description, severity) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                dump_id,
                injection.pid,
                &injection.description,
                format!("{:?}", injection.severity)
            ],
        )?;
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn store_mitre(&self, dump_id: i64, technique: &MitreTechnique) -> Result<()> {
        self.conn.execute(
            "INSERT INTO mitre_techniques (dump_id, technique_id, technique_name, tactic, confidence) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                dump_id,
                &technique.id,
                &technique.name,
                &technique.tactic,
                technique.confidence
            ],
        )?;
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn store_malware(&self, dump_id: i64, finding: &MalwareFinding) -> Result<()> {
        self.conn.execute(
            "INSERT INTO malware_findings (dump_id, indicator, description, severity) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                dump_id,
                &finding.indicator,
                &finding.description,
                format!("{:?}", finding.severity)
            ],
        )?;
        Ok(())
    }

    #[cfg(feature = "memory-forensics")]
    fn store_connection(&self, dump_id: i64, connection: &NetworkConnection) -> Result<()> {
        self.conn.execute(
            "INSERT INTO network_connections \
             (dump_id, local_addr, local_port, remote_addr, remote_port, protocol, state) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                dump_id,
                &connection.local_addr,
                connection.local_port,
                connection.remote_addr,
                connection.remote_port,
                &connection.protocol,
                connection.state
            ],
        )?;
        Ok(())
    }

    fn format_time(time: DateTime<Utc>) -> String {
        time.to_rfc3339()
    }
}
