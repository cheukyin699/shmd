use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls, Error};

use crate::config::Config;

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
