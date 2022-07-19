use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MediaListQuery {
    pub offset: i64,
    pub limit: i64,
}
