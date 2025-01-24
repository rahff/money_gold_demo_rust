use axum::Router;
use banking_demo_application::account_repository::InMemoryAccountTable;
use banking_demo_application::id_provider::UUIDGenerator;

use banking_demo_application::use_case::TransferGold;
use crate::router::gold_transfer_router;
use crate::transfer_gold_module::TransferGoldModule;

pub async fn app() -> () {
    let transfert_gold_use_case = init_transfer_gold_capacity();
    let app = Router::new().merge(gold_transfer_router(transfert_gold_use_case));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
// Question: How to swap database implementation without causing compilation error due to unknown generic parameter ?
fn init_transfer_gold_capacity() -> TransferGold<InMemoryAccountTable, UUIDGenerator> {
    let repository = InMemoryAccountTable::new();
    TransferGoldModule::new(repository, UUIDGenerator::new()).transfer_gold
}