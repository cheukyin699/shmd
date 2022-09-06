use std::convert::Infallible;

use tokio_postgres::Row;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::scanner;
use crate::models::Media;
use crate::config::Config;
use crate::queryobjects::{AlbumThumbnailQuery, MediaListQuery};

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

    match crate::db::get_media(&client, query).await {
        Ok(media) => Ok(warp::reply::json(&ListMediaResponse::positive_response(media))),
        Err(e) => Ok(warp::reply::json(&ListMediaResponse::negative_response(e.to_string()))),
    }
}

pub async fn scan_media(db: Db, cfg: Config) -> Result<impl warp::Reply, Infallible> {
    tokio::spawn(async move {
        let client = db.lock().await;
        scanner::scan_files(&client, &cfg.music.path).await;
    });

    Ok(StatusCode::OK)
}

pub async fn get_media(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;

    match crate::db::get_one_media(&client, id).await {
        Ok(media) => Ok(warp::reply::json(&media)),
        Err(e) => {
            error!("{}", e);
            Ok(warp::reply::json(&json!({
                "error": format!("{}", e),
            })))
        },
    }
}

pub async fn get_album_thumbnail(db: Db, cfg: Config, query: AlbumThumbnailQuery) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;
    let query = MediaListQuery {
        artist: None,
        album: Some(query.album),
        keyword: None,
        offset: 0,
        limit: 10,
    };

    match crate::db::get_media(&client, query).await {
        Ok(medias) => {
            for media in medias {
                if let Some(thumbnail) = media.thumbnail(&cfg.music.path) {
                    return Ok(warp::http::Response::builder()
                        .header("Content-Type", "image")
                        .body(thumbnail));
                }
            }
        },
        Err(e) => {
            error!("{}", e);
        },
    }

    Ok(warp::http::Response::builder()
        .status(404)
        .body(vec![]))
}

pub async fn get_status(db: Db) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;

    match crate::db::count_media(&client, MediaListQuery::empty()).await {
        Ok(total) => {
            Ok(warp::reply::json(&json!({
                "total": total,
            })))
        },
        Err(e) => {
            error!("{}", e);
            Ok(warp::reply::json(&json!({
                "error": format!("{}", e),
            })))
        },
    }
}

pub async fn count_media(db: Db, query: MediaListQuery) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;

    match crate::db::count_media(&client, query).await {
        Ok(total) => {
            Ok(warp::reply::json(&json!({
                "total": total,
            })))
        },
        Err(e) => {
            error!("{}", e);
            Ok(warp::reply::json(&json!({
                "error": format!("{}", e),
            })))
        },
    }
}
