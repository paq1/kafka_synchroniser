use std::sync::Arc;
use crate::core::queue::can_consume::CanConsumeQueue;
use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::datas::Data;
use crate::infra::kafka::consumer::SimpleKafkaConsumer;
use crate::infra::kafka::producer::SimpleKafkaProducer;


use std::thread::sleep;
use std::time::Duration;
use log::{debug, info};
use tokio::task;
use uuid::Uuid;
use crate::core::queue::sync::subscriber::{CanSubscribe, Subscriber};
use crate::infra::exemple_component::{EngineExemple, ExCmd, ExResultRecord};

pub mod core;
pub mod infra;


#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("RUST_LOG: {:?}", std::env::var("RUST_LOG"));
    info!("RDK_LOG: {:?}", std::env::var("RDK_LOG"));

    let producer_commands: Arc<dyn CanProduceInQueue<ExCmd>> = Arc::new(SimpleKafkaProducer::new("127.0.0.1:9092")?);
    let producer_results: Arc<dyn CanProduceInQueue<ExResultRecord>> = Arc::new(SimpleKafkaProducer::new("127.0.0.1:9092")?);
    let subscriber: Arc<dyn CanSubscribe<ExResultRecord>> = Arc::new(Subscriber::new());
    let engine = EngineExemple::new(
        producer_commands.clone(),
        producer_results.clone(),
        subscriber.clone(),
    )?;

    engine.start_listener()?;


    let random_correlation_id = Uuid::new_v4().to_string();
    debug!("current correlation id: {}", random_correlation_id);

    let test_cmd = ExCmd { nom: "paquin".to_string(), prenom: "pierre".to_string(), correlation_id: random_correlation_id };

    // debug!("wait listener created (because latest)");
    // sleep(Duration::from_secs(10));
    // debug!("go produce");

    let result = engine.offer(&test_cmd).await?;

    info!("Offer result: {:?}", result);

    Ok(())
}