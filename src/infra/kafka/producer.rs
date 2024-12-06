use crate::core::queue::datas::Data;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use serde::Serialize;
use std::time::Duration;
use rdkafka::ClientConfig;
use crate::core::queue::can_produce::CanProduceInQueue;

pub struct SimpleKafkaProducer {
    producer: BaseProducer,
    timeout_secs: u64
}

impl SimpleKafkaProducer {
    pub fn new(servers: &str) -> Result<Self, String> {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", servers)
            .create()
            .map_err(|e| e.to_string())?;
        Ok(Self { producer, timeout_secs: 30 })
    }
}

impl CanProduceInQueue for SimpleKafkaProducer {

    fn produce_data<T: Serialize>(&self, topic: &str, data: &Data<T>, key: Option<&str>) -> Result<(), String> {
        let data_stringify = serde_json::to_string(data).map_err(|e| e.to_string())?;

        match key {
            Some(k) => {
                self.producer
                    .send(
                        BaseRecord::to(topic)
                            .payload(&data_stringify)
                            .key(k),
                    )
            }
            _ => {
                self.producer
                    .send(
                        BaseRecord::to(topic)
                            .payload(&data_stringify)
                    )
            }
        }
            .map_err(|_| "Échec lors de l'envoi du message".to_string())?;

        self.producer
            .flush(Duration::from_secs(self.timeout_secs))
            .map_err(|err| err.to_string())
    }
}