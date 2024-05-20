use reqwest::Error;
use crate::models::Deck;

pub struct DeckOfCardsClient {
    base_url: String,
    deck_count: u32,
    shuffle: bool,
}

impl DeckOfCardsClient {
    pub fn new() -> Self {
        DeckOfCardsClient {
            base_url: "https://deckofcardsapi.com/api/deck".to_string(),
            deck_count: 1,
            shuffle: true,
        }
    }

    pub fn deck_count(mut self, count: u32) -> Self {
        self.deck_count = count;
        self
    }

    pub fn shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    pub async fn create_deck(&self) -> Result<Deck, Error> {
        let url = format!(
            "{}/new/{}?deck_count={}",
            self.base_url,
            if self.shuffle { "shuffle" } else { "" },
            self.deck_count
        );
        let response = reqwest::get(&url).await?;
        let deck: Deck = response.json().await?;
        Ok(deck)
    }
}
