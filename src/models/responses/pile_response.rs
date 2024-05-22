use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::Pile;

#[derive(Debug, Deserialize, Serialize)]
pub struct PileResponse {
    pub success: bool,
    pub deck_id: String,
    pub remaining: i32,
    pub piles: HashMap<String, Pile>
}