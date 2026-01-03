//! Multi-Progress Management
//!
//! Manages multiple concurrent progress bars and spinners

// ProgressBar and ProgressStyle available but not currently used in this module
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};

/// Status of a tracked operation
#[derive(Debug, Clone)]
pub enum OperationStatus {
    Pending,
    Running,
    Completed { success: bool },
}

/// A single tracked operation
#[derive(Debug)]
struct TrackedOperation {
    name: String,
    status: OperationStatus,
    current: u64,
    total: u64,
    message: String,
    discoveries: Vec<String>,
}

/// Multi-progress tracker for concurrent operations
pub struct MultiProgress {
    operations: HashMap<String, TrackedOperation>,
    order: Vec<String>,
    drawn_lines: usize,
}

impl MultiProgress {
    pub fn new() -> Self {
        Self {
            operations: HashMap::new(),
            order: Vec::new(),
            drawn_lines: 0,
        }
    }

    /// Add a new operation to track
    pub fn add(&mut self, id: &str, name: &str, total: u64) {
        let op = TrackedOperation {
            name: name.to_string(),
            status: OperationStatus::Pending,
            current: 0,
            total,
            message: String::new(),
            discoveries: Vec::new(),
        };
        self.operations.insert(id.to_string(), op);
        self.order.push(id.to_string());
    }

    /// Start an operation
    pub fn start(&mut self, id: &str) {
        if let Some(op) = self.operations.get_mut(id) {
            op.status = OperationStatus::Running;
            self.redraw();
        }
    }

    /// Update operation progress
    pub fn update(&mut self, id: &str, current: u64, message: Option<&str>) {
        if let Some(op) = self.operations.get_mut(id) {
            op.current = current;
            if let Some(msg) = message {
                op.message = msg.to_string();
            }
            self.redraw();
        }
    }

    /// Add a discovery to an operation
    pub fn add_discovery(&mut self, id: &str, discovery: &str) {
        if let Some(op) = self.operations.get_mut(id) {
            op.discoveries.push(discovery.to_string());
            if op.discoveries.len() > 3 {
                op.discoveries.remove(0);
            }
            self.redraw();
        }
    }

    /// Complete an operation
    pub fn complete(&mut self, id: &str, success: bool) {
        if let Some(op) = self.operations.get_mut(id) {
            op.status = OperationStatus::Completed { success };
            op.current = op.total;
            self.redraw();
        }
    }

    /// Clear previous output
    fn clear_previous(&self) {
        if self.drawn_lines > 0 {
            for _ in 0..self.drawn_lines {
                print!("\x1b[1A\x1b[2K");
            }
            let _ = io::stdout().flush();
        }
    }

    /// Redraw all progress indicators
    fn redraw(&mut self) {
        self.clear_previous();

        let mut lines = 0;

        for id in &self.order {
            if let Some(op) = self.operations.get(id) {
                // Status icon
                let (icon, color) = match &op.status {
                    OperationStatus::Pending => ("○", "dimmed"),
                    OperationStatus::Running => ("●", "cyan"),
                    OperationStatus::Completed { success: true } => ("✓", "green"),
                    OperationStatus::Completed { success: false } => ("✗", "red"),
                };

                let icon_colored = match color {
                    "dimmed" => icon.dimmed().to_string(),
                    "cyan" => icon.cyan().to_string(),
                    "green" => icon.green().to_string(),
                    "red" => icon.red().to_string(),
                    _ => icon.to_string(),
                };

                // Progress bar for running operations
                if matches!(op.status, OperationStatus::Running) && op.total > 0 {
                    let percentage = (op.current as f64 / op.total as f64 * 100.0) as u64;
                    let bar_width = 20;
                    let filled = (op.current as f64 / op.total as f64 * bar_width as f64) as usize;
                    let empty = bar_width - filled;

                    let bar: String = std::iter::repeat_n('█', filled).collect();
                    let bar_empty: String = std::iter::repeat_n('░', empty).collect();

                    println!(
                        "{} {} {}{} {}% ({}/{}) {}",
                        icon_colored,
                        op.name,
                        bar.green(),
                        bar_empty.dimmed(),
                        percentage,
                        op.current,
                        op.total,
                        op.message.dimmed()
                    );
                } else {
                    let status_text = match &op.status {
                        OperationStatus::Pending => "waiting".dimmed().to_string(),
                        OperationStatus::Running => "running...".cyan().to_string(),
                        OperationStatus::Completed { success: true } => "done".green().to_string(),
                        OperationStatus::Completed { success: false } => "failed".red().to_string(),
                    };

                    println!("{} {} - {}", icon_colored, op.name, status_text);
                }
                lines += 1;

                // Show discoveries for running operations
                for (i, discovery) in op.discoveries.iter().enumerate() {
                    let prefix = if i == op.discoveries.len() - 1 {
                        "    └─"
                    } else {
                        "    ├─"
                    };
                    println!("{} {}", prefix.dimmed(), discovery.green());
                    lines += 1;
                }
            }
        }

        let _ = io::stdout().flush();
        self.drawn_lines = lines;
    }

