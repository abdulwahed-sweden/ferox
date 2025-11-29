//! Attack Plan
//!
//! Serializable attack plan that can be saved, loaded, and executed

use super::types::*;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A phase in the attack plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    /// Phase number (1-indexed)
    pub number: usize,
    /// Phase name
    pub name: String,
    /// Modules in this phase
    pub modules: Vec<ModuleConfig>,
    /// Whether phase is completed
    #[serde(default)]
    pub completed: bool,
}

/// Module configuration in a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    /// Module path
    pub path: String,
    /// Display name
    pub name: String,
    /// Module options
    pub options: HashMap<String, String>,
    /// Whether module is enabled
    pub enabled: bool,
    /// Execution result (if executed)
    #[serde(default)]
    pub result: Option<ModuleExecutionResult>,
}

/// Complete attack plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPlan {
    /// Plan name/identifier
    pub name: String,
    /// Plan description
    #[serde(default)]
    pub description: String,
    /// Target configuration
    pub target: TargetConfig,
    /// Scope configuration
    pub scope: ScopeConfig,
    /// Phases with modules
    pub phases: Vec<Phase>,
    /// Execution mode
    pub execution_mode: ExecutionMode,
    /// Estimated duration in seconds
    pub estimated_duration_secs: u64,
    /// Creation timestamp
    pub created_at: String,
    /// Last modified timestamp
    #[serde(default)]
    pub modified_at: String,
    /// Plan version for compatibility
    pub version: String,
    /// Whether plan was interrupted
    #[serde(default)]
    pub interrupted: bool,
}

impl AttackPlan {
    /// Create a new attack plan from wizard state
    pub fn from_wizard_state(state: &WizardState) -> Self {
        // Group modules by phase
        let mut phase_map: HashMap<usize, Vec<&SelectedModule>> = HashMap::new();
        for module in &state.modules {
            if module.enabled {
                phase_map.entry(module.phase).or_default().push(module);
            }
        }

        // Create phases
        let mut phases: Vec<Phase> = phase_map
            .into_iter()
            .map(|(phase_num, modules)| {
                let phase_name = match phase_num {
                    1 => "Reconnaissance",
                    2 => "Enumeration",
                    3 => "Post-Exploitation",
                    _ => "Custom",
                };

                Phase {
                    number: phase_num,
                    name: phase_name.to_string(),
                    modules: modules
                        .into_iter()
                        .map(|m| ModuleConfig {
                            path: m.path.clone(),
                            name: m.name.clone(),
                            options: m.options.clone(),
                            enabled: m.enabled,
                            result: None,
                        })
                        .collect(),
                    completed: false,
                }
            })
            .collect();

        // Sort phases by number
        phases.sort_by_key(|p| p.number);

        // Estimate duration (rough: 5 seconds per module)
        let total_modules: usize = phases.iter().map(|p| p.modules.len()).sum();
        let estimated_duration = (total_modules * 5) as u64;

        Self {
            name: if state.plan_name.is_empty() {
                format!("plan_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
            } else {
                state.plan_name.clone()
            },
            description: format!(
                "{} assessment of {}",
                state.scope.level,
                state.target.target
            ),
            target: state.target.clone(),
            scope: state.scope.clone(),
            phases,
            execution_mode: state.execution_mode,
            estimated_duration_secs: estimated_duration,
            created_at: chrono::Utc::now().to_rfc3339(),
            modified_at: chrono::Utc::now().to_rfc3339(),
            version: "1.0".to_string(),
            interrupted: false,
        }
    }

    /// Save plan to YAML file
    pub fn save_yaml<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = serde_yaml::to_string(self)
            .context("Failed to serialize plan to YAML")?;
        std::fs::write(&path, yaml)
            .context(format!("Failed to write plan to {:?}", path.as_ref()))?;
        Ok(())
    }

