//! Ferox Desktop - Professional C2 Operations Console
//!
//! A Tauri-based desktop application for red team operations,
//! inspired by Cobalt Strike but built with modern Rust + React.

pub mod bridge;
pub mod commands;
pub mod security;
pub mod session;
pub mod terminal;

use bridge::FeroxBridge;
use commands::{module_commands, payload_commands, session_commands, simulation_commands, terminal_commands};
use security::AuditLogger;
use terminal::TerminalManager;

use std::sync::Arc;
use parking_lot::RwLock;
use tauri::{Manager, RunEvent};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Application state shared across all commands
pub struct AppState {
    pub bridge: Arc<FeroxBridge>,
    pub terminal_manager: TerminalManager,
    pub audit_logger: Arc<AuditLogger>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            bridge: Arc::new(FeroxBridge::new()),
            terminal_manager: TerminalManager::new(),
            audit_logger: Arc::new(AuditLogger::new()),
        }
    }

    /// Create with database persistence
    pub fn with_persistence(db_path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            bridge: Arc::new(FeroxBridge::with_persistence(db_path)?),
            terminal_manager: TerminalManager::new(),
            audit_logger: Arc::new(AuditLogger::new()),
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize tracing for logging
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ferox_desktop=debug,tauri=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Main entry point for the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_tracing();

    tracing::info!("Starting Ferox Desktop v1.0.0");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // Initialize application state with optional persistence
            let db_path = FeroxBridge::default_db_path();
            let state = match AppState::with_persistence(db_path.to_str().unwrap_or("sessions.db")) {
                Ok(s) => {
                    tracing::info!("Initialized with database persistence at {:?}", db_path);
                    s
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize persistence: {}, using in-memory storage", e);
                    AppState::new()
                }
            };

            let state = Arc::new(RwLock::new(state));
            app.manage(state.clone());

            // Initialize the bridge and start background sync
            let app_handle = app.handle().clone();
            let bridge = state.read().bridge.clone();

            tauri::async_runtime::spawn(async move {
                // Initialize bridge (load from database)
                if let Err(e) = bridge.initialize().await {
                    tracing::error!("Failed to initialize Ferox Bridge: {}", e);
                }

                // Start background sync (every 5 seconds)
                bridge.start_sync(app_handle, 5).await;
            });

            // Set up system tray
            #[cfg(desktop)]
            {
                let handle = app.handle().clone();
                setup_tray(&handle)?;
            }

            tracing::info!("Ferox Desktop initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Session commands
            session_commands::get_sessions,
            session_commands::get_session,
            session_commands::create_session,
            session_commands::terminate_session,
            session_commands::update_session_note,
            session_commands::get_session_tree,
            // Terminal commands
            terminal_commands::create_terminal,
            terminal_commands::write_terminal,
            terminal_commands::resize_terminal,
            terminal_commands::close_terminal,
            terminal_commands::get_terminal_history,
            terminal_commands::execute_terminal_command,
            // Module commands
            module_commands::execute_command,
            module_commands::run_privesc,
            module_commands::harvest_credentials,
            module_commands::install_persistence,
            module_commands::lateral_move,
            module_commands::network_discovery,
            // Payload simulation commands
            payload_commands::generate_simulated_payload,
            payload_commands::get_payload_types,
            payload_commands::get_payload_formats,
            // Telemetry simulation commands
            simulation_commands::simulate_network_scan,
            simulation_commands::simulate_credential_dump,
            simulation_commands::simulate_event_log,
            simulation_commands::simulate_scheduled_tasks,
            simulation_commands::simulate_session_notes,
            simulation_commands::simulate_directory_listing,
            simulation_commands::simulate_process_list,
        ])
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application")
        .run(|app_handle, event| {
            match event {
                RunEvent::ExitRequested { .. } => {
                    // Cleanup before exit
                    tracing::info!("Exit requested, cleaning up...");
                }
                RunEvent::WindowEvent { label, event, .. } => {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Hide window instead of closing (system tray behavior)
                        #[cfg(desktop)]
                        {
                            if let Some(window) = app_handle.get_webview_window(&label) {
                                let _ = window.hide();
                                api.prevent_close();
                            }
                        }
                    }
                }
                _ => {}
            }
        });
}

/// Set up the system tray
#[cfg(desktop)]
fn setup_tray(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::{
        menu::{Menu, MenuItem},
        tray::TrayIconBuilder,
    };

    let show = MenuItem::with_id(app, "show", "Show Ferox", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
