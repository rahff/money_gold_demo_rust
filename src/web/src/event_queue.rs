use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::events::TransferEvents;

#[async_trait]
pub trait EventPublisher {
    async fn publish(&mut self, event: TransferEvents) -> TransferEvents;
}


#[derive(Clone)]
pub struct InMemoryEventPublisher {
    events: Arc<Mutex<Vec<TransferEvents>>>,
}

impl InMemoryEventPublisher {
    pub(crate) fn new() -> Self {
        InMemoryEventPublisher { events: Arc::new(Mutex::new(Vec::new())) }
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish(&mut self, event: TransferEvents) -> TransferEvents {
        self.events.lock().unwrap().push(event.clone());
        println!("event published {:?}", self.events.clone());
        event
    }
}