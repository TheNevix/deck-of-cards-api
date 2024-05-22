use serde::{Deserialize, Serialize};
use crate::Card;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pile {
    pub cards: Option<Vec<Card>>,
    pub remaining: i32
}