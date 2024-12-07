use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::datas::Data;
use crate::core::queue::sync::subscriber::CanSubscribe;
use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;

#[async_trait]
pub trait QueueSynchronizer<M, R>: Send + Sync
where
    R: Send,
{
    async fn wait_result(&self, correlation_id: &str, topic: &str, message: &M, key: Option<&str>) -> Result<R, String>;
}

pub struct QueueSynchronizerImpl<M, R> {
    pub producer: Arc<dyn CanProduceInQueue<M>>,
    pub subscriber: Arc<dyn CanSubscribe<R>>,
}

#[async_trait]
impl<M, R> QueueSynchronizer<M, R> for QueueSynchronizerImpl<M, R>
where
    R: Send + Debug,
    M: Clone + Debug + Send + Sync + Serialize,
{
    async fn wait_result(&self, correlation_id: &str, topic: &str, message: &M, key: Option<&str>) -> Result<R, String> {
        let (tx, rx) = channel();
        self.subscriber.subscribe(correlation_id, tx).await?;
        println!("queue sync : produce data in : {topic}");
        self.producer.produce_data(topic, &Data {data: message.clone()}, key)?;

        println!("queue sync : en attente de sync");
        let c = rx
            .recv_timeout(Duration::from_secs(30))
            .map(|msg| {
                println!("Reçu : {msg:?}");
                msg
            })
            .map_err(|e| format!("Erreur lors de l'envoi: {}", e))?;

        self.subscriber.unsubscribe(correlation_id).await.map(|_| c)
    }
}
