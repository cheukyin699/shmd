use std::fs;
use std::path::PathBuf;

use tokio_postgres::Client;

use crate::db;
use crate::models::Media;

const AUDIO_EXTS: [&str; 7] = [
    "mp3", "m4a", "ogg", "oga", "opus", "wav", "webm",
];

/**
 * Check that a directory entry is an audio file.
 */
fn is_audio_file(p: &fs::DirEntry) -> bool {
    if let Some(ext) = p.path().extension() {
        if let Some(s) = ext.to_str() {
            AUDIO_EXTS.contains(&s)
        } else {
            false
        }
    } else {
        false
    }
}

/**
 * Collect a list of audio files given a root folder.
 *
 * Use DFS to check folder for files.
 */
fn collect_audio_files(root_folder: &String) -> Vec<PathBuf> {
    let mut paths = vec![];
    let mut to_check = vec![PathBuf::from(root_folder)];

    // DFS to try and decrease vector resizing (as opposed to BFS)
    while to_check.len() > 0 {
        let x = to_check.pop().unwrap();
        if let Ok(rd) = fs::read_dir(x.as_path()) {
            for entry in rd {
                if let Ok(entry) = entry {
                    let p = entry.path();
                    if p.is_dir() {
                        to_check.push(p);
                    } else if p.is_file() && is_audio_file(&entry) {
                        paths.push(p);
                    }
                }
            }
        }
    }

    paths
}

/**
 * Scan files.
 *
 * Add audio file metadata into database if they don't exist there. Remove rows from database if we
 * see that the file doesn't exist on the filesystem.
 */
pub async fn scan_files(client: &Client, root_folder: &String) {
    let files = collect_audio_files(root_folder);

    // Files may need updating, so insert (and update) all of them
    for f in &files {
        match Media::from_path(root_folder, f) {
            Ok(m) => {
                if let Err(e) = db::insert_media(client, &m).await {
                    error!("{}", e);
                }
            },
            Err(e) => {
                error!("{}", e);
            }
        }
    }

    // Look for files in database that aren't in the root folder
}
