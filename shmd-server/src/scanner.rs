use std::fs;
use std::path::{Path, PathBuf};
use std::collections::VecDeque;

const AUDIO_EXTS: [&'static str] = [
    "mp3", "m4a", "ogg", "oga", "opus", "wav", "webm",
];

async fn is_audio_file(p: &fs::DirEntry) -> bool {
    if let Some(ext) = p.path().extension() {
        AUDIO_EXTS.contains(ext)
    } else {
        false
    }
}

async fn collect_audio_files(root_folder: String) -> Vec<Path> {
    let mut paths = vec![];
    let mut to_check = vec![PathBuf::from(root_folder)];

    // DFS to try and decrease vector resizing (as opposed to BFS)
    while to_check.len() > 0 {
        let x = to_check.pop().unwrap();
        for entry in fs::read_dir(x.path()) {
            if entry.is_dir() {
                to_check.push(entry.path());
            } else if entry.is_file() && is_audio_file(&entry) {
                paths.push(entry.path());
            }
        }
    }

    paths
}

pub async fn scan_files(root_folder: String) {
    // Look for files in root folder that aren't in the database (or need updating in the database)
    // Look for files in database that aren't in the root folder
}
