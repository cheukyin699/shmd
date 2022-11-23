use std::sync::Arc;
use tokio::sync::Mutex;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::prelude::*;
use diesel::result::Error;

use crate::config::Config;
use crate::models::{Media, NewMedia};
use crate::queryobjects::MediaListQuery;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

const GET_MEDIA_BY_ID: &'static str = "
SELECT * FROM media WHERE id = $1
";

const COUNT_MEDIA: &'static str = "
SELECT COUNT(*) FROM media
WHERE
title LIKE '%' || COALESCE($1, '') || '%' AND
artist LIKE '%' || COALESCE($2, '') || '%' AND
album LIKE '%' || COALESCE($3, '') || '%'
";

const COUNT_MEDIA_BY_LOC: &'static str = "
SELECT COUNT(*) FROM media WHERE location = $1
";

pub fn init_db(cfg: &Config) -> PgPool {
    let db_url = cfg.db.url.as_str();

    let mgr = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(mgr).expect("Postgres connection pool could not be created")
}

// pub async fn file_in_db(db: &mut PgConnection, filename: &String) -> Result<bool, Error> {
//     use crate::schema::media::dsl::*;

//     media
//         .filter(location.eq(filename.as_str()))
//         .count()
//         .get_result(db)
//         .map(|c: i64| c == 1)
// }

pub async fn insert_media(db: &mut PgConnection, media: &Vec<NewMedia>) -> Result<bool, Error> {
    use crate::schema::media;

    diesel::insert_into(media::table)
        .values(media)
        .execute(db)
        .map(|s| s == media.len())
}

pub async fn get_one_media(db: &mut PgConnection, id: i32) -> Result<Media, Error> {
    use crate::schema::media::dsl::{id as dsl_id, media as dsl_media};

    dsl_media
        .filter(dsl_id.eq(id))
        .first(db)
}

pub async fn get_media(connection: &mut PgConnection, q: MediaListQuery) -> Result<Vec<Media>, Error> {
    use crate::schema::media;

    let mut query = media::table.into_boxed();
    if let Some(artist) = q.artist {
        query = query.filter(media::artist.like(format!("%{}%", artist)));
    }
    if let Some(album) = q.album {
        query = query.filter(media::album.like(format!("%{}%", album)));
    }
    if let Some(keyword) = q.keyword {
        query = query.filter(media::title.like(format!("%{}%", keyword)));
    }

    query
        .limit(q.limit)
        .offset(q.offset)
        .load::<Media>(connection)
}

pub async fn count_media(connection: &mut PgConnection, q: MediaListQuery) -> Result<i64, Error> {
    use crate::schema::media;

    let mut query = media::table.into_boxed();
    if let Some(artist) = q.artist {
        query = query.filter(media::artist.like(format!("%{}%", artist)));
    }
    if let Some(album) = q.album {
        query = query.filter(media::album.like(format!("%{}%", album)));
    }
    if let Some(keyword) = q.keyword {
        query = query.filter(media::title.like(format!("%{}%", keyword)));
    }

    query
        .count()
        .get_result(connection)
}
