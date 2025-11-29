//! Progress Bar Implementation
//!
//! Provides customizable progress bars with:
//! - Percentage display
//! - ETA calculation
//! - Custom styling
//! - Live discovery display

use colored::Colorize;
use std::io::{self, Write};
use std::time::{Duration, Instant};

/// Progress bar style configuration
#[derive(Debug, Clone)]
pub struct ProgressStyle {
    /// Character for filled portion
    pub filled: char,
    /// Character for empty portion
    pub empty: char,
    /// Width of the bar in characters
    pub width: usize,
    /// Show percentage
    pub show_percent: bool,
    /// Show count (current/total)
    pub show_count: bool,
    /// Show ETA
    pub show_eta: bool,
    /// Show elapsed time
    pub show_elapsed: bool,
    /// Prefix text
    pub prefix: Option<String>,
    /// Color for filled portion (green, blue, yellow, etc.)
    pub color: Option<String>,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        Self {
            filled: '█',
            empty: '░',
            width: 40,
            show_percent: true,
            show_count: true,
            show_eta: true,
            show_elapsed: false,
            prefix: None,
            color: Some("green".to_string()),
        }
    }
}

impl ProgressStyle {
    pub fn minimal() -> Self {
        Self {
            width: 20,
            show_count: false,
            show_eta: false,
            ..Default::default()
        }
    }

    pub fn detailed() -> Self {
        Self {
            width: 40,
            show_percent: true,
            show_count: true,
            show_eta: true,
            show_elapsed: true,
            ..Default::default()
        }
    }

    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.prefix = Some(prefix.to_string());
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }
}

/// Progress bar for tracking operation progress
pub struct ProgressBar {
    /// Total items to process
    total: u64,
    /// Current progress
    current: u64,
    /// Style configuration
    style: ProgressStyle,
    /// Start time for ETA calculation
    start_time: Instant,
    /// Current message
    message: String,
    /// Recent discoveries to display
    discoveries: Vec<String>,
    /// Maximum discoveries to show
    max_discoveries: usize,
    /// Whether the bar has been drawn
    drawn: bool,
    /// Lines used by discoveries
    discovery_lines: usize,
}

impl ProgressBar {
    pub fn new(total: u64) -> Self {
        Self {
            total,
            current: 0,
            style: ProgressStyle::default(),
            start_time: Instant::now(),
            message: String::new(),
            discoveries: Vec::new(),
            max_discoveries: 5,
            drawn: false,
            discovery_lines: 0,
        }
    }

    pub fn with_style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    /// Set current progress
    pub fn set(&mut self, current: u64) {
        self.current = current.min(self.total);
        self.draw();
    }

    /// Increment progress by 1
    pub fn inc(&mut self) {
        self.set(self.current + 1);
    }

    /// Increment progress by amount
    pub fn inc_by(&mut self, amount: u64) {
        self.set(self.current + amount);
    }

