use ferox::tools::doctor::{CheckResult, DoctorReport, SystemRequirements};

#[test]
fn default_requirements_match_spec() {
    let reqs = SystemRequirements::default();
    assert_eq!(reqs.min_rust_version, "1.70.0");
    assert_eq!(reqs.required_python_version, "3.8");
    assert!(reqs.min_ram_mb >= 2048);
    assert!(reqs.required_tools.contains(&"git".to_string()));
}

#[test]
fn doctor_report_updates_status() {
    let mut report = DoctorReport::new();
    report.add_result(CheckResult::success("Python", "Detected".into()));
    assert_eq!(
        report.overall_status,
        ferox::tools::doctor::OverallStatus::Healthy
    );

    report.add_result(CheckResult::warning("Disk", "Low space".into(), None));
    assert_eq!(
        report.overall_status,
        ferox::tools::doctor::OverallStatus::Degraded
    );

    report.add_result(CheckResult::error("Cargo", "Missing".into(), None));
    assert_eq!(
        report.overall_status,
        ferox::tools::doctor::OverallStatus::Critical
    );
}

#[test]
fn check_result_serializes() {
    let result = CheckResult::warning("Test", "Warn".into(), Some("Fix".into()));
    let serialized = serde_json::to_string(&result).expect("serialize");
    assert!(serialized.contains("\"Warn\""));
    assert!(serialized.contains("warning"));
}
