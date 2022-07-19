mod db;
mod config;
mod routes;
mod handlers;
mod queryobjects;

#[tokio::main]
async fn main() {
    let cfg = config::Config::new();
    let db = db::init_db(&cfg).await.unwrap();
    let media_routes = routes::media_routes(db);

    warp::serve(media_routes)
        .run(([127, 0, 0, 1], 3030))
        .await
}
