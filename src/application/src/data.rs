use banking_demo_core::data_object::{AccountState, AccountStatus, GoldQuantity};

#[derive(Clone, Debug)]
pub struct AccountData {
    pub id: String,
    pub balance: f32,
    pub account_status: AccountStatus
}

impl AccountData {
    pub fn to_account_state(&self) -> AccountState {
        AccountState::new(GoldQuantity::new(self.balance).unwrap_or(GoldQuantity::zero()), self.account_status.clone())
    }
}
