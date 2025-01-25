pub mod gold_transfer;
pub mod data_object;



#[cfg(test)]
mod tests {
    use crate::gold_transfer::*;
    use TransferRejectionCause::{AccountSuspended, InsufficientBalance, AccountBlocked};
    use crate::data_object::{AccountId, AccountState, AccountStatus, DestinationAccountId, TransferDecision, TransferRejectionCause, TransferRequest};
    use crate::data_object::GoldQuantity;


    #[test]
    fn customer_with_sufficient_funds_and_activated_account_can_make_gold_transfer() {
        let account_state = AccountState::new(200.0, AccountStatus::Active);
        let gold_transfer_request = TransferRequest::new(GoldQuantity::new(100.0).unwrap(), DestinationAccountId("98765".to_string()),AccountId("123".to_string()));
        let gold_transfer_decision = gold_transfer(account_state, gold_transfer_request);
        assert_eq!(gold_transfer_decision, TransferDecision::Accepted {gram_gold: 100.0, destination: DestinationAccountId("98765".to_string()), from_id: AccountId("123".to_string())});
    }

    #[test]
    fn customer_with_not_sufficient_funds_cannot_make_gold_transfer() {
        let account_state = AccountState::new(200.0, AccountStatus::Active);
        let gold_transfer_request = TransferRequest::new(GoldQuantity::new(201.0).unwrap(), DestinationAccountId("98765".to_string()), AccountId("123".to_string()));
        let gold_transfer_decision = gold_transfer(account_state, gold_transfer_request);
        assert_eq!(gold_transfer_decision, TransferDecision::Rejected{reason: InsufficientBalance})
    }

    #[test]
    fn customer_with_blocked_account_cannot_make_gold_transfer() {
        let account_state = AccountState::new(200.0, AccountStatus::Blocked);
        let gold_transfer_request = TransferRequest::new(GoldQuantity::new(100.0).unwrap(), DestinationAccountId("98765".to_string()), AccountId("123".to_string()));
        let gold_transfer_decision = gold_transfer(account_state, gold_transfer_request);
        assert_eq!(gold_transfer_decision, TransferDecision::Rejected{reason: AccountBlocked})
    }

    #[test]
    fn customer_with_suspended_account_cannot_make_gold_transfer() {
        let account_state = AccountState::new(200.0, AccountStatus::Suspended);
        let gold_transfer_request = TransferRequest::new(GoldQuantity::new(100.0).unwrap(), DestinationAccountId("98765".to_string()), AccountId("123".to_string()));
        let gold_transfer_decision = gold_transfer(account_state, gold_transfer_request);
        assert_eq!(gold_transfer_decision, TransferDecision::Rejected{reason: AccountSuspended})
    }

    #[test]
    fn the_customer_must_be_informed_why_its_transfer_was_rejected() {
        let account_state = AccountState::new(200.0, AccountStatus::Suspended);
        let gold_transfer_request = TransferRequest::new(GoldQuantity::new(100.0).unwrap(), DestinationAccountId("98765".to_string()), AccountId("123".to_string()));
        let gold_transfer_decision = gold_transfer(account_state, gold_transfer_request);
        assert_eq!(gold_transfer_decision, TransferDecision::Rejected{reason: AccountSuspended})
    }
}
