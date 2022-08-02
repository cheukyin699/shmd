mod db;
mod config;
mod routes;
mod models;
mod handlers;
mod scanner;
mod queryobjects;

#[tokio::main]
async fn main() {
    let cfg = config::Config::new();
    let db = db::init_db(&cfg).await.unwrap();
    let media_routes = routes::media_routes(db, cfg);
    let port = 3030;

    println!("Starting server on port {}", port);

    warp::serve(media_routes)
        .run(([127, 0, 0, 1], port))
        .await
}
