use std::sync::Arc;
use async_trait::async_trait;
use log::info;
use crate::core::queue::listener::Listener;
use crate::core::queue::sync::can_get_correlation_id::CanGetCorrelationId;
use crate::core::queue::sync::subscriber::CanSubscribe;

pub struct ListenerSynchronizer<R>
where
    R: CanGetCorrelationId
{
    pub subscriber: Arc<dyn CanSubscribe<R>>,
}

#[async_trait]
impl<R> Listener<R> for ListenerSynchronizer<R>
where
    R: CanGetCorrelationId + Send + Sync,
{
    async fn on_message(&self, message: &R, _key: Option<&str>) -> Result<(), String> {
        let correlation_id = message.get_correlation_id();
        info!("listener sync : envoie du message dans le subscriber pour : {correlation_id}");
        self.subscriber.send(&correlation_id, message).await
    }
}