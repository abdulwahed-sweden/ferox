//! Spinner Implementation
//!
//! Animated spinner for operations with unknown duration

use colored::Colorize;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;

/// Spinner animation frames
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const DOTS_FRAMES: &[&str] = &["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"];
const ARROW_FRAMES: &[&str] = &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"];
const BOX_FRAMES: &[&str] = &["◰", "◳", "◲", "◱"];

/// Spinner style presets
#[derive(Debug, Clone, Copy)]
pub enum SpinnerStyle {
    Dots,
    Arrow,
    Box,
    Default,
}

impl SpinnerStyle {
    fn frames(&self) -> &'static [&'static str] {
        match self {
            SpinnerStyle::Dots => DOTS_FRAMES,
            SpinnerStyle::Arrow => ARROW_FRAMES,
            SpinnerStyle::Box => BOX_FRAMES,
            SpinnerStyle::Default => SPINNER_FRAMES,
        }
    }
}

/// Animated spinner for unknown duration operations
pub struct Spinner {
    message: String,
    style: SpinnerStyle,
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    color: String,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            style: SpinnerStyle::Default,
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
            color: "cyan".to_string(),
        }
    }

    pub fn with_style(mut self, style: SpinnerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    /// Start the spinner animation
    pub fn start(&mut self) {
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let frames = self.style.frames();
        let message = self.message.clone();
        let color = self.color.clone();

        self.handle = Some(tokio::spawn(async move {
            let mut frame_idx = 0;

            while running.load(Ordering::SeqCst) {
                let frame = frames[frame_idx % frames.len()];
                let colored_frame = match color.as_str() {
                    "green" => frame.green().to_string(),
                    "blue" => frame.blue().to_string(),
                    "yellow" => frame.yellow().to_string(),
                    "red" => frame.red().to_string(),
                    "cyan" => frame.cyan().to_string(),
                    "magenta" => frame.magenta().to_string(),
                    _ => frame.to_string(),
                };

                print!("\r{} {}", colored_frame, message);
                let _ = io::stdout().flush();

                frame_idx += 1;
                tokio::time::sleep(Duration::from_millis(80)).await;
            }

            // Clear the line when stopped
            print!("\r\x1b[2K");
            let _ = io::stdout().flush();
        }));
    }

    /// Update the spinner message
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    /// Stop the spinner
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            // Give it a moment to clean up
            let _ = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let _ = tokio::time::timeout(Duration::from_millis(200), handle).await;
                })
            });
        }
    }

    /// Stop with a success message
    pub fn success(&mut self, message: &str) {
        self.stop();
        println!("{} {}", "✓".green(), message.green());
    }

    /// Stop with an error message
    pub fn error(&mut self, message: &str) {
        self.stop();
        println!("{} {}", "✗".red(), message.red());
    }

    /// Stop with a warning message
    pub fn warning(&mut self, message: &str) {
        self.stop();
        println!("{} {}", "⚠".yellow(), message.yellow());
    }

    /// Stop with an info message
    pub fn info(&mut self, message: &str) {
        self.stop();
        println!("{} {}", "ℹ".blue(), message);
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Simple synchronous spinner for quick operations
pub struct SyncSpinner {
    frames: &'static [&'static str],
    frame_idx: usize,
    message: String,
}

impl SyncSpinner {
    pub fn new(message: &str) -> Self {
        Self {
            frames: SPINNER_FRAMES,
            frame_idx: 0,
            message: message.to_string(),
        }
    }

    /// Tick the spinner (call in a loop)
    pub fn tick(&mut self) {
        let frame = self.frames[self.frame_idx % self.frames.len()];
        print!("\r{} {}", frame.cyan(), self.message);
        let _ = io::stdout().flush();
        self.frame_idx += 1;
    }

    /// Update message
    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    /// Clear and print success
    pub fn success(&self, message: &str) {
        print!("\r\x1b[2K");
        println!("{} {}", "✓".green(), message.green());
    }

    /// Clear and print error
    pub fn error(&self, message: &str) {
        print!("\r\x1b[2K");
        println!("{} {}", "✗".red(), message.red());
    }

    /// Clear the spinner
    pub fn clear(&self) {
        print!("\r\x1b[2K");
        let _ = io::stdout().flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new("Loading...");
        assert_eq!(spinner.message, "Loading...");
    }

    #[test]
    fn test_sync_spinner() {
        let spinner = SyncSpinner::new("Test");
        assert_eq!(spinner.message, "Test");
    }
}
