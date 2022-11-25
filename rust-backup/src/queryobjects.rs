use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MediaListQuery {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub keyword: Option<String>,
    pub offset: i64,
    pub limit: i64,
}

#[derive(Deserialize, Serialize)]
pub struct AlbumThumbnailQuery {
    pub album: String,
}

impl MediaListQuery {
    pub fn empty() -> Self {
        MediaListQuery {
            artist: None,
            album: None,
            keyword: None,
            offset: 0,
            limit: 0,
        }
    }
}
