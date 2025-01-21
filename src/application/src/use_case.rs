use banking_demo_core::data_object::{AccountState, TransferDecision, TransferRequest};
use banking_demo_core::gold_transfer::gold_transfer;
use crate::events::{GoldTransferEvent, TransferRejectedEvent};
use crate::services::{AccountService, CreateTransferRequestError};
use crate::spi::{AccountNotFoundError, AccountRepository, IdGenerator};

type PrepareTransferRequestResults = (Result<TransferRequest, CreateTransferRequestError>, Result<AccountState, AccountNotFoundError>);
type TransferResult = Result<GoldTransferEvent, TransferRejectedEvent>;
pub(crate) struct TransferGold<T: AccountRepository, I: IdGenerator> {
    pub account_service: AccountService<T>,
    pub id_generator: I
}
impl <T: AccountRepository, I: IdGenerator> TransferGold<T, I> {
    pub fn new(account_service: AccountService<T>, id_generator: I) -> TransferGold<T, I> {
        TransferGold {
            account_service,
            id_generator
        }
    }


    pub async fn execute(&self, destination_id: String, account_id: String, amount: f32) -> TransferResult {
        let event_id = self.id_generator.generate_id();
        let (transfer_request, account_state): PrepareTransferRequestResults = self.
            prepare_transfer_request(destination_id.clone(), amount, account_id.clone()).await;
        match transfer_request {
            Ok(request) =>
                Self::make_transfer_decision(destination_id, account_id, amount, event_id, account_state, request),
            Err(error) =>
                Err(TransferRejectedEvent::new(event_id, destination_id, account_id, amount, error.to_string()))
        }
    }

    fn make_transfer_decision(destination_id: String, account_id: String, amount: f32, event_id: String, account_state: Result<AccountState, AccountNotFoundError>, request: TransferRequest) -> TransferResult {
        match account_state {
            Ok(account_state) => {
                let decision = gold_transfer(account_state, request);
                Self::transfer_event(destination_id, account_id, amount, event_id, decision)
            },
            Err(account_error) => {
                Err(TransferRejectedEvent::new(event_id, destination_id, account_id, amount, account_error.message()))
            }
        }
    }

    fn transfer_event(destination_id: String, account_id: String, amount: f32, event_id: String, decision: TransferDecision) -> TransferResult {
        match decision {
            TransferDecision::Accepted { gram_gold, destination } =>
                Ok(GoldTransferEvent::new(event_id, destination, account_id, gram_gold)),
            TransferDecision::Rejected { reason } =>
                Err(TransferRejectedEvent::new(event_id, destination_id, account_id, amount, reason.to_string()))
        }
    }
    async fn prepare_transfer_request(&self, destination_id: String, amount: f32, account_id: String) -> PrepareTransferRequestResults {
       let (request_result, account_result) = tokio::join!(
           self.account_service.gold_transfer_request(destination_id, amount),
           self.account_service.get_account_state(account_id));
        (request_result, account_result)
    }
}
