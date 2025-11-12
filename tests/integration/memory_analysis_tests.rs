use ferox::memory_forensics::{
    CredentialExtractor, DumpParser, MalwareDetector, NetworkAnalyzer, ProcessAnalyzer,
};
use std::io::Write;

#[test]
fn synthetic_dump_analysis_runs() {
    let mut tmp = tempfile::NamedTempFile::new().expect("tempfile");
    writeln!(
        tmp,
        "System Process\nlsass.exe mimikatz VirtualAllocEx 192.168.0.5:443 -> 10.0.0.1:80"
    )
    .expect("write test dump");

    let dump = DumpParser::from_file(tmp.path()).expect("parse dump");
    let processes = ProcessAnalyzer::new(&dump).list_processes().expect("process list");
    assert!(!processes.is_empty());

    let detector = MalwareDetector::new(&dump);
    let malware = detector.detect_malware_strings().expect("malware detection");
    assert!(malware.iter().any(|finding| finding.indicator == "mimikatz"));

    let network = NetworkAnalyzer::new(&dump);
    let connections = network.list_connections().expect("connections");
    assert!(!connections.is_empty());

    let creds = CredentialExtractor::new(&dump)
        .extract_hashes()
        .expect("hash extraction");
    assert!(creds.len() <= 1); // synthetic dump may not trigger hashes
}
