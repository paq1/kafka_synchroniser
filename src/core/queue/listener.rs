use async_trait::async_trait;

#[async_trait]
pub trait Listener<M>: Send + Sync {
    async fn on_message(&self, message: &M, key: Option<&str>) -> Result<(), String>;
}

#[async_trait]
pub trait MutableListener<M>: Send + Sync {
    async fn on_message(&mut self, message: &M, key: Option<&str>) -> Result<(), String>;
}
