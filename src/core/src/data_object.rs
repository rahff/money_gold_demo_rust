use std::cmp::Ordering;
use std::fmt::Display;



pub struct AccountState {
    pub balance: GoldQuantity,
    pub status: AccountStatus
}
impl AccountState {
    pub fn new(balance: GoldQuantity, status: AccountStatus) -> AccountState {
        AccountState { balance, status }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccountStatus {
    Active,
    Suspended,
    Blocked,
}

#[derive(Clone)]
pub struct TransferRequest {
    pub account_id: AccountId,
    pub gram_gold: GoldQuantity,
    pub destination: DestinationAccountId
}
impl TransferRequest {
    pub fn new(gram_gold: GoldQuantity, destination: DestinationAccountId, account_id: AccountId) -> TransferRequest {
        TransferRequest { gram_gold, destination, account_id}
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct AccountId(pub String);
impl AccountId {
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub struct DestinationAccountId(pub String);
impl DestinationAccountId {
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum TransferDecision {
    Accepted {
        from_id: AccountId,
        gram_gold: f32,
        destination: DestinationAccountId,
    },
    Rejected {
        reason: TransferRejectionCause
    }
}
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum TransferRejectionCause {
    AccountSuspended,
    AccountBlocked,
    InsufficientBalance,
}

impl TransferRejectionCause {
    pub fn message(&self) -> String {
        match self {
            TransferRejectionCause::AccountSuspended => String::from("account suspended"),
            TransferRejectionCause::AccountBlocked => String::from("account blocked"),
            TransferRejectionCause::InsufficientBalance => String::from("insufficient balance")
        }
    }
}

impl Display for TransferRejectionCause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
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

    pub fn zero() -> GoldQuantity {
        GoldQuantity { gram_gold: 0.0 }
    }
}

impl PartialEq for GoldQuantity {
    fn eq(&self, other: &Self) -> bool {
        self.gram_gold == other.gram_gold
    }
}

impl PartialOrd for GoldQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gram_gold.partial_cmp(&other.gram_gold)
    }
}