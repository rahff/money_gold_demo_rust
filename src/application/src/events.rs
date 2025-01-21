pub struct GoldTransferEvent {
    pub(crate) id: String,
    pub(crate) destination_account: String,
    pub(crate) initiator_account: String,
    pub(crate) gram_gold_amount: f32,
}
impl GoldTransferEvent {

    pub fn new(id: String,
               destination_account: String,
               initiator_account: String,
               gram_gold_amount: f32) -> Self {
        GoldTransferEvent{
            id,
            destination_account,
            initiator_account,
            gram_gold_amount
        }
    }
}
pub struct TransferRejectedEvent {
    pub id: String,
    pub destination_account: String,
    pub initiator_account: String,
    pub gram_gold_amount: f32,
    pub reason: String,
}

impl TransferRejectedEvent {
    pub fn new(id: String, destination_account: String, initiator_account: String, gram_gold_amount: f32, reason: String) -> Self {
        TransferRejectedEvent{
            id,
            destination_account,
            initiator_account,
            gram_gold_amount,
            reason
        }
    }
}

