use std::fmt::Display;

pub struct AccountState {
    pub balance: f32,
    pub status: AccountStatus
}
impl AccountState {
    pub fn new(balance: f32, status: AccountStatus) -> AccountState {
        AccountState { balance, status }
    }
}

pub enum AccountStatus {
    Active,
    Suspended,
    Blocked,
}

pub struct TransferRequest {
    pub gram_gold: GoldQuantity,
    pub destination: String
}

impl TransferRequest {
    pub fn new(gram_gold: GoldQuantity, destination: String) -> TransferRequest {
        TransferRequest { gram_gold, destination }
    }
}
#[derive(PartialOrd, PartialEq, Debug)]
pub enum TransferDecision {
    Accepted {
        gram_gold: f32,
        destination: String,
    },
    Rejected {
        reason: TransferRejectionCause
    }
}
#[derive(PartialOrd, PartialEq, Debug)]
pub enum TransferRejectionCause {
    AccountSuspended,
    AccountBlocked,
    InsufficientBalance,
}
impl Display for TransferRejectionCause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct GoldQuantity {
    gram_gold: f32,
}
impl GoldQuantity {
    pub fn new(gram_gold: f32) -> Option<GoldQuantity> {
        if gram_gold > 0.0 {
            Some(GoldQuantity { gram_gold })
        }else { None }
    }
    pub fn value(&self) -> f32 {
        self.gram_gold
    }
}