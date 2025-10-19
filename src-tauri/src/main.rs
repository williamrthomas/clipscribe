// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_api_key,
            get_api_key,
            validate_api_key,
            analyze_transcript_for_clips,
            generate_clips,
            open_in_file_explorer,
            generate_transcript_from_video,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
