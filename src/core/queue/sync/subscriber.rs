use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use async_trait::async_trait;
use futures::lock::Mutex;

#[async_trait]
pub trait CanSubscribe<RESPONSE>: Send + Sync
where
    RESPONSE: Send,
{
    async fn subscribe(&self, correlation_id: &str, sender: Sender<RESPONSE>)
                       -> Result<(), String>;
    async fn unsubscribe(&self, correlation_id: &str) -> Result<(), String>;
    async fn send(&self, correlation_id: &str, message: &RESPONSE) -> Result<(), String>;
}

pub struct Subscriber<RESPONSE>
where
    RESPONSE: Send,
{
    pub datas: Arc<Mutex<HashMap<String, Sender<RESPONSE>>>>,
}


#[async_trait]
impl<RESPONSE> CanSubscribe<RESPONSE> for Subscriber<RESPONSE>
where
    RESPONSE: Send + Sync + Clone,
{
    async fn subscribe(
        &self,
        correlation_id: &str,
        sender: Sender<RESPONSE>,
    ) -> Result<(), String> {
        let mut lock = self.datas.lock().await;
        lock.insert(correlation_id.to_string(), sender);
        Ok(())
    }

    async fn unsubscribe(&self, correlation_id: &str) -> Result<(), String> {
        let mut lock = self.datas.lock().await;
        lock.remove(correlation_id);
        Ok(())
    }

    async fn send(&self, correlation_id: &str, message: &RESPONSE) -> Result<(), String> {
        let lock = self.datas.lock().await;

        if let Some(sender_arc) = lock.get(correlation_id) {
            sender_arc
                .send(message.clone())
                .map_err(|e| e.to_string())?;
        } else {
            return Err(format!("Aucun Sender trouvé pour l'ID '{}'", correlation_id).into());
        }

        Ok(())
    }
}
