use reqwest::Error;
use crate::models::{Deck, DrawCardsResponse};

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

    /// Sets the deck count to the number that has been passed.
    /// 
    /// # Parameters
    /// - `count`: Amount of decks.
    pub fn deck_count(mut self, count: u32) -> Self {
        self.deck_count = count;
        self
    }

    /// Sets if the decks need to be shuffled
    /// 
    /// # Parameters
    /// - `shuffle`: Set shuffle to true or false.
    pub fn shuffle(mut self, shuffle: bool) -> Self {
        self.shuffle = shuffle;
        self
    }

    /// Creates a new deck.
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

    /// Draws a certain amount of cards from a deck
    /// 
    /// # Parameters
    /// - `amount`: Amount of cards to draw.
    /// - `deck_id`: The id of the deck.
    pub async fn draw_cards(&self, amount: i16, deck_id: String) -> Result<DrawCardsResponse, Error> {
        let url = format!(
            "{}/{}/draw/?count={}",
            self.base_url,
            deck_id,
            amount
        );
        let response = reqwest::get(&url).await?;
        let draw_card_response: DrawCardsResponse = response.json().await?;
        Ok(draw_card_response)
    }




}
