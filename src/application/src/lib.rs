mod events;
mod spi;
mod services;
mod use_case;


#[cfg(test)]
mod tests {
    use crate::tests::fixtures::gold_transfer_testing_version;
    pub use crate::spi::{AccountRepository, IdGenerator};

    #[tokio::test]
    async fn customer_transfer_gold_to_valid_destination() {
        let gold_transfer = gold_transfer_testing_version();
        let result = gold_transfer
            .execute(String::from("good_destination_id"), String::from("good_account_id"), 5.0).await;
        match result {
            Ok(transfer_event) => {
                assert_eq!(transfer_event.initiator_account, String::from("good_account_id"));
                assert_eq!(transfer_event.id, String::from("fake_uuid_123"));
                assert_eq!(transfer_event.destination_account, String::from("good_destination_id"));
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
            Err(rejected) => assert_eq!(rejected.reason, String::from("InsufficientBalance"))
        }
    }

    #[tokio::test]
    async fn customer_cannot_transfer_negative_gold_quantity() {
        let gold_transfer = gold_transfer_testing_version();
        let result = gold_transfer
            .execute(String::from("good_destination_id"), String::from("good_account_id"), -500.0).await;
        match result {
            Ok(_) => assert!(false),
            Err(rejected) => assert_eq!(rejected.reason, String::from("Invalid gold quantity"))
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
                assert_eq!(reject_event.reason, String::from("Destination not found or invalid"));
                assert_eq!(reject_event.id, String::from("fake_uuid_123"));
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
        use crate::spi::{AccountNotFoundError};
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

            async fn get_account_state(&self, _: String) -> Result<AccountState, AccountNotFoundError> {
                Ok(AccountState::new(587.0, AccountStatus::Active))
            }

            async fn verify_destination(&self, destination_id: String) -> Option<String> {
                if destination_id == String::from("good_destination_id") {
                    Some(String::from("good_destination_id"))
                }else {
                    None
                }
            }
        }
    }
}
