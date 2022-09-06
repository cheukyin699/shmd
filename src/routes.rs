use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::config::Config;
use crate::queryobjects::{AlbumThumbnailQuery, MediaListQuery};

pub fn media_routes(
    db: Db,
    cfg: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_media(db.clone())
        .or(count_media(db.clone()))
        .or(get_media(db.clone()))
        .or(get_media_thumbnail(db.clone(), cfg.clone()))
        .or(scan_media(db.clone(), cfg.clone()))
        .or(download_media(cfg.clone()))
}

pub fn status_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_status(db.clone())
}

/**
 * Download media by specifying the path from either listing media or get-ing media.
 */
fn download_media(
    cfg: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("fs")
        .and(warp::fs::dir(cfg.music.path))
}

fn list_media(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("media")
        .and(warp::get())
        .and(with_db(db))
        .and(warp::query::<MediaListQuery>())
        .and_then(handlers::list_media)
}

fn count_media(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("mediacount")
        .and(warp::get())
        .and(with_db(db))
        .and(warp::query::<MediaListQuery>())
        .and_then(handlers::count_media)
}

fn scan_media(
    db: Db,
    cfg: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("scan")
        .and(warp::get())
        .and(with_db(db))
        .and(warp::any().map(move || cfg.clone()))
        .and_then(handlers::scan_media)
}

fn get_media(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("media" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_media)
}

fn get_media_thumbnail(
    db: Db,
    cfg: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("thumbnail")
        .and(warp::get())
        .and(with_db(db))
        .and(warp::any().map(move || cfg.clone()))
        .and(warp::query::<AlbumThumbnailQuery>())
        .and_then(handlers::get_album_thumbnail)
}

fn get_status(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_status)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
