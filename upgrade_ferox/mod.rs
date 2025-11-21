// Ferox Module Registry - سجل الوحدات
// Centralized module management for payloads and C2

use colored::Colorize;
use std::collections::HashMap;

pub mod payloads {
    pub mod rev_tcp_fileless;
}

pub mod c2 {
    pub mod teams_tunnel;
    pub mod github_gist_loader;
}

/// نوع الوحدة | Module Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Payload,
    C2,
    Exploit,
    PostExploitation,
    Auxiliary,
}

/// معلومات الوحدة | Module Information
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub module_type: ModuleType,
    pub description: String,
    pub author: String,
    pub version: String,
    pub references: Vec<String>,
    pub platforms: Vec<String>,
}

/// سجل الوحدات | Module Registry
pub struct ModuleRegistry {
    modules: HashMap<String, ModuleInfo>,
}

impl ModuleRegistry {
    /// إنشاء سجل جديد | Create new registry
    pub fn new() -> Self {
        let mut registry = Self {
            modules: HashMap::new(),
        };

        // تسجيل الوحدات الافتراضية | Register default modules
        registry.register_default_modules();
        registry
    }

    /// تسجيل الوحدات الافتراضية | Register default modules
    fn register_default_modules(&mut self) {
        // Payload Modules
        self.register(ModuleInfo {
            name: "payloads/fileless/reverse_tcp".to_string(),
            module_type: ModuleType::Payload,
            description: "Fileless Reverse TCP Shell with AES-256-GCM encryption".to_string(),
            author: "Ferox Team".to_string(),
            version: "1.0.0".to_string(),
            references: vec![
                "https://attack.mitre.org/techniques/T1059/".to_string(),
                "https://attack.mitre.org/techniques/T1055/".to_string(),
            ],
            platforms: vec![
                "Windows".to_string(),
                "Linux".to_string(),
                "macOS".to_string(),
            ],
        });

        // C2 Modules
        self.register(ModuleInfo {
            name: "c2/teams_tunnel".to_string(),
            module_type: ModuleType::C2,
            description: "Microsoft Teams webhook-based C2 channel".to_string(),
            author: "Ferox Team".to_string(),
            version: "1.0.0".to_string(),
            references: vec![
                "https://attack.mitre.org/techniques/T1071/".to_string(),
            ],
            platforms: vec!["Universal".to_string()],
        });

        self.register(ModuleInfo {
            name: "c2/github_gist".to_string(),
            module_type: ModuleType::C2,
            description: "GitHub Gist-based payload delivery and C2".to_string(),
            author: "Ferox Team".to_string(),
            version: "1.0.0".to_string(),
            references: vec![
                "https://attack.mitre.org/techniques/T1102/".to_string(),
            ],
            platforms: vec!["Universal".to_string()],
        });
    }

    /// تسجيل وحدة | Register module
    pub fn register(&mut self, info: ModuleInfo) {
        self.modules.insert(info.name.clone(), info);
    }

    /// إلغاء تسجيل وحدة | Unregister module
    pub fn unregister(&mut self, name: &str) -> Option<ModuleInfo> {
        self.modules.remove(name)
    }

    /// البحث عن وحدة | Find module
    pub fn find(&self, name: &str) -> Option<&ModuleInfo> {
        self.modules.get(name)
    }

    /// البحث حسب النوع | Find by type
    pub fn find_by_type(&self, module_type: ModuleType) -> Vec<&ModuleInfo> {
        self.modules
            .values()
            .filter(|m| m.module_type == module_type)
            .collect()
    }

    /// البحث حسب النظام | Find by platform
    pub fn find_by_platform(&self, platform: &str) -> Vec<&ModuleInfo> {
        self.modules
            .values()
            .filter(|m| {
                m.platforms.iter().any(|p| p.eq_ignore_ascii_case(platform))
                    || m.platforms.iter().any(|p| p.eq_ignore_ascii_case("universal"))
            })
            .collect()
    }

    /// عرض جميع الوحدات | List all modules
    pub fn list_all(&self) {
        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "📦 Ferox Module Registry".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());

        let mut modules_by_type: HashMap<ModuleType, Vec<&ModuleInfo>> = HashMap::new();
        for module in self.modules.values() {
            modules_by_type
                .entry(module.module_type)
                .or_insert_with(Vec::new)
                .push(module);
        }

