use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

use rand::Rng;
use regex::Regex;
use rfd::FileDialog;

const AUDIO_EXTENSIONS: [&str; 7] = [".wav", ".mp3", ".aac", ".m4a", ".m4r", ".flac", ".ogg"];

fn is_audio_file(file_name: &str) -> bool {
    let path = Path::new(file_name);
    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(|s| s.to_lowercase());

    match extension {
        Some(ext) => AUDIO_EXTENSIONS.contains(&ext.as_str()),
        None => false,
    }
}

fn select_directory() -> Option<String> {
    let dir = FileDialog::new().pick_folder();

    match dir {
        Some(path) => Some(path.into_os_string().into_string().unwrap()),
        None => None,
    }
}

fn main() -> io::Result<()> {
    let dir = match select_directory() {
        Some(dir) => dir,
        None => return Ok(()),
    };

    let re = Regex::new(r"^\d{3}").unwrap();

    let mut used_numbers = HashSet::new();
    let mut rng = rand::thread_rng();
    println!("Processing directory: {}", dir);

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            println!("Skipping directory: {}", file_name);
            continue;
        }

        if is_audio_file(file_name) {
            println!("Skipping non-audio file: {}", file_name);
            continue;
        }

        if !re.is_match(file_name) {
            println!("Skipping file without number: {}", file_name);
            continue;
        }
        println!("Processing file: {}", file_name);

        let mut new_number;
        loop {
            new_number = rng.gen_range(0..1000);
            if !used_numbers.contains(&new_number) {
                used_numbers.insert(new_number);
                break;
            }
        }

        let new_name = format!("{:03}{}", new_number, &file_name[3..]);
        let parent_dir = path.parent().unwrap();
        let new_path = parent_dir.join(new_name.clone());

        fs::rename(&path, &new_path)?;
        println!("Renamed: {} -> {}", file_name, new_name);
    }

    println!("Processing complete.");
    Ok(())
}
