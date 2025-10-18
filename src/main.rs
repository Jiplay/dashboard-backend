mod router;
mod handlers;
mod controllers;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Get routers from different modules
    // let feed_router = router::init_router();
    let services_router = handlers::services::init_services();

    // Combine all routers
    let combined_router = app
        // .merge(feed_router)
        .merge(services_router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, combined_router).await.unwrap();
}