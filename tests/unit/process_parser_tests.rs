use ferox::memory_forensics::{DumpParser, ProcessAnalyzer};
use std::io::Write;

#[test]
fn process_parser_detects_system() {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    writeln!(tmp, "System Process\ntestsvc.exe --flag").unwrap();

    let dump = DumpParser::from_file(tmp.path()).unwrap();
    let processes = ProcessAnalyzer::new(&dump).list_processes().unwrap();
    assert!(!processes.is_empty());
}
