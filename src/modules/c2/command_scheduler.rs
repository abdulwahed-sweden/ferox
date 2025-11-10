//! Command Scheduler scaffold (cron-like)
//!
//! Provides a minimal in-memory schedule representation. Real implementation
//! will parse cron expressions and manage timed dispatch.
//!
//! TODO:
//! - Support cron syntax parsing
//! - Persistent storage of scheduled tasks
//! - Jitter / backoff handling

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ScheduledCommand {
    pub id: String,
    pub command: String,
    pub next_run: DateTime<Utc>,
}

#[derive(Default)]
pub struct CommandScheduler {
    tasks: HashMap<String, ScheduledCommand>,
}

impl CommandScheduler {
    pub fn new() -> Self { Self { tasks: HashMap::new() } }

    pub fn schedule_once(&mut self, id: &str, command: &str, run_at: DateTime<Utc>) -> Result<()> {
        let sc = ScheduledCommand { id: id.to_string(), command: command.to_string(), next_run: run_at };
        self.tasks.insert(id.to_string(), sc);
        Ok(())
    }

    pub fn due(&self, now: DateTime<Utc>) -> Vec<ScheduledCommand> {
        self.tasks.values().filter(|t| t.next_run <= now).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_and_due() {
        let mut sched = CommandScheduler::new();
        let now = Utc::now();
        sched.schedule_once("task1", "whoami", now).unwrap();
        let due = sched.due(now);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].command, "whoami");
    }
}
