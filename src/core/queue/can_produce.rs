use serde::de::DeserializeOwned;
use crate::core::queue::datas::Data;
use serde::{Deserialize, Serialize};


pub trait CanProduceInQueue {
    fn produce_data<'a, T: Serialize>(
        &self,
        topic: &str,
        data: &Data<T>,
        key: Option<&str>
    ) -> Result<(), String>;
}
