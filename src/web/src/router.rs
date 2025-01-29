use axum::{Json, Router};
use axum::routing::post;
use banking_demo_application::account_repository::InMemoryAccountTable;
use banking_demo_application::id_provider::UUIDGenerator;
use crate::controller::{transfer_gold_controller, TransferGoldPayload};
use crate::event_queue::{EventPublisher, InMemoryEventPublisher};
use crate::http_response::make_response_from_event;
use crate::transfer_gold_module::TransferGoldModule;


pub async fn add_transfer_gold_capacity() -> Router {
    let mut event_publisher = InMemoryEventPublisher::new();
    let repository = InMemoryAccountTable::new();
    let transfer_gold_use_case = TransferGoldModule::new(repository, UUIDGenerator::new()).transfer_gold;
    Router::new().route("/transfer", post(|payload: Json<TransferGoldPayload>| async move {
        let event = transfer_gold_controller(transfer_gold_use_case, payload).await;
        let published_event = event_publisher.publish(event.clone()).await;
        make_response_from_event(published_event)

    }))
}
