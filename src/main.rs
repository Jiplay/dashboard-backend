mod router;
mod handlers;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    // Get the combined router from router.rs
    let api_router = router::init_router();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, api_router).await.unwrap();
}