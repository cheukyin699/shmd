extern crate pretty_env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;

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
    let media_routes = routes::media_routes(db, cfg);
    let port = 3030;

    info!("Starting server on port {}", port);

    warp::serve(media_routes)
        .run(([127, 0, 0, 1], port))
        .await
}
