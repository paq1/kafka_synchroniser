use async_trait::async_trait;

#[async_trait]
pub trait CanComputeCommand<CMD, R>: Send + Sync {
    async fn compute_cmd(&self, cmd: &CMD) -> Result<R, String>;
}