    /// Save plan to JSON file
    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize plan to JSON")?;
        std::fs::write(&path, json)
            .context(format!("Failed to write plan to {:?}", path.as_ref()))?;
        Ok(())
    }

    /// Load plan from YAML file
    pub fn load_yaml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .context(format!("Failed to read plan from {:?}", path.as_ref()))?;
        let plan: Self = serde_yaml::from_str(&content)
            .context("Failed to parse YAML plan")?;
        Ok(plan)
    }

    /// Load plan from JSON file
    pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .context(format!("Failed to read plan from {:?}", path.as_ref()))?;
        let plan: Self = serde_json::from_str(&content)
            .context("Failed to parse JSON plan")?;
        Ok(plan)
    }

    /// Load plan from file (auto-detect format)
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match extension.to_lowercase().as_str() {
            "yaml" | "yml" => Self::load_yaml(path),
            "json" => Self::load_json(path),
            _ => {
                // Try YAML first, then JSON
                Self::load_yaml(path)
                    .or_else(|_| Self::load_json(path))
                    .context("Failed to load plan (tried YAML and JSON)")
            }
        }
    }

    /// Get total module count
    pub fn total_modules(&self) -> usize {
        self.phases.iter()
            .map(|p| p.modules.iter().filter(|m| m.enabled).count())
            .sum()
    }

    /// Get count of completed modules
    pub fn completed_modules(&self) -> usize {
        self.phases.iter()
            .flat_map(|p| &p.modules)
            .filter(|m| m.enabled && m.result.is_some())
            .count()
    }

    /// Check if plan can be resumed
    pub fn can_resume(&self) -> bool {
        self.interrupted && self.completed_modules() < self.total_modules()
    }

    /// Get next phase to execute
    pub fn next_phase(&self) -> Option<&Phase> {
        self.phases.iter().find(|p| !p.completed)
    }

    /// Mark a phase as completed
    pub fn mark_phase_completed(&mut self, phase_number: usize) {
        if let Some(phase) = self.phases.iter_mut().find(|p| p.number == phase_number) {
            phase.completed = true;
        }
        self.modified_at = chrono::Utc::now().to_rfc3339();
    }

    /// Store module result
    pub fn store_module_result(&mut self, phase_number: usize, module_path: &str, result: ModuleExecutionResult) {
        if let Some(phase) = self.phases.iter_mut().find(|p| p.number == phase_number) {
            if let Some(module) = phase.modules.iter_mut().find(|m| m.path == module_path) {
                module.result = Some(result);
            }
        }
        self.modified_at = chrono::Utc::now().to_rfc3339();
    }

    /// Generate plan summary for display
    pub fn summary(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!("Plan: {}", self.name));
        lines.push(format!("Target: {} ({})", self.target.target, self.target.target_type));
        lines.push(format!("Scope: {} @ {}", self.scope.level, self.scope.intensity));
        lines.push(format!("Phases: {}", self.phases.len()));
        lines.push(format!("Modules: {}", self.total_modules()));
        lines.push(format!("Est. Duration: ~{} seconds", self.estimated_duration_secs));

        if self.interrupted {
            lines.push(format!("Status: INTERRUPTED ({}/{} modules completed)",
                self.completed_modules(), self.total_modules()));
        }

        lines.join("\n")
    }

    /// Get default save path for interrupted plans
    pub fn interrupted_plan_path() -> std::path::PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("ferox")
            .join("interrupted_plan.yaml")
    }

    /// Save as interrupted plan for resume
    pub fn save_interrupted(&mut self) -> Result<()> {
        self.interrupted = true;
        self.modified_at = chrono::Utc::now().to_rfc3339();

        let path = Self::interrupted_plan_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        self.save_yaml(&path)?;
        Ok(())
    }

    /// Load interrupted plan if exists
    pub fn load_interrupted() -> Result<Option<Self>> {
        let path = Self::interrupted_plan_path();
        if path.exists() {
            let plan = Self::load_yaml(&path)?;
            if plan.can_resume() {
                Ok(Some(plan))
            } else {
                // Plan is complete, remove it
                let _ = std::fs::remove_file(&path);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Clear interrupted plan file
    pub fn clear_interrupted() -> Result<()> {
        let path = Self::interrupted_plan_path();
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_creation() {
        let state = WizardState::default();
        let plan = AttackPlan::from_wizard_state(&state);
        assert!(!plan.name.is_empty());
        assert_eq!(plan.version, "1.0");
    }

    #[test]
    fn test_plan_serialization() {
        let mut state = WizardState::default();
        state.target.target = "example.com".to_string();
        state.target.target_type = TargetType::Domain;

        let plan = AttackPlan::from_wizard_state(&state);

        // Test YAML serialization
        let yaml = serde_yaml::to_string(&plan).unwrap();
        assert!(yaml.contains("example.com"));

        // Test JSON serialization
        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("example.com"));
    }
}
