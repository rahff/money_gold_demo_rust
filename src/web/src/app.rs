use crate::router::add_transfer_gold_capacity;

pub async fn app() -> () {
    let router = add_transfer_gold_capacity().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}


