use std::fmt::Debug;
use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::datas::Data;
use crate::core::queue::listener::Listener;
use std::sync::Arc;
use async_trait::async_trait;
use serde::Serialize;
use crate::core::queue::sync::read_write::can_compute_command::CanComputeCommand;

pub struct ListenerReadWrite<CMD, R> {
    pub producer: Arc<dyn CanProduceInQueue<R>>,
    pub compute_cmd: Box<dyn CanComputeCommand<CMD, R>>,
    pub topic_result: String,
}

#[async_trait]
impl<CMD, R> Listener<CMD> for ListenerReadWrite<CMD, R>
where
    CMD: Send + Sync + Debug,
    R: Serialize,
{
    async fn on_message(&self, message: &CMD, key: Option<&str>) -> Result<(), String> {
        println!("read write listener : Received message: {:?}", message);
        let r = self.compute_cmd.compute_cmd(message).await?;
        self.producer
            .produce_data(&self.topic_result, &r, key)
    }
}
