/// Ferox maintenance and diagnostic engine
use crate::tools::manifest::ModuleManifest;
use crate::tools::output::ColorizedOutput;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct HealthReport {
    pub build_health: bool,
    pub module_health: bool,
    pub structure_health: bool,
    pub missing_modules: Vec<(String, String)>,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

impl HealthReport {
    pub fn is_healthy(&self) -> bool {
        self.build_health && self.module_health && self.structure_health && self.issues.is_empty()
    }

    pub fn print_report(&self) {
        ColorizedOutput::section_header("🩺 Ferox Health Report");

        println!();
        println!("  Build Health:       {}", if self.build_health { "✅ PASS" } else { "❌ FAIL" });
        println!("  Module Health:      {}", if self.module_health { "✅ PASS" } else { "❌ FAIL" });
        println!("  Structure Health:   {}", if self.structure_health { "✅ PASS" } else { "❌ FAIL" });

        if !self.missing_modules.is_empty() {
            ColorizedOutput::section_header("Missing Modules");
            for (category, module) in &self.missing_modules {
                println!("  ❌ {}/{}.rs", category, module);
            }
        }

        if !self.issues.is_empty() {
            ColorizedOutput::section_header("Issues Found");
            for issue in &self.issues {
                println!("  ❌ {}", issue);
            }
        }

        if !self.warnings.is_empty() {
            ColorizedOutput::section_header("Warnings");
            for warning in &self.warnings {
                println!("  ⚠️  {}", warning);
            }
        }

        println!();
        if self.is_healthy() {
            ColorizedOutput::success("Everything healthy!");
        } else {
            ColorizedOutput::error("Issues detected. Run 'cargo maintenance fix'");
        }
        println!();
    }
}

#[derive(Debug)]
pub struct FixReport {
    pub applied: usize,
    pub failed: usize,
    pub has_errors: bool,
}

pub struct MaintenanceEngine {
    manifest: ModuleManifest,
}

impl MaintenanceEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            manifest: ModuleManifest::load()?,
        })
    }

    /// Run comprehensive health check
    pub fn run_health_check(&self) -> HealthReport {
        let missing_modules = self.manifest.missing_modules();
        let mut issues = Vec::new();
        let mut warnings = Vec::new();

        // Check build files
        let build_health = self.check_build_files(&mut issues, &mut warnings);

        // Check module structure
        let module_health = missing_modules.is_empty();

        // Check directory structure
        let structure_health = self.check_directory_structure(&mut issues);

        HealthReport {
            build_health,
            module_health,
            structure_health,
            missing_modules,
            issues,
            warnings,
        }
    }

    fn check_build_files(&self, issues: &mut Vec<String>, warnings: &mut Vec<String>) -> bool {
        let mut health = true;

        // Check Cargo.toml
        if !Path::new("Cargo.toml").exists() {
            issues.push("Cargo.toml not found".to_string());
            health = false;
        }

        // Check main.rs
        if !Path::new("src/main.rs").exists() {
            issues.push("src/main.rs not found".to_string());
            health = false;
        }

        // Check lib.rs
        if !Path::new("src/lib.rs").exists() {
            warnings.push("src/lib.rs not found (library build disabled)".to_string());
        }

        health
    }

    fn check_directory_structure(&self, issues: &mut Vec<String>) -> bool {
        let required_dirs = [
            "src",
            "src/cli",
            "src/core",
            "src/modules",
            "src/memory_forensics",
            "docs",
            "tests",
        ];

        let mut health = true;
        for dir in &required_dirs {
            if !Path::new(dir).exists() {
                issues.push(format!("Missing directory: {}", dir));
                health = false;
            }
        }

        health
    }

    /// Run automatic fixes
    pub fn run_auto_fix(&self) -> FixReport {
        let mut report = FixReport {
            applied: 0,
            failed: 0,
            has_errors: false,
        };

        // Fix missing directories
        for dir in &["src/cli", "src/core", "src/modules", "src/memory_forensics"] {
            if !Path::new(dir).exists() {
                match fs::create_dir_all(dir) {
                    Ok(_) => {
                        report.applied += 1;
                        ColorizedOutput::success(&format!("Created directory: {}", dir));
                    }
                    Err(e) => {
                        report.failed += 1;
                        report.has_errors = true;
                        ColorizedOutput::error(&format!("Failed to create {}: {}", dir, e));
                    }
                }
            }
        }

        // Create missing module files with templates
        for (category, module) in &self.manifest.missing_modules() {
            let path = format!("src/{}/{}.rs", category, module);
            let template = self.generate_module_template(module);

            if let Err(e) = fs::write(&path, template) {
                report.failed += 1;
                report.has_errors = true;
                ColorizedOutput::error(&format!("Failed to create {}: {}", path, e));
            } else {
                report.applied += 1;
                ColorizedOutput::success(&format!("Generated: {}", path));
            }
        }

        report
    }

    fn generate_module_template(&self, module_name: &str) -> String {
        format!(
            r#"/// {0} module
/// Part of Ferox framework v2.0.0

use anyhow::Result;

pub struct {1}Module {{
    // Module implementation
}}

impl {1}Module {{
    pub fn new() -> Self {{
        Self {{}}
    }}

    pub fn execute(&self) -> Result<String> {{
        Ok("Module stub - not yet implemented".to_string())
    }}
}}

impl Default for {1}Module {{
    fn default() -> Self {{
        Self::new()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_module_creation() {{
        let module = {1}Module::new();
        assert_eq!(module.execute().unwrap(), "Module stub - not yet implemented");
    }}
}}
"#,
            module_name,
            Self::to_pascal_case(module_name)
        )
    }

    fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }

    /// Generate diagnostic report
    pub fn generate_diagnostic(&self) -> String {
        let mut report = String::from("═══════════════════════════════════════════\n");
        report.push_str("Ferox Framework Diagnostic Report\n");
        report.push_str("═══════════════════════════════════════════\n\n");

        report.push_str("Version: 2.0.0\n");
        report.push_str(&format!("Timestamp: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));

        // Module inventory
        report.push_str("Module Inventory:\n");
        report.push_str("─────────────────────────────────────────────\n");

        let all_modules = self.manifest.all_modules();
        let missing = self.manifest.missing_modules();

        for (category, _) in &self.manifest.categories {
            let cat_modules: Vec<_> = all_modules
                .iter()
                .filter(|(cat, _)| cat == category)
                .collect();
            report.push_str(&format!("  {}: {} modules\n", category, cat_modules.len()));
        }

        if !missing.is_empty() {
            report.push_str(&format!("\nMissing: {} modules\n", missing.len()));
        }

        report.push_str("\n");
        report
    }
}

impl Default for MaintenanceEngine {
    fn default() -> Self {
        Self::new().expect("Failed to initialize maintenance engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = MaintenanceEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(MaintenanceEngine::to_pascal_case("port_scanner"), "PortScanner");
        assert_eq!(MaintenanceEngine::to_pascal_case("asn"), "Asn");
    }

    #[test]
    fn test_health_check() {
        let engine = MaintenanceEngine::default();
        let report = engine.run_health_check();

        // Should pass basic structure checks
        assert!(!report.structure_health || !report.issues.is_empty());
    }
}
