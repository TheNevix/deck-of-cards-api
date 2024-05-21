use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Deck {
    pub deck_id: String,
    pub success: bool,
    pub shuffled: bool,
    pub remaining: u32,
}