use uuid::Uuid;
use crate::spi::IdGenerator;


#[derive(Clone)]
pub struct UUIDGenerator {}

impl UUIDGenerator {
    pub fn new() -> Self {
        UUIDGenerator {}
    }
}

impl IdGenerator for UUIDGenerator {
    fn generate_id(&self) -> String {
        Uuid::new_v4().to_string()
    }
}