use crate::core::queue::datas::Data;
use rdkafka::producer::Producer;
use serde::Serialize;
use crate::core::queue::can_produce::CanProduceInQueue;
use crate::infra::kafka::producer::SimpleKafkaProducer;

pub mod core;
pub mod infra;

#[tokio::main]
async fn main() -> Result<(), String> {
    let producer = SimpleKafkaProducer::new("127.0.0.1:9092")?;

    let data = Data { data: "petit test avec wrapper service kafka" };
    let key = "monid";

    producer.produce_data("test-cmd", &data, None)?;

    Ok(println!("Message envoyé !"))
}
