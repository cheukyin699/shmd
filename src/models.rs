use std::path::{Path, PathBuf};

use serde::Serialize;
use id3::{Tag, TagLike};
use diesel::prelude::*;
use crate::schema::media;

#[derive(Queryable, Serialize)]
pub struct Media {
    pub id: i32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub location: String,
}

#[derive(Insertable)]
#[diesel(table_name = media)]
pub struct NewMedia {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub location: String,
}

const ID3_EXTS: [&str; 3] = [
    "mp3", "wav", "aiff",
];

impl NewMedia {
    fn from_id3(root_folder: &String, p: &Path) -> Result<Self, String> {
        // Strip root folder
        let location = p.display().to_string().replacen(root_folder, "", 1);

        match Tag::read_from_path(p) {
            Ok(tags) => {
                let title = tags.title().unwrap_or("No title").to_string().replace("\u{0000}", "");
                let album = tags.album().map(|x| x.to_string().replace("\u{0000}", ""));
                let artist = tags.artist().map(|x| x.to_string().replace("\u{0000}", ""));

                Ok(NewMedia {
                    title,
                    artist,
                    album,
                    location,
                })
            },
            Err(e) => {
                error!("from_id3: {}", e);
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
                    NewMedia::from_id3(root_folder, p)
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

    fn id3_thumbnail(&self, loc: &PathBuf) -> Option<Vec<u8>> {
        match Tag::read_from_path(loc) {
            Ok(tags) => {
                tags.pictures().next().map(|p| p.data.clone())
            },
            Err(e) => {
                error!("id3_thumbnail: {}", e);
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
                    error!("thumbnail: Unsupported extension '{}'", ext);
                    None
                }
            } else {
                error!("thumbnail: Could not convert OSStr to &str");
                None
            }
        } else {
            error!("thumbnail: No extensions found");
            None
        }
    }
}
