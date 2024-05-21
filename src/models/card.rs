use serde::{Deserialize, Serialize};
use crate::models::Images;

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub code: String,
    pub image: String,
    pub images: Images,
    pub value: String,
    pub suit: String
}