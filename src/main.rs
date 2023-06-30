use axum::{routing, Router};
use std::env;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = Router::new().route("/", routing::get(|| async { "Hello, Space!" }));
    // Get the port to listen on from the environment, or default to 8080 if not present.
    let addr = format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or("8080".to_string())
    );

    println!("Listening on http://{}", addr);
    // Run it with hyper on localhost.
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
