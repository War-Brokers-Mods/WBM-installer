#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod constants;
mod util;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::install::install,
            commands::status::status,
            commands::update::update
        ])
        // you might see a unresolved macro error in the IDE but it's nothing to worry about.
        // rust-analyzer doesn't do well with nested macros.
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
