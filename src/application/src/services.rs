use std::fmt::{Display, Formatter};
use banking_demo_core::data_object::{AccountState, GoldQuantity, TransferRequest};
use crate::spi::{AccountNotFoundError, AccountRepository};

pub struct AccountService<T: AccountRepository> {
    pub account_repository: T
}


pub enum CreateTransferRequestError {
    InvalidGoldQuantity,
    InvalidTransferRequest,
}
impl Display for CreateTransferRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateTransferRequestError::InvalidGoldQuantity => write!(f, "Invalid gold quantity"),
            CreateTransferRequestError::InvalidTransferRequest => write!(f, "Destination not found or invalid"),
        }
    }
}

impl <T: AccountRepository> AccountService<T> {

    pub fn new(account_repository: T) -> Self {
        AccountService{account_repository}
    }

    pub(crate) async fn gold_transfer_request(&self, destination_id: String, amount: f32) -> Result<TransferRequest, CreateTransferRequestError> {
        if let Some(gold_quantity) = GoldQuantity::new(amount) {
            let is_destination_exist = self.account_repository.verify_destination(destination_id).await;
            match is_destination_exist {
                Some(destination_id) => Ok(TransferRequest::new(gold_quantity, destination_id)),
                None => Err(CreateTransferRequestError::InvalidTransferRequest)
            }
        }else {
            Err(CreateTransferRequestError::InvalidGoldQuantity)
        }

    }
    pub(crate) async fn get_account_state(&self, account_id: String) -> Result<AccountState, AccountNotFoundError> {
        let get_result = self.account_repository.get_account_state(account_id).await;
        match get_result {
            Ok(account_state) => Ok(account_state),
            Err(error) => {Err(error)}
        }
    }
}
