#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::path::Path;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_audio_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_audio_list(path: &Path) -> Vec<(String, f32)> {
    fs::read_dir(path).unwrap().filter_map(|entry| {
        let entry = entry.ok()?;
        if entry.file_type().ok()?.is_file() {
            Some((entry.file_name().to_string_lossy().into_owned(), 35.3))
        } else {
            None
        }
    })
    .collect()
}
