use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls, Error};

use crate::config::Config;
use crate::models::Media;

pub type Db = Arc<Mutex<Client>>;

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
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Arc::new(Mutex::new(client)))
}

pub async fn file_in_db(db: &Client, filename: &String) -> Result<bool, Error> {
    db
        .query_one("SELECT COUNT(*) FROM media WHERE location = $1", &[filename])
        .await
        .map(|row| row.get::<'_, usize, i64>(0) == 1)
}

pub async fn insert_media(db: &Client, media: &Media) -> Result<bool, Error> {
    db
        .execute("INSERT INTO media (title, artist, album, location) VALUES ($1, $2, $3, $4)",
        &[&media.title, &media.artist, &media.album, &media.location])
        .await
        .map(|_| true)
}
