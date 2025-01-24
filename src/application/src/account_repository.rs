use std::sync::{Arc, Mutex, MutexGuard};
use async_trait::async_trait;
use banking_demo_core::data_object::{AccountState, AccountStatus};
use crate::data::AccountData;
use crate::exceptions::{AccountStatusError, TransactionError};
use crate::spi::AccountRepository;


#[derive(Clone)]
pub struct InMemoryAccountTable {
    data: Arc<Mutex<Vec<AccountData>>>
}

impl InMemoryAccountTable {
    pub fn new() -> Self {
        let mut data: Vec<AccountData> = Vec::new();
        data.push(AccountData{id: "123".to_string(), balance: 800.0, account_status: AccountStatus::Active});
        data.push(AccountData{id: "456".to_string(), balance: 500.0, account_status: AccountStatus::Active});
        data.push(AccountData{id: "789".to_string(), balance: 50.0, account_status: AccountStatus::Blocked});
        InMemoryAccountTable {
            data: Arc::new(Mutex::new(data))
        }
    }
}


#[async_trait]
impl AccountRepository for InMemoryAccountTable {

    async fn get_account_state(&self, account_id: String) -> Result<AccountState, AccountStatusError> {
        let data = self.data.lock().unwrap();
        let find_result = data.iter().find(|account| account.id == account_id).cloned();
        match find_result {
            Some(account_state) => Ok(account_state.to_account_state()),
            None => Err(AccountStatusError::AccountNotFoundError)
        }

    }

    async fn verify_destination(&self, destination_id: String) -> Result<String, AccountStatusError> {
        let data = self.data.lock().unwrap();
        let find_result = data.iter().find(|account| account.id == destination_id).cloned();
        match find_result {
            Some(account_state) => {
                match account_state.account_status {
                    AccountStatus::Active => Ok(destination_id),
                    AccountStatus::Suspended => Err(AccountStatusError::AccountSuspendedError),
                    AccountStatus::Blocked => Err(AccountStatusError::AccountBlockedError)
                }
            },
            None => Err(AccountStatusError::AccountNotFoundError)
        }
    }

    async fn persist_transaction(&self, from_id: String, destination_id: String, amount: f32) -> Result<(), TransactionError> {
        let mut data: MutexGuard<Vec<AccountData>> = self.data.lock().unwrap();
        let from = data.iter_mut().find(|account| account.id == from_id).unwrap();
        from.balance -= amount;
        let destination = data.iter_mut().find(|account| account.id == destination_id).unwrap();
        destination.balance += amount;
        Ok(())
    }
}