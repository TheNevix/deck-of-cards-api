use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Deck {
    pub deck_id: String,
    pub success: bool,
    pub shuffled: bool,
    pub remaining: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DrawCardsResponse {
    pub success: bool,
    pub deck_id: String,
    pub cards: Vec<Card>,
    pub remaining: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub code: String,
    pub image: String,
    pub images: Images,
    pub value: String,
    pub suit: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Images {
    pub svg: String,
    pub png: String
}