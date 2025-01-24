use std::fmt::{Display, Formatter};
use banking_demo_core::data_object::TransferRejectionCause;


#[derive(Debug, PartialEq, Clone)]
pub enum CreateTransferRequestError {
    InvalidGoldQuantity,
    InvalidTransferRequest(AccountStatusError),
}

impl Display for CreateTransferRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateTransferRequestError::InvalidGoldQuantity => write!(f, "Invalid gold quantity"),
            CreateTransferRequestError::InvalidTransferRequest(reason) => write!(f, "{}", reason.message()),
        }
    }
}

impl CreateTransferRequestError {
    pub fn message(&self) -> String {
        match self {
            CreateTransferRequestError::InvalidGoldQuantity => "Invalid gold quantity".to_string(),
            CreateTransferRequestError::InvalidTransferRequest(reason) => reason.to_string()
        }

    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AccountStatusError {
    AccountNotFoundError,
    AccountSuspendedError,
    AccountBlockedError
}

impl Display for AccountStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountStatusError::AccountNotFoundError => write!(f, "Account not found"),
            AccountStatusError::AccountSuspendedError => write!(f, "Account destination suspended"),
            AccountStatusError::AccountBlockedError => write!(f, "Account destination blocked"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TransactionError {
    TechnicalReason(AccountStatusError),
    BusinessReason(TransferRejectionCause),
    CreateTransferRequestError(CreateTransferRequestError)
}

#[derive(PartialEq, Debug, Clone)]
pub enum TransferFailedError {
    AccountStatusError(AccountStatusError),
    TransactionError(TransactionError),
    InvalidRequest(CreateTransferRequestError)
}



impl TransactionError {
    pub fn message(&self) -> String {
        match self {
            TransactionError::TechnicalReason(reason) =>  reason.message(),
            TransactionError::BusinessReason(reason) => reason.to_string(),
            TransactionError::CreateTransferRequestError(reason) => reason.to_string()
        }

    }
}

impl AccountStatusError {
    pub fn message(&self) -> String {
        match self {
            AccountStatusError::AccountNotFoundError => "Account not found".to_string(),
            AccountStatusError::AccountBlockedError => "Account destination blocked".to_string(),
            AccountStatusError::AccountSuspendedError => "Account destination suspended".to_string()
        }
    }
}

impl TransferFailedError {
    pub fn message(&self) -> String {
        match self {
            TransferFailedError::AccountStatusError(reason) => reason.message(),
            TransferFailedError::InvalidRequest(reason) => reason.message(),
            TransferFailedError::TransactionError(reason) => reason.message()
        }
    }
}