//! Enhanced module metadata system for Phase 2
//!
//! Provides advanced module capabilities:
//! - Dependency declaration and resolution
//! - Version compatibility checking
//! - Module tags and categorization
//! - Author and license information

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Module dependency declaration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModuleDependency {
    /// Module path (e.g., "scanner/port_scanner")
    pub module_path: String,
    /// Version requirement (e.g., ">=1.0.0", "^2.0.0")
    pub version_requirement: String,
    /// Whether this dependency is optional
    pub optional: bool,
}

impl ModuleDependency {
    pub fn new(module_path: impl Into<String>, version_requirement: impl Into<String>) -> Self {
        Self {
            module_path: module_path.into(),
            version_requirement: version_requirement.into(),
            optional: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Check if a given version satisfies this dependency requirement
    pub fn satisfies(&self, version: &str) -> Result<bool> {
        // Simple version comparison (can be enhanced with semver crate)
        if self.version_requirement.starts_with(">=") {
            let required = self.version_requirement.trim_start_matches(">=");
            Ok(version >= required)
        } else if self.version_requirement.starts_with("^") {
            // Caret requirement: compatible with version
            let required = self.version_requirement.trim_start_matches("^");
            let required_parts: Vec<&str> = required.split('.').collect();
            let version_parts: Vec<&str> = version.split('.').collect();

            if required_parts.len() != 3 || version_parts.len() != 3 {
                return Err(anyhow!("Invalid version format"));
            }

            // Same major version, minor and patch can be equal or greater
            Ok(version_parts[0] == required_parts[0] && version >= required)
        } else if self.version_requirement.starts_with("=") {
            let required = self.version_requirement.trim_start_matches("=");
            Ok(version == required)
        } else {
            // Exact match by default
            Ok(version == self.version_requirement.as_str())
        }
    }
}

/// Enhanced module metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedModuleMetadata {
    /// Module identifier (e.g., "scanner/port_scanner")
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Version (semver format)
    pub version: String,

    /// Author information
    pub author: String,

    /// License (e.g., "MIT", "Apache-2.0")
    pub license: String,

    /// Module description
    pub description: String,

    /// Module category/type
    pub category: String,

    /// Tags for searchability
    pub tags: Vec<String>,

    /// Module dependencies
    pub dependencies: Vec<ModuleDependency>,

    /// Platforms supported
    pub platforms: Vec<String>,

    /// Whether module requires confirmation before execution
    pub requires_confirmation: bool,

    /// References (URLs to documentation, papers, CVEs, etc.)
    pub references: Vec<String>,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AdvancedModuleMetadata {
    pub fn builder(id: impl Into<String>) -> MetadataBuilder {
        MetadataBuilder::new(id)
    }

    /// Check if this module is compatible with a given platform
    pub fn supports_platform(&self, platform: &str) -> bool {
        self.platforms.contains(&"any".to_string())
            || self.platforms.contains(&platform.to_lowercase())
    }

    /// Check if this module has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }
}

/// Builder for creating module metadata
pub struct MetadataBuilder {
    id: String,
    name: Option<String>,
    version: String,
    author: String,
    license: String,
    description: String,
    category: String,
    tags: Vec<String>,
    dependencies: Vec<ModuleDependency>,
    platforms: Vec<String>,
    requires_confirmation: bool,
    references: Vec<String>,
    metadata: HashMap<String, serde_json::Value>,
}

impl MetadataBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            name: None,
            id: id.clone(),
            version: "1.0.0".to_string(),
            author: "Ferox Team".to_string(),
            license: "MIT".to_string(),
            description: String::new(),
            category: String::new(),
            tags: Vec::new(),
            dependencies: Vec::new(),
            platforms: vec!["any".to_string()],
            requires_confirmation: false,
            references: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = license.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn dependency(mut self, dep: ModuleDependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        if self.platforms.contains(&"any".to_string()) {
            self.platforms.clear();
        }
        self.platforms.push(platform.into());
        self
    }

    pub fn requires_confirmation(mut self, requires: bool) -> Self {
        self.requires_confirmation = requires;
        self
    }

    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.references.push(reference.into());
        self
    }

    pub fn custom_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn build(self) -> AdvancedModuleMetadata {
        AdvancedModuleMetadata {
            name: self.name.unwrap_or_else(|| self.id.clone()),
            id: self.id,
            version: self.version,
            author: self.author,
            license: self.license,
            description: self.description,
            category: self.category,
            tags: self.tags,
            dependencies: self.dependencies,
            platforms: self.platforms,
            requires_confirmation: self.requires_confirmation,
            references: self.references,
            metadata: self.metadata,
        }
    }
}

