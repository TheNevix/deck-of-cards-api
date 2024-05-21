use crate::models::enums::{Suit, Value};

#[derive(Debug)]
pub struct DeckCard {
    pub value: Value,
    pub suit: Suit
}

impl DeckCard {
    pub fn to_code(&self) -> String {
        let value_code = match self.value {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "0",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        };

        let suit_code = match self.suit {
            Suit::Spades => "S",
            Suit::Diamonds => "D",
            Suit::Clubs => "C",
            Suit::Hearts => "H",
        };

        format!("{}{}", value_code, suit_code)
    }
}