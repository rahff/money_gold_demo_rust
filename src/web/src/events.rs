use banking_demo_application::events::{GoldTransferEvent, TransferRejectedEvent};

#[derive(Clone, Debug)]
pub enum TransferEvents {
    GoldTransferEvent(GoldTransferEvent),
    TransferRejectedEvent(TransferRejectedEvent),
    InternalServerEvent(TransferRejectedEvent)
}
