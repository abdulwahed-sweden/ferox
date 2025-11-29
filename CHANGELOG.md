# Changelog

All notable changes to Ferox will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.0.0] - 2024-11-29

### Added

- **Mobile App Analysis Modules**
  - `mobile/apk_analyzer` - Android APK security analysis with manifest parsing, permission analysis, secrets detection
  - `mobile/ipa_analyzer` - iOS IPA security analysis with Info.plist analysis, binary protection checks, entitlements extraction
  - `mobile/app_recon` - App store reconnaissance for Android and iOS applications

- **Security Assessment Workflow Wizard**
  - Interactive guided assessment workflow
  - Pre-built templates: Full Penetration Test, Web Application, Network Security, Mobile App, Cloud Security
  - Auto platform detection for mobile files
  - Progress tracking with real-time discoveries

- **Desktop Application Enhancements**
  - Modern Ferox fox logo with theme variants
  - Icon generation for all platforms (macOS, Windows, Linux)
  - Updated favicon and app icons

### Changed

- Updated all dependencies to latest versions
- Improved CLI interface with better progress feedback
- Enhanced module organization with mobile category
- Updated README with v4.0 features and mobile usage examples

### Fixed

- Port scanner timeout issues
- HTTP scanner redirect handling
- DNS enumeration for all record types
- Edition year in Cargo.toml (2024 -> 2021)

### Removed

- Deprecated debug test binaries (`test_modules.rs`, `test_subdomain.rs`)

## [3.1.1] - Previous Release

- Memory forensics engine
- Smart Payload System
- Post-Exploitation Engines
- Ferox Desktop initial release

---

For older releases, see [GitHub Releases](https://github.com/abdulwahed-sweden/ferox/releases).
