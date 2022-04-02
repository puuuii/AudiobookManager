#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_audio_list, record_audio_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_audio_list(path: &Path) -> HashMap<String, f32> {
    let titles: Vec<(String, f32)> = fs::read_dir(path).unwrap().filter_map(|entry| {
        let entry = entry.ok()?;
        if entry.file_type().ok()?.is_file() {
            Some((entry.file_name().to_string_lossy().into_owned(), 0.0))
        } else {
            None
        }
    })
    .collect();

    let mut info: HashMap<String, f32> = titles.into_iter().collect();

    if let Ok(info_text) = fs::read_to_string("./info.txt") {
        let info_fromfile: HashMap<String, f32> = serde_json::from_str(&info_text).unwrap();
        info_fromfile.into_iter().for_each(|(title, time)| {
            match info.get_mut(&title) {
                Some(time_orign) => *time_orign = time,
                None => println!("no {title}"),
            }
        });
    };

    info
}

#[tauri::command]
fn record_audio_list(info: HashMap<String, f32>) {
    let info_text = serde_json::to_string(&info).unwrap();
    let mut file = File::create("./info.txt").unwrap();
    file.write_all(info_text.as_bytes()).unwrap();
}