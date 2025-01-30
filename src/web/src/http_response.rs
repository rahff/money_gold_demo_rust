use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use banking_demo_application::events::{GoldTransferEvent, TransferRejectedEvent};
use banking_demo_application::exceptions::{AccountStatusError, CreateTransferRequestError, TransactionError, TransferFailedError};
use crate::events::TransferEvents;
use crate::exceptions::ErrorResponse;

pub fn make_response_from_event(event: TransferEvents) -> impl IntoResponse {
    let response = match event {
        TransferEvents::GoldTransferEvent(ok_event) => ok_response(ok_event),
        TransferEvents::TransferRejectedEvent(rejected_event) => reject_response(rejected_event),
        TransferEvents::InternalServerEvent(failed_event) => reject_response(failed_event)
    };
    (response.status(), Json(response.body())).into_response()
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