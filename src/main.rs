extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;

use warp::Filter;

mod db;
mod config;
mod routes;
mod models;
mod handlers;
mod scanner;
mod queryobjects;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cfg = config::Config::new();
    let db = db::init_db(&cfg).await.unwrap();
    let all_routes = routes::media_routes(db.clone(), cfg.clone())
        .or(routes::status_routes(db));
    let port = 3030;

    info!("Starting server on port {}", port);

    warp::serve(all_routes)
        .run(([0, 0, 0, 0], port))
        .await
}
