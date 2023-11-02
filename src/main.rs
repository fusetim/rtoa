use axum::{
    routing::get,
    Router,
};

pub mod views;
pub mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/route_selection/:name", get(routes::route_selection));

    // Run the server
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
