use async_trait::async_trait;
use banking_demo_core::data_object::AccountState;
use crate::exceptions::{AccountStatusError, TransactionError};



#[async_trait]
pub trait AccountRepository {
    async fn get_account_state(&self, account_id: String) -> Result<AccountState, AccountStatusError>;
    async fn verify_destination(&self, destination_id: String) -> Result<String, AccountStatusError>;
    async fn persist_transaction(&self, from_id: String, destination_id: String, amount: f32) -> Result<(), TransactionError>;
}

pub trait IdGenerator {
    fn generate_id(&self) -> String;
}

#[cfg(test)]
mod spi_test {
    use banking_demo_core::data_object::{AccountState, AccountStatus, GoldQuantity};
    use crate::account_repository::InMemoryAccountTable;
    use crate::spi::{AccountRepository, AccountStatusError};



    #[tokio::test]
    async fn get_account_state() {
        let repository = InMemoryAccountTable::new();
        let result = repository.get_account_state("123".to_string()).await;
        match result {
            Ok(account_state) => assert!(account_state.status.eq(&AccountStatus::Active)),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn verify_account_destination() {
        let repository = InMemoryAccountTable::new();
        let result = repository.verify_destination("456".to_string()).await;
        match result {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, AccountStatusError::AccountSuspendedError)
        }
    }

    #[tokio::test]
    async fn do_transaction() {
        let repository = InMemoryAccountTable::new();
        let result = repository.persist_transaction("123".to_string(), "456".to_string(), 50.0).await;
        match result {
            Ok(_) => {
                let (result1, result2) = tokio::join!(repository.get_account_state("123".to_string()), repository.get_account_state("456".to_string()));
                check_from_account_balance(result1);
                check_destination_account_balance(result2);
            },
            Err(_) => assert!(false)
        }
    }

    fn check_destination_account_balance(result2: Result<AccountState, AccountStatusError>) {
        match result2 {
            Ok(account_state) => {
                let expected: GoldQuantity = GoldQuantity::new(550.0).unwrap();
                assert_eq!(account_state.balance, expected)
            },
            Err(_) => assert!(false)
        }
    }

    fn check_from_account_balance(result1: Result<AccountState, AccountStatusError>) {
        match result1 {
            Ok(account_state) => {
                let expected = GoldQuantity::new(750.0).unwrap();
                assert_eq!(account_state.balance, expected)
            },
            Err(_) => assert!(false)
        }
    }
}