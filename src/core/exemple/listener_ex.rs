use crate::core::queue::datas::Data;
use crate::core::queue::listener::Listener;
use async_trait::async_trait;

pub struct CmdTestListener {}

#[async_trait]
impl Listener<Data<String>> for CmdTestListener {
    async fn on_message(&self, message: &Data<String>, _key: Option<&str>) -> Result<(), String> {
        println!("payload object {message:?}");
        Ok(())
    }
}