pub mod events;
pub mod spi;
pub mod services;
pub mod use_case;
pub mod id_provider;
pub mod account_repository;
pub mod exceptions;
mod data;


#[cfg(test)]
mod tests {
    use banking_demo_core::data_object::TransferRejectionCause;
    use crate::events::EventId;
    use crate::exceptions::{AccountStatusError, CreateTransferRequestError, TransactionError, TransferFailedError};
    use crate::tests::fixtures::gold_transfer_testing_version;
    pub use crate::spi::{AccountRepository, IdGenerator};

    #[tokio::test]
    async fn customer_transfer_gold_to_valid_destination() {
        let gold_transfer = gold_transfer_testing_version();
        let result = gold_transfer
            .execute(String::from("good_destination_id"), String::from("good_account_id"), 5.0).await;
        match result {
            Ok(transfer_event) => {
                assert_eq!(transfer_event.initiator_account, "good_account_id".to_string());
                assert_eq!(transfer_event.id, EventId("fake_uuid_123".to_string()));
                assert_eq!(transfer_event.destination_account, "good_destination_id".to_string());
                assert_eq!(transfer_event.gram_gold_amount, 5.0)
            },
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn customer_transfer_gold_but_failed_for_insufficient_balance() {
        let gold_transfer = gold_transfer_testing_version();
        let result = gold_transfer
            .execute(String::from("good_destination_id"), String::from("good_account_id"), 50000.0).await;
        match result {
            Ok(_) => assert!(false),
            Err(rejected) => {
                assert_eq!(rejected.reason, TransferFailedError::TransactionError(TransactionError::BusinessReason(TransferRejectionCause::InsufficientBalance)));
                assert_eq!(rejected.initiator_account, String::from("good_account_id"));
                assert_eq!(rejected.destination_account, String::from("good_destination_id"));
            }
        }
    }

    #[tokio::test]
    async fn customer_cannot_transfer_negative_gold_quantity() {
        let gold_transfer = gold_transfer_testing_version();
        let result = gold_transfer
            .execute(String::from("good_destination_id"), String::from("good_account_id"), -500.0).await;
        match result {
            Ok(_) => assert!(false),
            Err(rejected) => {
                assert_eq!(rejected.reason, TransferFailedError::InvalidRequest(CreateTransferRequestError::InvalidGoldQuantity));
                assert_eq!(rejected.initiator_account, String::from("good_account_id"));
                assert_eq!(rejected.destination_account, String::from("good_destination_id"));
            }
        }
    }

    #[tokio::test]
    async fn customer_transfer_gold_to_invalid_destination() {
        let gold_transfer = gold_transfer_testing_version();
        let result = gold_transfer
            .execute(String::from("wrong_destination_id"), String::from("good_account_id"), 5.0).await;
        match result {
            Ok(_) => assert!(false),
            Err(reject_event) => {
                assert_eq!(reject_event.reason, TransferFailedError::InvalidRequest(CreateTransferRequestError::InvalidTransferRequest(AccountStatusError::AccountNotFoundError)));
                assert_eq!(reject_event.id, EventId("fake_uuid_123".to_string()));
                assert_eq!(reject_event.destination_account, String::from("wrong_destination_id"));
                assert_eq!(reject_event.initiator_account, String::from("good_account_id"));
                assert_eq!(reject_event.gram_gold_amount, 5.0);
            }
        }
    }



    mod fixtures {
        use crate::services::AccountService;
        use crate::tests::fakes::{FakeIdGenerator, FakesAccountRepository};
        use crate::use_case::TransferGold;

        pub fn gold_transfer_testing_version() -> TransferGold<FakesAccountRepository, FakeIdGenerator> {
            let account_repository = FakesAccountRepository::new();
            let id_generator = FakeIdGenerator::new();
            let account_service = AccountService::new(account_repository);
            TransferGold::new(account_service, id_generator)
        }
    }
    mod fakes {
        use async_trait::async_trait;
        use banking_demo_core::data_object::{AccountState, AccountStatus};
        use crate::exceptions::{AccountStatusError, TransactionError};
        use crate::tests::{AccountRepository, IdGenerator};
        pub struct FakeIdGenerator {}
        impl FakeIdGenerator {
            pub fn new() -> FakeIdGenerator {
                FakeIdGenerator{}
            }
        }
        impl IdGenerator for  FakeIdGenerator {
            fn generate_id(&self) -> String {
                String::from("fake_uuid_123")
            }
        }
        pub struct FakesAccountRepository {}
        impl FakesAccountRepository {
            pub fn new() -> Self {
                FakesAccountRepository{}
            }
        }

        #[async_trait]
        impl AccountRepository for FakesAccountRepository {

            async fn get_account_state(&self, _: String) -> Result<AccountState, AccountStatusError> {
                Ok(AccountState::new(587.0, AccountStatus::Active))
            }

            async fn verify_destination(&self, destination_id: String) -> Result<String, AccountStatusError> {
                if destination_id == String::from("good_destination_id") {
                    Ok(String::from("good_destination_id"))
                }else {
                    Err(AccountStatusError::AccountNotFoundError)
                }
            }

            async fn persist_transaction(&self, _: String, _: String, _: f32) -> Result<(), TransactionError> {
                Ok(())
            }
        }
    }
}
