use crate::data_object::{AccountState, AccountStatus, TransferDecision, TransferRejectionCause, TransferRequest};
use crate::data_object::TransferRejectionCause::{InsufficientBalance, AccountBlocked, AccountSuspended};


pub fn gold_transfer(account_state: AccountState, request: TransferRequest) -> TransferDecision {
    match account_state.status {
        AccountStatus::Active => check_balance(account_state, request),
        _ => reject_with_account_status_reason(account_state)
    }
}
fn reject_with_account_status_reason(account_state: AccountState) ->TransferDecision {
    match account_state.status {
        AccountStatus::Blocked => reject_transfer(AccountBlocked),
        _ => reject_transfer(AccountSuspended)
    }
}
fn check_balance(account_state: AccountState, request: TransferRequest) -> TransferDecision {
    match account_state.balance > request.gram_gold {
        true => accept_transfer(request),
        false => reject_transfer(InsufficientBalance)
    }
}
fn reject_transfer(rejection_cause: TransferRejectionCause) -> TransferDecision {
    TransferDecision::Rejected{reason: rejection_cause}
}

fn accept_transfer(request: TransferRequest) -> TransferDecision {
    TransferDecision::Accepted {
        gram_gold: request.gram_gold.value(),
        destination: request.destination,
        from_id: request.account_id
    }
}