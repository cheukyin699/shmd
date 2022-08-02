use std::path::Path;

use id3::{Tag, TagLike};

pub struct Media {
    pub id: Option<usize>,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub location: String,
    pub genreid: Option<usize>,
}

const ID3_EXTS: [&str; 3] = [
    "mp3", "wav", "aiff",
];

impl Media {
    fn error(err: String, location: String) -> Self {
        Media {
            id: None,
            title: err,
            artist: None,
            album: None,
            location,
            genreid: None,
        }
    }

    fn from_id3(p: &Path) -> Self {
        let location = p.display().to_string();

        match Tag::read_from_path(p) {
            Ok(tags) => {
                let title = tags.title().unwrap_or("No title").to_string();
                let album = tags.album().map(|x| x.to_string());
                let artist = tags.artist().map(|x| x.to_string());

                Media {
                    id: None,
                    title,
                    artist,
                    album,
                    location,
                    genreid: None,
                }
            },
            Err(e) => {
                eprintln!("{}", e);
                Media::error(e.to_string(), location)
            },
        }
    }

    /**
     * Read file metadata to create structure.
     */
    pub fn from_path(p: &Path) -> Self {
        if let Some(ext) = p.extension() {
            if let Some(ext) = ext.to_str() {
                if ID3_EXTS.contains(&ext) {
                    Media::from_id3(p)
                } else {
                    let err = format!("Unsupported extension '{}'", ext);
                    eprintln!("{}", err);
                    Media::error(err, p.display().to_string())
                }
            } else {
                let err = String::from("Could not convert OsStr to &str");
                eprintln!("{}", err);
                Media::error(err, p.display().to_string())
            }
        } else {
            let err = String::from("No extensions found");
            eprintln!("{}", err);
            Media::error(err, p.display().to_string())
        }
    }
}