/// Module dependency resolver
pub struct DependencyResolver {
    available_modules: HashMap<String, String>, // module_path -> version
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            available_modules: HashMap::new(),
        }
    }

    /// Register an available module
    pub fn register_module(&mut self, path: String, version: String) {
        self.available_modules.insert(path, version);
    }

    /// Check if all dependencies are satisfied
    pub fn check_dependencies(&self, metadata: &AdvancedModuleMetadata) -> Result<Vec<String>> {
        let mut missing = Vec::new();

        for dep in &metadata.dependencies {
            if dep.optional {
                continue; // Skip optional dependencies
            }

            match self.available_modules.get(&dep.module_path) {
                Some(version) => {
                    if !dep.satisfies(version)? {
                        missing.push(format!(
                            "{} (requires {}, found {})",
                            dep.module_path, dep.version_requirement, version
                        ));
                    }
                }
                None => {
                    missing.push(format!(
                        "{} (requires {})",
                        dep.module_path, dep.version_requirement
                    ));
                }
            }
        }

        if missing.is_empty() {
            Ok(Vec::new())
        } else {
            Err(anyhow!("Missing dependencies: {}", missing.join(", ")))
        }
    }

    /// Resolve load order based on dependencies (topological sort)
    pub fn resolve_load_order(&self, modules: &[AdvancedModuleMetadata]) -> Result<Vec<String>> {
        // Simple topological sort
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_mark = std::collections::HashSet::new();

        fn visit(
            module_id: &str,
            modules_map: &HashMap<String, &AdvancedModuleMetadata>,
            visited: &mut std::collections::HashSet<String>,
            temp_mark: &mut std::collections::HashSet<String>,
            result: &mut Vec<String>,
        ) -> Result<()> {
            if visited.contains(module_id) {
                return Ok(());
            }

            if temp_mark.contains(module_id) {
                return Err(anyhow!(
                    "Circular dependency detected involving {}",
                    module_id
                ));
            }

            temp_mark.insert(module_id.to_string());

            if let Some(module) = modules_map.get(module_id) {
                for dep in &module.dependencies {
                    visit(&dep.module_path, modules_map, visited, temp_mark, result)?;
                }
            }

            temp_mark.remove(module_id);
            visited.insert(module_id.to_string());
            result.push(module_id.to_string());

            Ok(())
        }

        let modules_map: HashMap<String, &AdvancedModuleMetadata> =
            modules.iter().map(|m| (m.id.clone(), m)).collect();

        for module in modules {
            visit(
                &module.id,
                &modules_map,
                &mut visited,
                &mut temp_mark,
                &mut result,
            )?;
        }

        Ok(result)
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_builder() {
        let metadata = AdvancedModuleMetadata::builder("scanner/port_scanner")
            .name("Port Scanner")
            .version("2.0.0")
            .description("High-performance port scanner")
            .category("scanner")
            .tag("network")
            .tag("scanning")
            .platform("linux")
            .platform("windows")
            .build();

        assert_eq!(metadata.id, "scanner/port_scanner");
        assert_eq!(metadata.name, "Port Scanner");
        assert_eq!(metadata.version, "2.0.0");
        assert_eq!(metadata.tags.len(), 2);
        assert_eq!(metadata.platforms.len(), 2);
    }

    #[test]
    fn test_dependency_version_satisfaction() {
        let dep1 = ModuleDependency::new("test/module", ">=1.0.0");
        assert!(dep1.satisfies("1.0.0").unwrap());
        assert!(dep1.satisfies("1.5.0").unwrap());
        assert!(dep1.satisfies("2.0.0").unwrap());
        assert!(!dep1.satisfies("0.9.0").unwrap());

        let dep2 = ModuleDependency::new("test/module", "^1.0.0");
        assert!(dep2.satisfies("1.0.0").unwrap());
        assert!(dep2.satisfies("1.5.0").unwrap());
        assert!(!dep2.satisfies("2.0.0").unwrap());

        let dep3 = ModuleDependency::new("test/module", "=1.0.0");
        assert!(dep3.satisfies("1.0.0").unwrap());
        assert!(!dep3.satisfies("1.0.1").unwrap());
    }

    #[test]
    fn test_dependency_resolver() {
        let mut resolver = DependencyResolver::new();
        resolver.register_module("core/crypto".to_string(), "1.0.0".to_string());

        let module = AdvancedModuleMetadata::builder("c2/teams_tunnel")
            .dependency(ModuleDependency::new("core/crypto", ">=1.0.0"))
            .build();

        assert!(resolver.check_dependencies(&module).is_ok());
    }

    #[test]
    fn test_missing_dependency() {
        let resolver = DependencyResolver::new();

        let module = AdvancedModuleMetadata::builder("c2/teams_tunnel")
            .dependency(ModuleDependency::new("core/crypto", ">=1.0.0"))
            .build();

        assert!(resolver.check_dependencies(&module).is_err());
    }

    #[test]
    fn test_optional_dependency() {
        let resolver = DependencyResolver::new();

        let module = AdvancedModuleMetadata::builder("test/module")
            .dependency(ModuleDependency::new("optional/dep", ">=1.0.0").optional())
            .build();

        // Optional dependencies don't cause errors
        assert!(resolver.check_dependencies(&module).is_ok());
    }

    #[test]
    fn test_platform_support() {
        let metadata = AdvancedModuleMetadata::builder("test/module")
            .platform("linux")
            .platform("windows")
            .build();

        assert!(metadata.supports_platform("linux"));
        assert!(metadata.supports_platform("windows"));
        assert!(!metadata.supports_platform("macos"));
    }

    #[test]
    fn test_load_order_simple() {
        let resolver = DependencyResolver::new();

        let module_a = AdvancedModuleMetadata::builder("module_a").build();
        let module_b = AdvancedModuleMetadata::builder("module_b")
            .dependency(ModuleDependency::new("module_a", "1.0.0"))
            .build();

        let modules = vec![module_b, module_a];
        let order = resolver.resolve_load_order(&modules).unwrap();

        // module_a should be loaded before module_b
        assert_eq!(order[0], "module_a");
        assert_eq!(order[1], "module_b");
    }

    #[test]
    fn test_circular_dependency_detection() {
        let resolver = DependencyResolver::new();

        let module_a = AdvancedModuleMetadata::builder("module_a")
            .dependency(ModuleDependency::new("module_b", "1.0.0"))
            .build();

        let module_b = AdvancedModuleMetadata::builder("module_b")
            .dependency(ModuleDependency::new("module_a", "1.0.0"))
            .build();

        let modules = vec![module_a, module_b];
        let result = resolver.resolve_load_order(&modules);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Circular"));
    }
}
