use banking_demo_core::data_object::{AccountId, DestinationAccountId};
use serde::Serialize;
use crate::exceptions::TransferFailedError;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct EventId(pub String);

impl EventId {
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug, Serialize)]
pub struct GoldTransferEvent {
    pub id: EventId,
    pub(crate) destination_account: String,
    pub(crate) initiator_account: String,
    pub(crate) gram_gold_amount: f32,
}

impl GoldTransferEvent {

    pub fn new(id: EventId,
               destination_account: DestinationAccountId,
               initiator_account: AccountId,
               gram_gold_amount: f32) -> Self {
        GoldTransferEvent{
            id,
            destination_account: destination_account.value(),
            initiator_account: initiator_account.value(),
            gram_gold_amount
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransferRejectedEvent {
    pub id: EventId,
    pub destination_account: String,
    pub initiator_account: String,
    pub gram_gold_amount: f32,
    pub reason: TransferFailedError,
}

impl TransferRejectedEvent {
    pub fn new(id: EventId, destination_account: DestinationAccountId, initiator_account: AccountId, gram_gold_amount: f32, reason: TransferFailedError) -> Self {
        TransferRejectedEvent{
            id,
            destination_account: destination_account.value(),
            initiator_account: initiator_account.value(),
            gram_gold_amount,
            reason
        }
    }
}

