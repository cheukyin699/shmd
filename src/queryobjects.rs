use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MediaListQuery {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub keyword: Option<String>,
    pub offset: i64,
    pub limit: i64,
}
