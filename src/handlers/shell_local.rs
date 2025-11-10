use anyhow::{Context, Result};
use sysinfo::System;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;

/// Local shell handler for executing commands on the local system
/// This handler manages local process execution with async I/O
#[derive(Debug)]
pub struct LocalShellHandler {
    system: System,
}

impl LocalShellHandler {
    /// Create a new local shell handler
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Execute a command and return the output
    pub async fn execute(&self, command: &str) -> Result<CommandOutput> {
        let (shell, flag) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let mut child = Command::new(shell)
            .arg(flag)
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn command")?;

        let stdout = child.stdout.take().context("Failed to capture stdout")?;
        let stderr = child.stderr.take().context("Failed to capture stderr")?;

        // Read stdout asynchronously
        let stdout_reader = BufReader::new(stdout);
        let mut stdout_lines = stdout_reader.lines();
        let mut stdout_output = String::new();

        while let Some(line) = stdout_lines.next_line().await? {
            stdout_output.push_str(&line);
            stdout_output.push('\n');
        }

        // Read stderr asynchronously
        let stderr_reader = BufReader::new(stderr);
        let mut stderr_lines = stderr_reader.lines();
        let mut stderr_output = String::new();

        while let Some(line) = stderr_lines.next_line().await? {
            stderr_output.push_str(&line);
            stderr_output.push('\n');
        }

        let status = child.wait().await?;

        Ok(CommandOutput {
            stdout: stdout_output,
            stderr: stderr_output,
            exit_code: status.code().unwrap_or(-1),
            success: status.success(),
        })
    }

    /// Execute a command and stream output line by line
    pub async fn execute_streaming<F>(&self, command: &str, mut callback: F) -> Result<i32>
    where
        F: FnMut(String) -> Result<()>,
    {
        let (shell, flag) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let mut child = Command::new(shell)
            .arg(flag)
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn command")?;

        let stdout = child.stdout.take().context("Failed to capture stdout")?;
        let stdout_reader = BufReader::new(stdout);
        let mut stdout_lines = stdout_reader.lines();

        while let Some(line) = stdout_lines.next_line().await? {
            callback(line)?;
        }

        let status = child.wait().await?;
        Ok(status.code().unwrap_or(-1))
    }

    /// Get current working directory
    pub fn get_cwd(&self) -> Result<String> {
        let cwd = std::env::current_dir()
            .context("Failed to get current directory")?;
        Ok(cwd.to_string_lossy().to_string())
    }

    /// Change working directory
    pub fn change_directory(&self, path: &str) -> Result<()> {
        std::env::set_current_dir(path)
            .context("Failed to change directory")?;
        Ok(())
    }

    /// List running processes
    pub fn list_processes(&mut self) -> Vec<ProcessInfo> {
        self.system.refresh_processes();
        self.system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32() as i32,
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            })
            .collect()
    }

    /// Get system information
    pub fn get_system_info(&mut self) -> SystemInfo {
        self.system.refresh_all();

        SystemInfo {
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
            os_name: System::name().unwrap_or_else(|| "unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
            total_memory: self.system.total_memory(),
            used_memory: self.system.used_memory(),
            total_swap: self.system.total_swap(),
            cpu_count: self.system.cpus().len(),
        }
    }

    /// Kill a process by PID
    pub fn kill_process(&self, pid: i32) -> Result<()> {
        let pid = sysinfo::Pid::from(pid as usize);

        #[cfg(unix)]
        {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid;
            kill(Pid::from_raw(pid.as_u32() as i32), Signal::SIGKILL)
                .context("Failed to kill process")?;
        }

        #[cfg(windows)]
        {
            // On Windows, use Command to kill process
            std::process::Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string()])
                .output()
                .context("Failed to kill process")?;
        }

        Ok(())
    }

    /// Get environment variables
    pub fn get_env_vars(&self) -> Vec<(String, String)> {
        std::env::vars().collect()
    }

    /// Set environment variable
    pub fn set_env_var(&self, key: &str, value: &str) {
        unsafe {
            std::env::set_var(key, value);
        }
    }
}

impl Default for LocalShellHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Output from a command execution
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

/// System information
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub cpu_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_command() {
        let handler = LocalShellHandler::new();

        #[cfg(unix)]
        let result = handler.execute("echo 'Hello, Ferox!'").await;

        #[cfg(windows)]
        let result = handler.execute("echo Hello, Ferox!").await;

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.stdout.contains("Hello"));
    }

    #[tokio::test]
    async fn test_get_cwd() {
        let handler = LocalShellHandler::new();
        let cwd = handler.get_cwd();
        assert!(cwd.is_ok());
        assert!(!cwd.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_system_info() {
        let mut handler = LocalShellHandler::new();
        let info = handler.get_system_info();
        assert!(!info.hostname.is_empty());
        assert!(!info.os_name.is_empty());
        assert!(info.cpu_count > 0);
    }
}
