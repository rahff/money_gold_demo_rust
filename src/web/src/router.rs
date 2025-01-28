use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::post;
use banking_demo_application::account_repository::InMemoryAccountTable;
use banking_demo_application::id_provider::UUIDGenerator;
use banking_demo_application::use_case::TransferGold;
use crate::controller::{transfer_gold_controller, TransferGoldPayload};



pub fn gold_transfer_router(use_case: TransferGold<InMemoryAccountTable, UUIDGenerator>) -> Router {
    Router::new().route("/transfer", post(|payload: Json<TransferGoldPayload>| async move {
        let (response, _) = transfer_gold_controller(use_case, payload).await;
        // should publish event into queue here to update read side model
        (response.status(), Json(response.body())).into_response()
    }))
}