use banking_demo_core::data_object::{AccountId, AccountState, DestinationAccountId, TransferDecision, TransferRequest};
use banking_demo_core::gold_transfer::gold_transfer;
use crate::events::{EventId, GoldTransferEvent, TransferRejectedEvent};
use crate::exceptions::{AccountStatusError, CreateTransferRequestError, TransactionError, TransferFailedError};
use crate::services::{AccountService};
use crate::spi::{AccountRepository, IdGenerator};

type PrepareTransferRequestResults = (Result<TransferRequest, CreateTransferRequestError>, Result<AccountState, AccountStatusError>);
pub type TransferResult = Result<GoldTransferEvent, TransferRejectedEvent>;

#[derive(Clone)]
pub struct TransferGold<T: AccountRepository, I: IdGenerator> {
    account_service: AccountService<T>,
    id_generator: I
}

impl <T: AccountRepository, I: IdGenerator> TransferGold<T, I> {
    pub fn new(account_service: AccountService<T>, id_generator: I) -> TransferGold<T, I> {
        TransferGold {
            account_service,
            id_generator
        }
    }


    pub async fn execute(&self, destination_id: String, account_id: String, amount: f32) -> TransferResult {
        let event_id = EventId(self.id_generator.generate_id());
        let (transfer_request, account_state): PrepareTransferRequestResults = self.
            prepare_transfer_request(destination_id.clone(), amount, account_id.clone()).await;
        match transfer_request {
            Ok(request) => {
                match account_state {
                    Ok(state) => {
                        let decision = gold_transfer(state, request.clone());
                        let transaction_result = self.transfer_result_handler(decision, request, event_id.clone()).await;
                        match transaction_result {
                            Ok(event) => Ok(event),
                            Err(error) => Err(TransferRejectedEvent::new(event_id, DestinationAccountId(destination_id), AccountId(account_id), amount, TransferFailedError::TransactionError(error)))
                        }
                    }
                    Err(status_error) =>
                        Err(TransferRejectedEvent::new(event_id, DestinationAccountId(destination_id), AccountId(account_id), amount, TransferFailedError::AccountStatusError(status_error)))
                }
            },
            Err(error) =>
                Err(TransferRejectedEvent::new(event_id, DestinationAccountId(destination_id), AccountId(account_id), amount, TransferFailedError::InvalidRequest(error)))
        }
    }
    async fn transfer_result_handler(&self, transfer_decision: TransferDecision, request: TransferRequest, event_id: EventId) -> Result<GoldTransferEvent, TransactionError> {
        match transfer_decision {
            TransferDecision::Accepted{gram_gold, destination, from_id} => {
                let result = self.account_service.do_account_transaction(from_id, destination, gram_gold).await;
                match result {
                    Ok(_) => Ok(GoldTransferEvent::new(event_id, request.destination, request.account_id, request.gram_gold.value())),
                    Err(err) => Err(err)
                }
            },
            TransferDecision::Rejected { reason } => Err(TransactionError::BusinessReason(reason))
        }
    }

    async fn prepare_transfer_request(&self, destination_id: String, amount: f32, account_id: String) -> PrepareTransferRequestResults {
       let (request_result, account_result) = tokio::join!(
           self.account_service.gold_transfer_request(destination_id, amount, account_id.clone()),
           self.account_service.get_account_state(account_id));
        (request_result, account_result)
    }
}
