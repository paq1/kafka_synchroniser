use crate::core::queue::can_consume::CanConsumeQueue;
use crate::core::queue::can_produce::CanProduceInQueue;
use crate::core::queue::sync::can_get_correlation_id::CanGetCorrelationId;
use crate::core::queue::sync::listener_synchronizer::ListenerSynchronizer;
use crate::core::queue::sync::queue_synchronizer::{QueueSynchronizer, QueueSynchronizerImpl};
use crate::core::queue::sync::read_write::listener_read_write::ListenerReadWrite;
use crate::core::queue::sync::subscriber::{CanSubscribe, Subscriber};
use crate::infra::kafka::consumer::SimpleKafkaConsumer;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use log::debug;
use tokio::task;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExCmd {
    pub nom: String,
    pub prenom: String,
    pub correlation_id: String,
}

impl CanGetCorrelationId for ExCmd {
    fn get_correlation_id(&self) -> String {
        self.correlation_id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExResultRecord {
    pub nom: String,
    pub prenom: String,
    pub correlation_id: String,
    pub at: String,
    pub by: String,
}

impl CanGetCorrelationId for ExResultRecord {
    fn get_correlation_id(&self) -> String {
        self.correlation_id.clone()
    }
}

use crate::core::queue::sync::read_write::can_compute_command::CanComputeCommand;
use crate::infra::kafka::producer::SimpleKafkaProducer;

pub struct ExempleComputeCommand {}

#[async_trait]
impl CanComputeCommand<ExCmd, ExResultRecord> for ExempleComputeCommand {
    async fn compute_cmd(&self, cmd: &ExCmd) -> Result<ExResultRecord, String> {
        Ok(ExResultRecord {
            nom: cmd.nom.clone(),
            prenom: cmd.prenom.clone(),
            correlation_id: cmd.get_correlation_id(),
            at: "whatever".to_string(),
            by: "whatever".to_string(),
        })
    }
}

pub struct EngineExemple {
    pub queue_sync: Arc<dyn QueueSynchronizer<ExCmd, ExResultRecord>>,
    pub read_write_consumer: Arc<dyn CanConsumeQueue>,
    pub result_consumer: Arc<dyn CanConsumeQueue>,
}

impl EngineExemple {
    pub fn new(handler: Box<dyn CanComputeCommand<ExCmd, ExResultRecord>>) -> Result<Self, String> {
        let producer_commands: Arc<dyn CanProduceInQueue<ExCmd>> = Arc::new(SimpleKafkaProducer::new("127.0.0.1:9092")?); // TODO : voir pour instancier un seul producer générique
        let producer_results: Arc<dyn CanProduceInQueue<ExResultRecord>> = Arc::new(SimpleKafkaProducer::new("127.0.0.1:9092")?);
        let subscriber: Arc<dyn CanSubscribe<ExResultRecord>> = Arc::new(Subscriber::new());

        let group_id_random = format!("exemple-engine-consumer-{}", Uuid::new_v4().to_string());

        let read_write_listener = Box::new(ListenerReadWrite {
            producer: producer_results.clone(),
            compute_cmd: handler,
            topic_result: "exemple-ontology-results".to_string(),
        });

        let listener_sync = Box::new(ListenerSynchronizer {
            subscriber: subscriber.clone(),
        });

        Ok(Self {
            queue_sync: Arc::new(QueueSynchronizerImpl {
                producer: producer_commands.clone(),
                subscriber: subscriber.clone(),
            }),
            read_write_consumer: Arc::new(SimpleKafkaConsumer::new(
                "exemple-ontology-commands",
                &group_id_random,
                read_write_listener,
            )?),
            result_consumer: Arc::new(SimpleKafkaConsumer::new(
                "exemple-ontology-results",
                &group_id_random,
                listener_sync,
            )?),
        })
    }

    pub fn start_listener(&self) -> Result<(), String> {

        let read_write_consumer = self.read_write_consumer.clone();
        let result_consumer = self.result_consumer.clone();

        debug!("thead starting");

        task::spawn(async move { read_write_consumer.consume().await });
        task::spawn(async move { result_consumer.consume().await });

        Self::temp_fix_for_waiting_thread_stated(Duration::from_secs(10)); // FIXME : à retirer lorsqu'on aura l'info que le listener est démarré
        Ok(())
    }
    pub async fn offer(&self, cmd: &ExCmd) -> Result<ExResultRecord, String> {
        let _correlation_id = "1234abcd"; // TODO faire un wrapper pour la command je pense qui va contenir la cmd + le correlation id ?

        debug!("offer : en attente de sync");
        self.queue_sync
            .wait_result(&cmd.get_correlation_id(), "exemple-ontology-commands", cmd, None)
            .await
    }

    #[deprecated]
    fn temp_fix_for_waiting_thread_stated(duration: Duration) -> () {
        sleep(duration)
    }
}
