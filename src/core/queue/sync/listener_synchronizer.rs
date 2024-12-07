use std::sync::Arc;
use async_trait::async_trait;
use crate::core::queue::listener::Listener;
use crate::core::queue::sync::can_get_correlation_id::CanGetCorrelationId;
use crate::core::queue::sync::subscriber::CanSubscribe;

pub struct ListenerSynchronizer<R>
where
    R: CanGetCorrelationId
{
    subscriber: Arc<dyn CanSubscribe<R>>,
}

#[async_trait]
impl<R> Listener<R> for ListenerSynchronizer<R>
where
    R: CanGetCorrelationId + Send + Sync,
{
    async fn on_message(&self, message: &R) -> Result<(), String> {
        let correlation_id = message.get_correlation_id();
        self.subscriber.send(&correlation_id, message).await
    }
}