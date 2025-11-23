// Ferox Desktop - Professional C2 Operations Console
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ferox_desktop_lib::run()
}
