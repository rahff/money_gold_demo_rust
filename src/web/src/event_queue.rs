use std::fmt::Error;
use async_trait::async_trait;
use banking_demo_application::events::{GoldTransferEvent, TransferRejectedEvent};

pub enum TransferEvents {
    GoldTransferEvent(GoldTransferEvent),
    TransferRejectedEvent(TransferRejectedEvent)
}

#[async_trait]
pub trait EventPublisher {
    async fn publish(event: TransferEvents) -> Result<(), Error>;
}