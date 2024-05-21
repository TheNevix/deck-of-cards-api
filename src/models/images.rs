use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Images {
    pub svg: String,
    pub png: String
}