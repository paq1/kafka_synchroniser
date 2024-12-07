use crate::core::exemple::listener_ex::CmdTestListener;
use crate::core::queue::can_consume::CanConsumeQueue;
use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::datas::Data;
use crate::infra::kafka::consumer::SimpleKafkaConsumer;
use crate::infra::kafka::producer::SimpleKafkaProducer;





use std::thread::sleep;
use std::time::Duration;
use tokio::task;

pub mod core;
pub mod infra;

#[tokio::main]
async fn main() -> Result<(), String> {
    let producer = SimpleKafkaProducer::new("127.0.0.1:9092")?;

    let listener = Box::new(CmdTestListener {});
    let consumer: SimpleKafkaConsumer<Data<String>> =
        SimpleKafkaConsumer::new("test-cmd", "my_group", listener)?;

    let data = Data {
        data: "petit test avec wrapper service kafka",
    };
    let key = "monid";

    sleep(Duration::from_secs(10));
    producer.produce_data("test-cmd", &data, Some(key))?;

    // TODO : voir comment le stopper (service arc qui stop ca lol)
    task::spawn(async move { consumer.consume().await });
    // consumer.consume().await?;
    sleep(Duration::from_secs(10));

    Ok(println!("Message envoyé !"))
}