        for (module_type, modules) in modules_by_type.iter() {
            println!("\n{} {:?} Modules ({} total)", "→".bright_blue(), module_type, modules.len());
            for module in modules {
                println!("  {} {}", "•".bright_white(), module.name.bright_yellow());
                println!("    {}", module.description.bright_black());
            }
        }

        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("  {} modules registered", self.modules.len());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    }

    /// عرض تفاصيل الوحدة | Show module details
    pub fn show_details(&self, name: &str) {
        if let Some(module) = self.find(name) {
            println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
            println!("{}", "📋 Module Details".bright_green().bold());
            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
            println!("\n{} {}", "Name:".bright_white().bold(), module.name.bright_yellow());
            println!("{} {:?}", "Type:".bright_white().bold(), module.module_type);
            println!("{} {}", "Version:".bright_white().bold(), module.version);
            println!("{} {}", "Author:".bright_white().bold(), module.author);
            println!("\n{}", "Description:".bright_white().bold());
            println!("  {}", module.description);
            
            if !module.platforms.is_empty() {
                println!("\n{}", "Supported Platforms:".bright_white().bold());
                for platform in &module.platforms {
                    println!("  {} {}", "•".bright_blue(), platform);
                }
            }

            if !module.references.is_empty() {
                println!("\n{}", "References:".bright_white().bold());
                for reference in &module.references {
                    println!("  {} {}", "•".bright_blue(), reference);
                }
            }

            println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        } else {
            println!("{} Module '{}' not found", "❌".bright_red(), name);
        }
    }

    /// عدد الوحدات | Module count
    pub fn count(&self) -> usize {
        self.modules.len()
    }

    /// عدد الوحدات حسب النوع | Count by type
    pub fn count_by_type(&self, module_type: ModuleType) -> usize {
        self.find_by_type(module_type).len()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// مدير الوحدات العالمي | Global module manager
pub struct ModuleManager {
    registry: ModuleRegistry,
}

impl ModuleManager {
    /// إنشاء مدير جديد | Create new manager
    pub fn new() -> Self {
        Self {
            registry: ModuleRegistry::new(),
        }
    }

    /// الحصول على السجل | Get registry
    pub fn registry(&self) -> &ModuleRegistry {
        &self.registry
    }

    /// الحصول على السجل القابل للتعديل | Get mutable registry
    pub fn registry_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.registry
    }

    /// استخدام وحدة | Use module
    pub fn use_module(&self, name: &str) -> Result<(), String> {
        if let Some(module) = self.registry.find(name) {
            println!("\n{} Using module: {}", "→".bright_blue(), module.name.bright_yellow());
            println!("  {}", module.description.bright_black());
            Ok(())
        } else {
            Err(format!("Module '{}' not found", name))
        }
    }

    /// عرض الإحصائيات | Show statistics
    pub fn show_stats(&self) {
        println!("\n{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("{}", "📊 Module Statistics".bright_green().bold());
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        println!("  {} Total: {}", "•".bright_white(), self.registry.count());
        println!("  {} Payloads: {}", "•".bright_white(), self.registry.count_by_type(ModuleType::Payload));
        println!("  {} C2: {}", "•".bright_white(), self.registry.count_by_type(ModuleType::C2));
        println!("  {} Exploits: {}", "•".bright_white(), self.registry.count_by_type(ModuleType::Exploit));
        println!("  {} Post-Exploitation: {}", "•".bright_white(), self.registry.count_by_type(ModuleType::PostExploitation));
        println!("  {} Auxiliary: {}", "•".bright_white(), self.registry.count_by_type(ModuleType::Auxiliary));
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ModuleRegistry::new();
        assert!(registry.count() > 0);
    }

    #[test]
    fn test_find_module() {
        let registry = ModuleRegistry::new();
        let module = registry.find("payloads/fileless/reverse_tcp");
        assert!(module.is_some());
    }

    #[test]
    fn test_find_by_type() {
        let registry = ModuleRegistry::new();
        let payloads = registry.find_by_type(ModuleType::Payload);
        assert!(!payloads.is_empty());
    }

    #[test]
    fn test_module_manager() {
        let manager = ModuleManager::new();
        manager.show_stats();
        assert!(manager.registry().count() > 0);
    }
}
