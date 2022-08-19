use std::path::{Path, PathBuf};

use id3::{Tag, TagLike};
use tokio_postgres::Row;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Media {
    pub id: Option<i32>,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub location: String,
    pub genreid: Option<i32>,
}

const ID3_EXTS: [&str; 3] = [
    "mp3", "wav", "aiff",
];

impl Media {
    fn from_id3(root_folder: &String, p: &Path) -> Result<Self, String> {
        // Strip root folder
        let location = p.display().to_string().replacen(root_folder, "", 1);

        match Tag::read_from_path(p) {
            Ok(tags) => {
                let title = tags.title().unwrap_or("No title").to_string();
                let album = tags.album().map(|x| x.to_string());
                let artist = tags.artist().map(|x| x.to_string());

                Ok(Media {
                    id: None,
                    title,
                    artist,
                    album,
                    location,
                    genreid: None,
                })
            },
            Err(e) => {
                eprintln!("{}", e);
                Err(e.to_string())
            },
        }
    }

    /**
     * Read file metadata to create structure.
     */
    pub fn from_path(root_folder: &String, p: &Path) -> Result<Self, String> {
        if let Some(ext) = p.extension() {
            if let Some(ext) = ext.to_str() {
                if ID3_EXTS.contains(&ext) {
                    Media::from_id3(root_folder, p)
                } else {
                    let err = format!("Unsupported extension '{}'", ext);
                    Err(err)
                }
            } else {
                let err = String::from("Could not convert OsStr to &str");
                Err(err)
            }
        } else {
            let err = String::from("No extensions found");
            Err(err)
        }
    }

    pub fn from_row(row: &Row) -> Self {
        Media {
            id: Some(row.get("id")),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            location: row.get("location"),
            genreid: row.get("genreid"),
        }
    }

    pub fn from_rows(rows: Vec<Row>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Media::from_row(&row))
            .collect()
    }

    fn id3_thumbnail(&self, loc: &PathBuf) -> Option<Vec<u8>> {
        match Tag::read_from_path(loc) {
            Ok(tags) => {
                tags.pictures().next().map(|p| p.data.clone())
            },
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }

    pub fn thumbnail(&self, root_folder: &String) -> Option<Vec<u8>> {
        let abs_location = Path::new(root_folder).join(&self.location);
        if let Some(ext) = abs_location.extension() {
            if let Some(ext) = ext.to_str() {
                if ID3_EXTS.contains(&ext) {
                    self.id3_thumbnail(&abs_location)
                } else {
                    error!("Unsupported extension '{}'", ext);
                    None
                }
            } else {
                error!("Could not convert OSStr to &str");
                None
            }
        } else {
            error!("No extensions found");
            None
        }
    }
}
