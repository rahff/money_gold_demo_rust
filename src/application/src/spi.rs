use async_trait::async_trait;
use banking_demo_core::data_object::AccountState;


pub struct  AccountNotFoundError {}
impl AccountNotFoundError {
    pub fn message(&self) -> String {
        String::from("account not found")
    }
}
#[async_trait]
pub trait AccountRepository {
    async fn get_account_state(&self, account_id: String) -> Result<AccountState, AccountNotFoundError>;
    async fn verify_destination(&self, destination_id: String) -> Option<String>;
}
pub trait IdGenerator {
    fn generate_id(&self) -> String;
}