    /// Update message
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
        self.draw();
    }

    /// Add a discovery to display
    pub fn add_discovery(&mut self, discovery: &str) {
        self.discoveries.push(discovery.to_string());
        if self.discoveries.len() > self.max_discoveries {
            self.discoveries.remove(0);
        }
        self.draw();
    }

    /// Calculate ETA based on current progress
    fn calculate_eta(&self) -> Option<Duration> {
        if self.current == 0 {
            return None;
        }

        let elapsed = self.start_time.elapsed();
        let rate = self.current as f64 / elapsed.as_secs_f64();

        if rate == 0.0 {
            return None;
        }

        let remaining = self.total - self.current;
        let eta_secs = remaining as f64 / rate;

        Some(Duration::from_secs_f64(eta_secs))
    }

    /// Format duration for display
    fn format_duration(d: Duration) -> String {
        let secs = d.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else {
            format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
        }
    }

    /// Draw the progress bar
    pub fn draw(&mut self) {
        // Clear previous output if needed
        if self.drawn {
            // Move up and clear lines
            let lines_to_clear = 1 + self.discovery_lines;
            for _ in 0..lines_to_clear {
                print!("\x1b[1A\x1b[2K");
            }
        }

        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64 * 100.0) as u64
        } else {
            0
        };

        let filled_width = if self.total > 0 {
            (self.current as f64 / self.total as f64 * self.style.width as f64) as usize
        } else {
            0
        };

        let empty_width = self.style.width - filled_width;

        // Build the bar
        let filled: String = std::iter::repeat(self.style.filled).take(filled_width).collect();
        let empty: String = std::iter::repeat(self.style.empty).take(empty_width).collect();

        let bar = if let Some(ref color) = self.style.color {
            match color.as_str() {
                "green" => format!("{}{}", filled.green(), empty.dimmed()),
                "blue" => format!("{}{}", filled.blue(), empty.dimmed()),
                "yellow" => format!("{}{}", filled.yellow(), empty.dimmed()),
                "red" => format!("{}{}", filled.red(), empty.dimmed()),
                "cyan" => format!("{}{}", filled.cyan(), empty.dimmed()),
                _ => format!("{}{}", filled, empty),
            }
        } else {
            format!("{}{}", filled, empty)
        };

        // Build info string
        let mut info_parts = Vec::new();

        if self.style.show_percent {
            info_parts.push(format!("{}%", percentage));
        }

        if self.style.show_count {
            info_parts.push(format!("{}/{}", self.current, self.total));
        }

        if self.style.show_eta {
            if let Some(eta) = self.calculate_eta() {
                if self.current < self.total {
                    info_parts.push(format!("ETA: {}", Self::format_duration(eta)));
                }
            }
        }

        if self.style.show_elapsed {
            info_parts.push(format!("elapsed: {}", Self::format_duration(self.start_time.elapsed())));
        }

        let info = info_parts.join(" │ ");

        // Print the bar
        if let Some(ref prefix) = self.style.prefix {
            print!("{} ", prefix.dimmed());
        }

        print!("{} ", bar);

        if !info.is_empty() {
            print!("{}", info.dimmed());
        }

        if !self.message.is_empty() {
            print!(" │ {}", self.message);
        }

        println!();

        // Print discoveries
        self.discovery_lines = 0;
        for (i, discovery) in self.discoveries.iter().enumerate() {
            let prefix = if i == self.discoveries.len() - 1 {
                "  └─"
            } else {
                "  ├─"
            };
            println!("{} {}", prefix.dimmed(), discovery.green());
            self.discovery_lines += 1;
        }

        let _ = io::stdout().flush();
        self.drawn = true;
    }

    /// Finish the progress bar
    pub fn finish(&mut self) {
        self.current = self.total;
        self.draw();
    }

    /// Finish with a custom message
    pub fn finish_with_message(&mut self, message: &str) {
        self.message = message.to_string();
        self.finish();
    }

    /// Clear the progress bar from screen
    pub fn clear(&self) {
        if self.drawn {
            let lines_to_clear = 1 + self.discovery_lines;
            for _ in 0..lines_to_clear {
                print!("\x1b[1A\x1b[2K");
            }
            let _ = io::stdout().flush();
        }
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        // Ensure final state is shown
        if self.drawn && self.current < self.total {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_creation() {
        let bar = ProgressBar::new(100);
        assert_eq!(bar.total, 100);
        assert_eq!(bar.current, 0);
    }

    #[test]
    fn test_progress_style() {
        let style = ProgressStyle::default();
        assert_eq!(style.width, 40);
        assert!(style.show_percent);
    }

    #[test]
    fn test_eta_calculation() {
        let mut bar = ProgressBar::new(100);
        bar.current = 50;
        // ETA should be calculable
        let eta = bar.calculate_eta();
        assert!(eta.is_some() || bar.start_time.elapsed().as_secs() == 0);
    }
}
