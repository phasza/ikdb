// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use xlsx_transform::{response::TransformCommandResponse, transform_xlsx_command};

mod logging;
mod xlsx_transform;

#[tauri::command]
async fn transform_xlsx_file(src_path: String, dest_path: String) -> TransformCommandResponse {
    transform_xlsx_command(src_path, dest_path).await
}

fn main() {
    logging::configure_logging().expect("Failed to initialize logger");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![transform_xlsx_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
