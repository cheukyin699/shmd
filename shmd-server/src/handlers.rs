use std::convert::Infallible;
use tokio_postgres::Row;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::scanner;
use crate::config::Config;
use crate::queryobjects::MediaListQuery;

#[derive(Deserialize, Serialize)]
struct Media {
    id: i64,
    title: String,
    artist: Option<String>,
    album: Option<String>,
    location: String,
}

impl From<Row> for Media {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            artist: row.get("artist"),
            album: row.get("album"),
            location: row.get("location"),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct ListMediaResponse {
    success: bool,
    error: Option<String>,
    data: Option<Vec<Media>>,
}

impl ListMediaResponse {
    fn positive_response(data: Vec<Media>) -> Self {
        Self {
            success: true,
            error: None,
            data: Some(data),
        }
    }

    fn negative_response(error: String) -> Self {
        Self {
            success: false,
            error: Some(error),
            data: None,
        }
    }
}

pub async fn list_media(db: Db, query: MediaListQuery) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;

    match client.query("SELECT * FROM media OFFSET $1 LIMIT $2", &[&query.offset, &query.limit]).await {
        Ok(rows) => Ok(warp::reply::json(
                &ListMediaResponse::positive_response(
                    rows.into_iter()
                    .map(|row| Media::from(row))
                    .collect()
                )
        )),
        Err(e) => Ok(warp::reply::json(
                &ListMediaResponse::negative_response(e.to_string())
        )),
    }
}

pub async fn scan_media(db: Db, cfg: Config) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;

    scanner::scan_files(&client, &cfg.music.path);

    Ok(StatusCode::OK)
}
