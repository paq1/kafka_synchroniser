use crate::core::queue::datas::Data;
use serde::Serialize;


pub trait CanProduceInQueue {
    fn produce_data<T: Serialize>(
        &self,
        topic: &str,
        data: &Data<T>,
        key: Option<&str>
    ) -> Result<(), String>;
}
