use std::convert::Infallible;

use tokio_postgres::Row;
use warp::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::scanner;
use crate::models::Media;
use crate::config::Config;
use crate::queryobjects::MediaListQuery;

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

pub async fn get_media_thumbnail(id: i32, db: Db, cfg: Config) -> Result<impl warp::Reply, Infallible> {
    let client = db.lock().await;

    match crate::db::get_one_media(&client, id).await {
        Ok(media) => {
            if let Some(thumbnail) = media.thumbnail(&cfg.music.path) {
                Ok(warp::http::Response::builder()
                    .header("Content-Type", "image")
                    .body(thumbnail))
            } else {
                Ok(warp::http::Response::builder()
                    .status(404)
                    .body(vec![]))
            }
        },
        Err(e) => {
            error!("{}", e);
            Ok(warp::http::Response::builder()
                .status(404)
                .body(vec![]))
        }
    }
}
