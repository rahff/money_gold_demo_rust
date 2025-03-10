mod router;
mod controller;
mod exceptions;
mod transfer_gold_module;
pub mod app;
mod event_queue;
mod events;
mod http_response;

#[cfg(test)]
mod tests {
    use crate::app::app;

    #[tokio::test]
    async fn it_works() {
        tokio::spawn(app());

    }
}
