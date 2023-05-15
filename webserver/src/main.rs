use axum::{ Router};
use axum::routing::get_service;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let www = env!("CARGO_MANIFEST_DIR").replace("/webserver", "/www");
    // build our application with a single route
    let app = Router::new()
        .nest_service("/", get_service(ServeDir::new(www)));

    // run it with hyper on localhost:8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}