use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls, Error};

use crate::config::Config;
use crate::models::Media;
use crate::queryobjects::MediaListQuery;

pub type Db = Arc<Mutex<Client>>;

const GET_MEDIA_BY_ID: &'static str = "
SELECT * FROM media WHERE id = $1
";

const GET_MEDIA: &'static str = "
SELECT * FROM media OFFSET $1 LIMIT $2
";

const COUNT_MEDIA: &'static str = "
SELECT COUNT(*) FROM media WHERE location = $1
";

const INSERT_CONFLICT: &'static str = "
INSERT INTO media (title, artist, album, location)
VALUES ($1, $2, $3, $4)
ON CONFLICT (location) DO UPDATE SET
title = $1,
artist = $2,
album = $3
";

pub async fn init_db(cfg: &Config) -> Result<Db, Error> {
    // connection object performs actual communication with db, so put it in thread
    let (client, connection) = tokio_postgres::Config::new()
        .host("localhost")
        .dbname(cfg.db.name.as_str())
        .user(cfg.db.user.as_str())
        .password(cfg.db.password.as_str())
        .connect(NoTls)
        .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(client)))
}

pub async fn file_in_db(db: &Client, filename: &String) -> Result<bool, Error> {
    db
        .query_one(COUNT_MEDIA, &[filename])
        .await
        .map(|row| row.get::<'_, usize, i64>(0) == 1)
}

pub async fn insert_media(db: &Client, media: &Media) -> Result<bool, Error> {
    db
        .execute(INSERT_CONFLICT,
        &[&media.title, &media.artist, &media.album, &media.location])
        .await
        .map(|_| true)
}

pub async fn get_one_media(db: &Client, id: i32) -> Result<Media, Error> {
    db
        .query_one(GET_MEDIA_BY_ID, &[&id])
        .await
        .map(|row| Media::from_row(&row))
}

pub async fn get_media(db: &Client, q: MediaListQuery) -> Result<Vec<Media>, Error> {
    db
        .query(GET_MEDIA, &[&q.offset, &q.limit])
        .await
        .map(Media::from_rows)
}
