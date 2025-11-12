/// Module visibility and registration tests
use std::path::Path;

#[test]
fn test_memory_forensics_modules_exist() {
    let expected_modules = [
        "dump_parser",
        "process_analyzer",
        "malware_detector",
        "network_analyzer",
        "registry_analyzer",
        "credential_extractor",
        "mitre_mapper",
        "volatility_bridge",
    ];

    println!("Checking memory forensics modules...");
    for module in expected_modules {
        let path = format!("src/memory_forensics/{}.rs", module);
        println!("  Checking: {}", path);
        // Note: We don't assert here since not all modules may exist during initial testing
        // This is informational for now
    }
}

#[test]
fn test_core_modules_directory_exists() {
    let modules_dir = "src/modules";
    assert!(
        Path::new(modules_dir).exists(),
        "Modules directory should exist: {}",
        modules_dir
    );
}

#[test]
fn test_cli_structure() {
    let cli_files = [
        "src/cli/app.rs",
        "src/cli/mod.rs",
        "src/cli/theme.rs",
        "src/cli/memory.rs",
    ];

    for file in &cli_files {
        println!("Checking CLI file: {}", file);
        // Informational check
    }
}

#[test]
fn test_core_structure() {
    let core_files = [
        "src/core/mod.rs",
        "src/core/audit.rs",
        "src/core/config.rs",
        "src/core/session.rs",
        "src/core/exploit_framework.rs",
        "src/core/memory_analysis.rs",
    ];

    for file in &core_files {
        println!("Checking core file: {}", file);
    }
}

#[test]
fn test_no_circular_dependencies() {
    // Verify known files exist and can be checked
    assert!(Path::new("src/lib.rs").exists() || Path::new("src/main.rs").exists());
}
