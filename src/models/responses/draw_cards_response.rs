use serde::{Deserialize, Serialize};
use crate::models::Card;

#[derive(Debug, Deserialize, Serialize)]
pub struct DrawCardsResponse {
    pub success: bool,
    pub deck_id: String,
    pub cards: Vec<Card>,
    pub remaining: i32
}