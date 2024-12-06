use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::datas::Data;
use crate::infra::kafka::producer::SimpleKafkaProducer;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::Producer;
use rdkafka::Message;
use serde::Serialize;

pub mod core;
pub mod infra;

#[tokio::main]
async fn main() -> Result<(), String> {
    let producer = SimpleKafkaProducer::new("127.0.0.1:9092")?;

    let consumer: StreamConsumer = rdkafka::ClientConfig::new()
        .set("group.id", "group-pouet")
        .set("bootstrap.servers", "127.0.0.1:9092")
        .set("auto.offset.reset", "earliest" /* latest */)
        .create()
        .map_err(|e| e.to_string())?;

    let data = Data { data: "petit test avec wrapper service kafka" };
    let key = "monid";

    producer.produce_data("test-cmd", &data, Some(key))?;

    consume("test-cmd", &consumer).await;

    Ok(println!("Message envoyé !"))
}

async fn consume(topic: &str, consumer: &StreamConsumer) {
    consumer.subscribe(&[topic]).expect("Souscription échouée");

    // TODO : clean ca
    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(Ok(paylaod)) = message.payload_view::<str>().or(None) {
                    println!("payload str : {paylaod}");
                    let x = serde_json::from_str::<Data<String>>(paylaod).unwrap();

                    println!("payload object {:?}", x);
                }
            },
            _ => {
                println!("xxx");
            }
        }
    }
}