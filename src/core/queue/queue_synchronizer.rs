use async_trait::async_trait;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::time::Duration;
use serde::Serialize;
use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::datas::Data;

#[async_trait]
pub trait QueueSynchronizer<M, R>: Send + Sync
where
    R: Send,
{
    async fn wait_result(&self, correlation_id: &str, topic: &str, message: &M, key: Option<&str>) -> Result<R, String>;
}

//////////////// FIXME a bouger

pub struct KafkaSynchronizer<M, R> {
    producer: Arc<dyn CanProduceInQueue<M>>,
    subscriber: Arc<dyn CanSubscribe<R>>,
}

#[async_trait]
impl<M, R> QueueSynchronizer<M, R> for KafkaSynchronizer<M, R>
where
    R: Send + Debug,
    M: Clone + Debug + Send + Sync + Serialize,
{
    async fn wait_result(&self, correlation_id: &str, topic: &str, message: &M, key: Option<&str>) -> Result<R, String> {
        let (tx, rx) = channel();
        self.subscriber.subscribe(correlation_id, tx).await?;
        self.producer.produce_data(topic, &Data {data: message.clone()}, key)?;

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

pub struct Subscriber<RESPONSE>
where
    RESPONSE: Send,
{
    pub datas: Arc<Mutex<HashMap<String, Sender<RESPONSE>>>>,
}

#[async_trait]
pub trait CanSubscribe<RESPONSE>: Send + Sync
where
    RESPONSE: Send,
{
    async fn subscribe(&self, correlation_id: &str, sender: Sender<RESPONSE>)
        -> Result<(), String>;
    async fn unsubscribe(&self, correlation_id: &str) -> Result<(), String>;
    async fn send(&self, correlation_id: &str, message: RESPONSE) -> Result<(), String>;
}

#[async_trait]
impl<RESPONSE> CanSubscribe<RESPONSE> for Subscriber<RESPONSE>
where
    RESPONSE: Send,
{
    async fn subscribe(
        &self,
        correlation_id: &str,
        sender: Sender<RESPONSE>,
    ) -> Result<(), String> {
        let mut lock = self.datas.lock().await;
        // Stocke un Arc<Sender> directement
        lock.insert(correlation_id.to_string(), sender);
        Ok(())
    }

    async fn unsubscribe(&self, correlation_id: &str) -> Result<(), String> {
        let mut lock = self.datas.lock().await;
        // Stocke un Arc<Sender> directement
        lock.remove(correlation_id);
        Ok(())
    }

    async fn send(&self, correlation_id: &str, message: RESPONSE) -> Result<(), String> {
        let lock = self.datas.lock().await;

        if let Some(sender_arc) = lock.get(correlation_id) {
            // Cloner l'Arc<Sender> pour envoyer le message
            sender_arc
                .send(message)
                .map_err(|e| e.to_string())?;
        } else {
            return Err(format!("Aucun Sender trouvé pour l'ID '{}'", correlation_id).into());
        }

        Ok(())
    }
}
