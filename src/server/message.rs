use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub kind: String,
    pub topic: Option<String>,
    pub message: Option<Value>,
}