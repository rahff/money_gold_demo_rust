use banking_demo_core::data_object::{AccountId, AccountState, DestinationAccountId, GoldQuantity, TransferRequest};
use crate::exceptions::{AccountStatusError, CreateTransferRequestError, TransactionError};
use crate::spi::AccountRepository;


#[derive(Clone)]
pub struct AccountService<T: AccountRepository> {
    pub account_repository: T
}


impl <T: AccountRepository> AccountService<T> {

    pub fn new(account_repository: T) -> Self {
        AccountService{account_repository}
    }

    pub(crate) async fn gold_transfer_request(&self, destination_id: String, amount: f32, account_id: String) -> Result<TransferRequest, CreateTransferRequestError> {
        if let Some(gold_quantity) = GoldQuantity::new(amount) {
            let is_destination_exist = self.account_repository.verify_destination(destination_id).await;
            match is_destination_exist {
                Ok(destination_id) => Ok(TransferRequest::new(gold_quantity, DestinationAccountId(destination_id), AccountId(account_id))),
                Err(error) => Err(CreateTransferRequestError::InvalidTransferRequest(error))
            }
        }else {
            Err(CreateTransferRequestError::InvalidGoldQuantity)
        }

    }

    pub(crate) async fn get_account_state(&self, account_id: String) -> Result<AccountState, AccountStatusError> {
        let get_result = self.account_repository.get_account_state(account_id).await;
        match get_result {
            Ok(account_state) => Ok(account_state),
            Err(error) => {Err(error)}
        }
    }
    pub async fn do_account_transaction(&self, from_account_id: AccountId, destination_account_id: DestinationAccountId, amount: f32) -> Result<(), TransactionError> {
        self.account_repository.persist_transaction(from_account_id.0, destination_account_id.0, amount).await
    }
}