    /// Finish and show final summary
    pub fn finish(&mut self) {
        self.redraw();
        println!(); // Add spacing after progress
    }

    /// Get summary of completed operations
    pub fn summary(&self) -> (usize, usize) {
        let mut success = 0;
        let mut failed = 0;

        for op in self.operations.values() {
            if let OperationStatus::Completed { success: s } = op.status {
                if s {
                    success += 1;
                } else {
                    failed += 1;
                }
            }
        }

        (success, failed)
    }
}

impl Default for MultiProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Phase progress tracker for wizard execution
pub struct PhaseProgress {
    phase_name: String,
    phase_num: usize,
    total_phases: usize,
    modules: Vec<(String, String)>, // (id, name)
    current_module: usize,
}

impl PhaseProgress {
    pub fn new(phase_name: &str, phase_num: usize, total_phases: usize) -> Self {
        Self {
            phase_name: phase_name.to_string(),
            phase_num,
            total_phases,
            modules: Vec::new(),
            current_module: 0,
        }
    }

    pub fn add_module(&mut self, id: &str, name: &str) {
        self.modules.push((id.to_string(), name.to_string()));
    }

    pub fn print_header(&self) {
        println!();
        println!(
            "{}",
            "━".repeat(80).dimmed()
        );
        println!(
            " 🚀 PHASE {}/{}: {}",
            self.phase_num,
            self.total_phases,
            self.phase_name.to_uppercase().bold()
        );
        println!(
            "{}",
            "━".repeat(80).dimmed()
        );
        println!();
    }

    pub fn module_start(&mut self, module_idx: usize, module_name: &str) {
        self.current_module = module_idx;
        println!(
            " [{}/{}] {} on target",
            module_idx + 1,
            self.modules.len(),
            module_name.bold()
        );
        println!(" {}", "─".repeat(76).dimmed());
    }

    pub fn module_complete(&self, success: bool, summary: &str) {
        if success {
            println!(" {} {}", "✓".green(), summary.green());
        } else {
            println!(" {} {}", "✗".red(), summary.red());
        }
        println!();
    }

    pub fn phase_complete(&self, success_count: usize, total: usize) {
        println!(
            "{}",
            "━".repeat(80).dimmed()
        );
        println!(
            " {} PHASE {} COMPLETE: {}/{} modules succeeded",
            if success_count == total { "✅" } else { "⚠️" },
            self.phase_num,
            success_count,
            total
        );
        println!(
            "{}",
            "━".repeat(80).dimmed()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_progress() {
        let mut mp = MultiProgress::new();
        mp.add("scan1", "Port Scanner", 100);
        mp.add("scan2", "HTTP Scanner", 50);

        mp.start("scan1");
        mp.update("scan1", 50, Some("Scanning..."));
        mp.complete("scan1", true);

        let (success, failed) = mp.summary();
        assert_eq!(success, 1);
        assert_eq!(failed, 0);
    }
}
