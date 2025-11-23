use crate::tui::components::c2_panel::render_c2_panel;
use crate::tui::components::doctor_panel::render_doctor_panel;
use crate::tui::components::footer::render_footer;
use crate::tui::components::header::render_header;
use crate::tui::components::memory_snapshot::render_memory_snapshot;
use crate::tui::components::module_arsenal::render_module_arsenal;
use crate::tui::components::recent_activity::render_recent_activity;
use crate::tui::components::system_health::render_system_health;
use crate::tui::theme::{FeroxTheme, ThemeMode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

pub struct FeroxDashboard {
    theme: FeroxTheme,
    snapshot: DashboardSnapshot,
    focus: PanelFocus,
}

impl FeroxDashboard {
    pub fn new() -> Self {
        Self::with_mode(ThemeMode::Predator)
    }

    pub fn with_mode(mode: ThemeMode) -> Self {
        Self {
            theme: FeroxTheme::new(mode),
            snapshot: DashboardSnapshot::demo(),
            focus: PanelFocus::Modules,
        }
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        render_header(
            frame,
            layout[0],
            &self.theme,
            &self.snapshot,
            self.theme.mode,
        );

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(layout[1]);

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(11),
                Constraint::Length(7),
                Constraint::Min(0),
            ])
            .split(body[0]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(11),
                Constraint::Length(7),
                Constraint::Min(0),
            ])
            .split(body[1]);

        render_module_arsenal(
            frame,
            left[0],
            &self.theme,
            &self.snapshot,
            self.focus == PanelFocus::Modules,
        );
        render_doctor_panel(
            frame,
            left[1],
            &self.theme,
            &self.snapshot.doctor,
            self.focus == PanelFocus::Doctor,
        );
        render_recent_activity(
            frame,
            left[2],
            &self.theme,
            &self.snapshot.logs,
            self.focus == PanelFocus::Logs,
        );

        render_system_health(frame, right[0], &self.theme, &self.snapshot.system);
        render_memory_snapshot(frame, right[1], &self.theme, &self.snapshot.memory);
        render_c2_panel(
            frame,
            right[2],
            &self.theme,
            &self.snapshot.c2,
            self.focus == PanelFocus::C2,
        );

        render_footer(
            frame,
            layout[2],
            &self.theme,
            &self.snapshot.c2,
            self.theme.mode,
        );
    }

    pub fn snapshot(&self) -> &DashboardSnapshot {
        &self.snapshot
    }

    pub fn focus_panel(&mut self, focus: PanelFocus) {
        self.focus = focus;
    }

    pub fn refresh(&mut self) {
        self.snapshot.refresh();
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.theme.set_mode(mode);
    }

    pub fn theme_mode(&self) -> ThemeMode {
        self.theme.mode
    }

    pub fn cycle_theme(&mut self) {
        let mut iter = ThemeMode::ALL.iter().copied();
        while let Some(mode) = iter.next() {
            if mode == self.theme.mode {
                let next = iter.next().unwrap_or(ThemeMode::Predator);
                self.set_theme_mode(next);
                return;
            }
        }
        self.set_theme_mode(ThemeMode::Predator);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelFocus {
    Modules,
    Doctor,
    Logs,
    C2,
}

#[derive(Clone, Debug)]
pub struct ModuleRecord {
    pub domain: &'static str,
    pub operational: u16,
    pub highlights: &'static str,
}

#[derive(Clone, Debug)]
pub struct SystemHealth {
    pub cpu_utilization: f64,
    pub memory_usage: f64,
    pub detection_surface: f64,
    pub integrity_score: u8,
    pub threat_level: &'static str,
    pub cpu_history: Vec<u64>,
    pub memory_history: Vec<u64>,
    pub detection_history: Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct MemorySnapshot {
    pub yara_hits: u16,
    pub volatility_profiles: u8,
    pub credential_artifacts: u16,
    pub active_dump_jobs: u8,
}

#[derive(Clone, Debug)]
pub struct DoctorStatus {
    pub status: &'static str,
    pub open_findings: u8,
    pub last_audit: &'static str,
    pub patch_window: &'static str,
}

#[derive(Clone, Debug)]
pub struct C2Telemetry {
    pub active_sessions: u8,
    pub live_beacons: u8,
    pub last_sync: &'static str,
    pub exfiltration_rate_mbps: f32,
    pub opsec_directive: &'static str,
    pub session_id: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Success,
}

#[derive(Clone, Debug)]
pub struct ActivityEntry {
    pub timestamp: &'static str,
    pub level: LogLevel,
    pub message: &'static str,
}

#[derive(Clone, Debug)]
pub struct DashboardSnapshot {
    pub modules: Vec<ModuleRecord>,
    pub system: SystemHealth,
    pub memory: MemorySnapshot,
    pub doctor: DoctorStatus,
    pub logs: Vec<ActivityEntry>,
    pub c2: C2Telemetry,
}

impl DashboardSnapshot {
    pub fn demo() -> Self {
        Self {
            modules: vec![
                ModuleRecord {
                    domain: "Reconnaissance",
                    operational: 6,
                    highlights: "ASN, DNS, WHOIS",
                },
                ModuleRecord {
                    domain: "Scanning",
                    operational: 8,
                    highlights: "Port, HTTP, SMB",
                },
                ModuleRecord {
                    domain: "Exploitation",
                    operational: 4,
                    highlights: "EDR bypass, pivoting",
                },
                ModuleRecord {
                    domain: "Memory Forensics",
                    operational: 8,
                    highlights: "YARA + Volatility",
                },
                ModuleRecord {
                    domain: "Post-Exploitation",
                    operational: 7,
                    highlights: "Persistence, lateral",
                },
                ModuleRecord {
                    domain: "C2 / Evasion",
                    operational: 12,
                    highlights: "Teams Tunnel, Silent Shadow",
                },
                ModuleRecord {
                    domain: "Auxiliary",
                    operational: 7,
                    highlights: "Cloud sync, SMTP",
                },
            ],
            system: SystemHealth {
                cpu_history: vec![26, 28, 30, 34, 32],
                memory_history: vec![55, 56, 58, 57, 58],
                detection_history: vec![82, 84, 86, 88, 89],
                cpu_utilization: 0.32,
                memory_usage: 0.58,
                detection_surface: 0.89,
                integrity_score: 96,
                threat_level: "Amber",
            },
            memory: MemorySnapshot {
                yara_hits: 4,
                volatility_profiles: 3,
                credential_artifacts: 27,
                active_dump_jobs: 2,
            },
            doctor: DoctorStatus {
                status: "Healthy",
                open_findings: 2,
                last_audit: "2025-11-12 20:50 UTC",
                patch_window: "+4 hours",
            },
            logs: vec![
                ActivityEntry {
                    timestamp: "20:58:00",
                    level: LogLevel::Success,
                    message: "Memory recon module completed",
                },
                ActivityEntry {
                    timestamp: "20:57:40",
                    level: LogLevel::Success,
                    message: "Ferox Doctor cleared baseline",
                },
                ActivityEntry {
                    timestamp: "20:57:10",
                    level: LogLevel::Warn,
                    message: "Credential extractor flagged anomalies",
                },
                ActivityEntry {
                    timestamp: "20:56:30",
                    level: LogLevel::Info,
                    message: "Teams Tunnel heartbeat matched",
                },
                ActivityEntry {
                    timestamp: "20:55:55",
                    level: LogLevel::Info,
                    message: "New module manifest synced",
                },
            ],
            c2: C2Telemetry {
                active_sessions: 3,
                live_beacons: 7,
                last_sync: "20:58:05 UTC",
                exfiltration_rate_mbps: 1.4,
                opsec_directive: "Blend with Teams telemetry",
                session_id: "predator-alpha-07",
            },
        }
    }

    pub fn module_total(&self) -> u16 {
        self.modules.iter().map(|m| m.operational).sum()
    }

    pub fn refresh(&mut self) {
        self.system.advance_history();
        if !self.logs.is_empty() {
            self.logs.rotate_left(1);
        }
    }
}

impl SystemHealth {
    fn advance_history(&mut self) {
        rotate(&mut self.cpu_history);
        rotate(&mut self.memory_history);
        rotate(&mut self.detection_history);
        self.recompute_ratios();
    }

    fn recompute_ratios(&mut self) {
        if let Some(value) = self.cpu_history.last() {
            self.cpu_utilization = *value as f64 / 100.0;
        }
        if let Some(value) = self.memory_history.last() {
            self.memory_usage = *value as f64 / 100.0;
        }
        if let Some(value) = self.detection_history.last() {
            self.detection_surface = *value as f64 / 100.0;
        }
    }
}

fn rotate(values: &mut Vec<u64>) {
    if values.len() > 1 {
        values.rotate_left(1);
    }
}
