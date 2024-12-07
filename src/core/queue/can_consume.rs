use async_trait::async_trait;

#[async_trait]
pub trait CanConsumeQueue: Send + Sync {
    async fn consume(&self) -> Result<(), String>;
    async fn stop(&mut self) -> Result<(), String>;
}
