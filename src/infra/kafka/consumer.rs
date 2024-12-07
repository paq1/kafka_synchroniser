use crate::core::queue::can_consume::CanConsumeQueue;
use crate::core::queue::listener::Listener;
use async_trait::async_trait;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde::de::DeserializeOwned;

pub struct SimpleKafkaConsumer<M> {
    pub consumer: StreamConsumer,
    pub listener: Box<dyn Listener<M>>,
    is_started: bool,
}

impl<M> SimpleKafkaConsumer<M> {
    pub fn new(topic: &str, group_id: &str, listener: Box<dyn Listener<M>>) -> Result<Self, String> {

        println!("Creating new Kafka consumer from {}", topic);

        // TODO : passer la config en parametre ?
        let consumer: StreamConsumer = rdkafka::ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", "127.0.0.1:9092")
            .set("auto.offset.reset", "earliest" /* earliest */)
            .create()
            .map_err(|e| e.to_string())?;

        consumer.subscribe(&[topic]).map_err(|e| e.to_string())?;

        Ok(Self { consumer, listener, is_started: true })
    }

    pub fn start(&mut self) -> () {
        self.is_started = true;
    }

}

#[async_trait]
impl<M> CanConsumeQueue for SimpleKafkaConsumer<M>
where
    M: DeserializeOwned + Send + Sync,
{
    async fn consume(&self) -> Result<(), String> {
        println!("consuming stream");

        loop {

            if !self.is_started {
                break Ok::<(), String>(());
            }

            match self.consumer.recv().await {
                Ok(message) => {
                    if let Some(Ok(paylaod)) = message.payload_view::<str>().or(None) {
                        println!("payload str : {paylaod}");
                        let deserialize_msg = serde_json::from_str::<M>(paylaod);
                        if let Ok(msg) = deserialize_msg {
                            let _ = self.listener.on_message(&msg, None).await; // TODO : recupere la key et la donner ici None to Some(key)
                        } else {
                            println!("error deserializing message"); // TODO : mettre un warning en logging
                        }
                        // break Ok(());
                    }
                }
                Err(e) => {
                    let err_str = format!("{:?}", e);
                    println!("kafka erreur {err_str}");
                    // break Err(format!("une erreur est survenue : {err_str}"));
                }
            }
        }?;

        println!("end stream");

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), String> {
        self.is_started = false;
        Ok(())
    }
}