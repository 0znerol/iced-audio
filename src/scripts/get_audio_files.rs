use std::fs;

pub fn get_audio_files(dir: &str) -> Vec<String> {
    let mut audio_files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".wav") {
                            audio_files.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }
    audio_files
}
