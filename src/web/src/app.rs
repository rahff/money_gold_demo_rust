use axum::Router;
use banking_demo_application::account_repository::InMemoryAccountTable;
use banking_demo_application::id_provider::UUIDGenerator;

use crate::router::gold_transfer_router;
use crate::transfer_gold_module::TransferGoldModule;

pub async fn app() -> () {
    let router = Router::new().merge(init_transfer_gold_capacity());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
// Question: How to swap database implementation without causing compilation error due
// to unknown generic parameter (from trait AccountRepository and IdGenerator) ?
// Response: Having many a gold_transfer_router implementation for each database adapter implementation
// using pattern matching on environment variable.
fn init_transfer_gold_capacity() -> Router {
    let repository = InMemoryAccountTable::new();
    let transfert_gold_use_case = TransferGoldModule::new(repository, UUIDGenerator::new()).transfer_gold;
    Router::new().merge(gold_transfer_router(transfert_gold_use_case))
}