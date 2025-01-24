use banking_demo_application::services::AccountService;
use banking_demo_application::spi::{AccountRepository, IdGenerator};
use banking_demo_application::use_case::TransferGold;



pub struct TransferGoldModule<T: AccountRepository, I: IdGenerator> {
    pub transfer_gold: TransferGold<T, I>
}

impl <T: AccountRepository, I: IdGenerator> TransferGoldModule<T, I> {
    pub fn new(repository: T, id_generator: I) -> TransferGoldModule<T, I> {
        let service = AccountService::new(repository);
        TransferGoldModule {
            transfer_gold: TransferGold::new(service, id_generator),
        }
    }
}