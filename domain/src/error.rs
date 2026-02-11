use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub description: String,
    pub data: Value,
}

impl Error {
    pub fn new<T: Sized + Serialize>(
        code: impl Into<String>,
        description: impl Into<String>,
        data: T,
    ) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
            data: serde_json::to_value(data).unwrap(),
        }
    }
}

pub trait ErrorTrait {
    fn error<T: Sized + Serialize>(&self) -> Error;
}
