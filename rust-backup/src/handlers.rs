use std::convert::Infallible;
use warp::http::StatusCode;

use crate::db::PooledPg;
use crate::scanner;
use crate::config::Config;
use crate::queryobjects::{AlbumThumbnailQuery, MediaListQuery};

pub async fn list_media(mut pool: PooledPg, query: MediaListQuery) -> Result<impl warp::Reply, Infallible> {
    match crate::db::get_media(&mut pool, query).await {
        Ok(media) => Ok(warp::reply::json(&json!({
            "success": true,
            "data": Some(media),
        }))),
        Err(e) => Ok(warp::reply::json(&json!({
            "success": false,
            "error": Some(e.to_string()),
        }))),
    }
}

pub async fn scan_media(mut pool: PooledPg, cfg: Config) -> Result<impl warp::Reply, Infallible> {
    tokio::spawn(async move {
        scanner::scan_files(&mut pool, &cfg.music.path).await;
    });

    Ok(StatusCode::OK)
}

pub async fn count_media(mut pool: PooledPg, query: MediaListQuery) -> Result<impl warp::Reply, Infallible> {
    match crate::db::count_media(&mut pool, query).await {
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

pub async fn get_media(mut pool: PooledPg, id: i32) -> Result<impl warp::Reply, Infallible> {
    match crate::db::get_one_media(&mut pool, id).await {
        Ok(media) => Ok(warp::reply::json(&media)),
        Err(e) => {
            error!("{}", e);
            Ok(warp::reply::json(&json!({
                "error": format!("{}", e),
            })))
        },
    }
}

pub async fn get_album_thumbnail(mut pool: PooledPg, cfg: Config, query: AlbumThumbnailQuery) -> Result<impl warp::Reply, Infallible> {
    let query = MediaListQuery {
        artist: None,
        album: Some(query.album),
        keyword: None,
        offset: 0,
        limit: 10,
    };

    match crate::db::get_media(&mut pool, query).await {
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

pub async fn get_status(mut pool: PooledPg) -> Result<impl warp::Reply, Infallible> {
    match crate::db::count_media(&mut pool, MediaListQuery::empty()).await {
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
