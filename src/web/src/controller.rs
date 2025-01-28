use axum::body::Body;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use banking_demo_application::account_repository::InMemoryAccountTable;
use banking_demo_application::exceptions::{AccountStatusError, CreateTransferRequestError, TransactionError, TransferFailedError};
use banking_demo_application::id_provider::UUIDGenerator;
use banking_demo_application::use_case::{TransferGold, TransferResult};
use serde::{Deserialize, Serialize};
use banking_demo_application::events::{GoldTransferEvent, TransferRejectedEvent};
use banking_demo_application::spi::{AccountRepository, IdGenerator};
use crate::exceptions::ErrorResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferGoldPayload {
    from_id: String,
    destination_id: String,
    amount: f32
}


pub async fn transfer_gold_controller<T: AccountRepository, I: IdGenerator>(transfer_gold: TransferGold<T, I>, payload: Json<TransferGoldPayload>) -> (Response<Value>, TransferResult) {
    let result = transfer_gold.execute(payload.destination_id.clone(), payload.from_id.clone(), payload.amount).await;
    match result {
        Ok(ok_event) => (ok_response(ok_event.clone()), Ok(ok_event)),
        Err(reject_event) => (reject_response(reject_event.clone()), Err(reject_event))
    }
}

fn reject_response(reject_event: TransferRejectedEvent) -> Response<Value> {
    let error_response = handle_rejected_transfer(reject_event.clone());
    Response::builder().status(error_response.status()).body(json!({
                "status": error_response.status(),
                "message": reject_event.reason.message()})).unwrap()
}

fn ok_response(ok_event: GoldTransferEvent) -> Response<Value> {
    Response::builder().status(200).body(json!({"event": ok_event, "status": 200})).unwrap()
}

fn handle_rejected_transfer(reject_event: TransferRejectedEvent) -> ErrorResponse {
    match reject_event.reason {
        TransferFailedError::InvalidRequest(reason) => handle_invalid_request(reason),
        TransferFailedError::TransactionError(reason) => handle_transaction_error(reason),
        TransferFailedError::AccountStatusError(reason) => bad_request_from_account_status_error(reason)
    }
}

fn bad_request_from_account_status_error(reason: AccountStatusError) -> ErrorResponse {
    match reason {
        AccountStatusError::AccountNotFoundError => ErrorResponse::BadRequest { message: AccountStatusError::AccountNotFoundError.message() },
        AccountStatusError::AccountSuspendedError => ErrorResponse::BadRequest { message: AccountStatusError::AccountSuspendedError.message() },
        AccountStatusError::AccountBlockedError => ErrorResponse::BadRequest { message: AccountStatusError::AccountBlockedError.message() }
    }

}

fn handle_transaction_error(reason: TransactionError) -> ErrorResponse {
    match reason {
        TransactionError::CreateTransferRequestError(cause) => ErrorResponse::BadRequest { message: cause.message() },
        TransactionError::TechnicalReason(cause) => ErrorResponse::Forbidden { message: cause.message() },
        TransactionError::BusinessReason(cause) => ErrorResponse::Forbidden { message: cause.message() },
    }
}

fn handle_invalid_request(reason: CreateTransferRequestError) -> ErrorResponse {
    match reason {
        CreateTransferRequestError::InvalidTransferRequest(cause) => bad_request_from_account_status_error(cause),
        CreateTransferRequestError::InvalidGoldQuantity => {
            ErrorResponse::BadRequest { message: CreateTransferRequestError::InvalidGoldQuantity.message() }
        }
    }
}