use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContactRequest {
    pub name: String,
    pub company: String,
    pub email: String,
    pub service: String,
    pub message: String,
}