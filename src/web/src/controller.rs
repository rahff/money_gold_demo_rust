
use axum::Json;

use serde_json::{json, Value};
use banking_demo_application::exceptions::{AccountStatusError, CreateTransferRequestError, TransactionError, TransferFailedError};
use banking_demo_application::use_case::{TransferGold, TransferResult};
use serde::{Deserialize, Serialize};
use banking_demo_application::events::{GoldTransferEvent, TransferRejectedEvent};
use banking_demo_application::spi::{AccountRepository, IdGenerator};
use crate::events::TransferEvents;
use crate::exceptions::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferGoldPayload {
    from_id: String,
    destination_id: String,
    amount: f32
}


pub async fn transfer_gold_controller<T: AccountRepository, I: IdGenerator>(transfer_gold: TransferGold<T, I>, payload: Json<TransferGoldPayload>) -> TransferEvents {
    let result = transfer_gold.execute(payload.destination_id.clone(), payload.from_id.clone(), payload.amount).await;
    match result {
        Ok(ok_event) => TransferEvents::GoldTransferEvent(ok_event),
        Err(reject_event) => TransferEvents::TransferRejectedEvent(reject_event)
    }
}

