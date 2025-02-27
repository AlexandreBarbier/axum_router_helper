use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseBody<T> {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<T>,
}
