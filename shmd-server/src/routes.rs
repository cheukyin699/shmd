use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::queryobjects::MediaListQuery;

pub fn media_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_media(db.clone())
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

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
