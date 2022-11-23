use warp::{self, Filter, reject};

use crate::db::PgPool;
use crate::handlers;
use crate::config::Config;
use crate::queryobjects::{AlbumThumbnailQuery, MediaListQuery};

pub fn media_routes(
    pool: PgPool,
    cfg: Config,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let pg = warp::any()
        .and_then(move || {
            let pool = pool.clone();
            async move {
                match pool.get() {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(reject::not_found()),
                }
            }
        });
    let music_path = cfg.music.path.clone();
    let cfg = warp::any().map(move || cfg.clone());

    warp::path("fs")
        .and(warp::fs::dir(music_path))
        .or(warp::path("media")
            .and(warp::get())
            .and(pg.clone())
            .and(warp::query::<MediaListQuery>())
            .and_then(handlers::list_media))
        .or(warp::path("scan")
            .and(warp::get())
            .and(pg.clone())
            .and(cfg.clone())
            .and_then(handlers::scan_media))
        .or(warp::path("mediacount")
            .and(warp::get())
            .and(pg.clone())
            .and(warp::query::<MediaListQuery>())
            .and_then(handlers::count_media))
        .or(warp::path("media")
            .and(warp::get())
            .and(pg.clone())
            .and(warp::path::param::<i32>())
            .and_then(handlers::get_media))
        .or(warp::path("thumbnail")
            .and(warp::get())
            .and(pg.clone())
            .and(cfg.clone())
            .and(warp::query::<AlbumThumbnailQuery>())
            .and_then(handlers::get_album_thumbnail))
        .or(warp::path("status")
            .and(warp::get())
            .and(pg.clone())
            .and_then(handlers::get_status))
}
