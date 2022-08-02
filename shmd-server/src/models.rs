use std::path::Path;

pub struct Media {
    pub id: Option<usize>,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub location: String,
    pub genreid: Option<usize>,
}

impl Media {
    /**
     * Read file metadata to create structure.
     */
    pub fn from_path(p: &Path) -> Self {
        let title = String::from("No title");
        let album = Some(String::from("Giants"));
        let artist = Some(String::from("V O E"));
        let location = p.display().to_string();

        Media {
            id: None,
            title,
            artist,
            album,
            location,
            genreid: None,
        }
    }
}
