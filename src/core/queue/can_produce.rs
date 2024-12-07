use crate::core::queue::datas::Data;
use serde::Serialize;


pub trait CanProduceInQueue<T>: Send + Sync
where
    T: Serialize
{
    fn produce_data(
        &self,
        topic: &str,
        data: &Data<T>,
        key: Option<&str>
    ) -> Result<(), String>;
}
