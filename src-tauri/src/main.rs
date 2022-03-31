#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![test_play])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn test_play() {
  // Get a output stream handle to the default physical sound device
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  // Load a sound from a file, using a path relative to Cargo.toml
  let file = BufReader::new(File::open("F:/kindletext/audio/世界史.flac").unwrap());
  // Decode that sound file into a source
  let source = Decoder::new(file).unwrap();
  // Play the sound directly on the device
  match stream_handle.play_raw(source.convert_samples()) {
    Ok(_) => {
      // The sound plays in a separate audio thread,
      // so we need to keep the main thread alive while it's playing.
      std::thread::sleep(std::time::Duration::from_secs(5));
    },
    Err(e) => println!("{:?}", e),
  }
}