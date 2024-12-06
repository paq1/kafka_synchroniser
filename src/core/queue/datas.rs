use serde::{Deserialize, Serialize};

// TODO : Deserialize, Debug, Clone
#[derive(Serialize)]
pub struct Data<T: Serialize> {
    pub data: T,
}
