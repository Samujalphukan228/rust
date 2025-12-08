use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello from Axum" }))
        .route("/aisha", get(|| async { "Hello from Aisha" }))
        .route("/ariana", get(|| async { "Hello from Ariana" }))
        .route("/maaha", get(|| async { "Hello from Maaha" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
