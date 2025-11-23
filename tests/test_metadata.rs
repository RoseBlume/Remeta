

use utils::{collect_music_files, is_roman_alphabet, SCANFILE_PATH};
use remeta::SongMetadata;
use rand::RandomInt;
use serde_json::Value;
// use serde::Serialize;
// use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;


use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
    sync::{Arc, Mutex},
};
// use tauri::AppHandle;
use serde_json::json;
#[test]
fn test_all_files() {
    use std::fs::File;
    use std::io::Result;
    use remeta::{VideoMetadata, ImageMetadata, SongMetadata};

    // Audio
    let song = SongMetadata::from_file("tests/song.flac").expect("Failed to find file");
    println!("{:?}", song);

    // Video
    let video = VideoMetadata::from_file("tests/movie.mp4").expect("Failed to find file");
    println!("{:?}", video);

    // Images (bitmap or vector)
    let video  = ImageMetadata::from_file("tests/image.png").expect("Failed to find file");
    println!("{:?}", video);
}

#[test]
fn scan_music() -> Result<(), Box<dyn std::error::Error>>{
    fn calculate_chunk_size(file_count: usize, threads: usize) -> usize {
        file_count / threads.max(1)
    }
    let music_files: Vec<PathBuf> = collect_music_files();
    let total_files = music_files.len();
    let total_files_f32 = total_files as f32;

    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let chunk_size = calculate_chunk_size(total_files, threads);

    let chunks: Vec<Vec<PathBuf>> = music_files
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();

    let num_chunks = chunks.len();

    // Shared vector for final output
    let entries = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));

    // Shared counter for progress updates
    let progress_count = Arc::new(Mutex::new(0usize));

    let mut handles = Vec::new();
    let start = Instant::now();
    for chunk in chunks {
        let entries_ref = Arc::clone(&entries);
        let progress_ref = Arc::clone(&progress_count);

        let handle = thread::spawn(move || {
            for music_file in chunk {
                // --- Progress update ---
                {
                    let mut counter = progress_ref.lock().unwrap();
                    *counter += 1;
                    let pct = ((*counter as f32 / total_files_f32) * 100.0).round() as u32;
                }

                // --- Metadata processing ---
                let file_string = music_file.to_str().unwrap_or("File String").to_string();
                let metadata = SongMetadata::from_file(music_file.clone()).unwrap();

                let artist = match metadata.artist {
                    Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone(),
                    _ => "Unknown Artist".to_string(),
                };

                let album = match metadata.album {
                    Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone(),
                    _ => "Unknown Album".to_string(),
                };

                let title = match metadata.title {
                    Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone(),
                    _ => "Unknown Title".to_string(),
                };

                let genre = match metadata.genre {
                    Some(ref n) if is_roman_alphabet(n.to_string()) => n.clone().to_string(),
                    _ => "Unknown Genre".to_string(),
                };

                let duration = metadata.duration_ms.unwrap();

                let covers = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 14, 15, 16, 17];
                let idx = RandomInt::new(0, ((covers.len() - 1) as i32).try_into().unwrap()) as usize;

                let entry = json!({
                    "location": file_string,
                    "artist": artist,
                    "album": album,
                    "title": title,
                    "genre": genre.trim_start_matches(|c: char| !c.is_ascii_alphabetic()),
                    "duration": duration,
                    "cover": format!("covers/{}.avif", covers[idx]),
                });

                entries_ref.lock().unwrap().push(entry);

                // --- CPU throttling (keeps CPU from hitting 100%) ---
                thread::sleep(Duration::from_millis(1));
            }
        });

        handles.push(handle);
    }

    // Wait for all threads
    for h in handles {
        let _ = h.join();
    }
    let end = start.elapsed();
    println!("{:?}", end);

    // Return JSON array
    let final_entries = Arc::try_unwrap(entries)
        .unwrap()
        .into_inner()
        .unwrap();

    let expected_json: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("tests/scan.json")?
    ).expect("failed to read");

    // if expected_json != serde_json::Value::Array(final_entries) {
    //     return Err(io::Error::new(
    //         io::ErrorKind::Other,
    //         "Parsed output does not match tests/scan.json",
    //     ));
    // }
    let file_path = PathBuf::from("tests/output.json");
    let scan_data = serde_json::Value::Array(final_entries);
    let data_string = serde_json::to_string_pretty(&scan_data).expect("Failed to turn to string");
    let mut file = File::create(&file_path)
        .expect("Failed to create scan file");
    file.write_all(data_string.as_bytes())
        .expect("Failed to write scan data");
    // assert_eq!(expected_json, data_string);
    Ok(())
}