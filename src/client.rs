use reqwest::Error;
use crate::models::{responses::PileResponse, Deck, DeckCard, DrawCardsResponse};

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
    pub async fn draw_cards(&self, amount: i16, deck_id: &String) -> Result<DrawCardsResponse, Error> {
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

    /// Reshuffles a deck of cards.
    /// 
    /// # Parameters
    /// - `deck_id`: The id of the deck.
    /// - `remaining`: An optional parameter to only reshuffle the remaining cards, leaving drawn cards alone.
    pub async fn reshuffle_deck(&self, deck_id: &String, remaining: Option<bool>) -> Result<Deck, Error> {
        let url = match remaining {
            Some(rem) => {
                if rem {
                    format!("{}/{}/shuffle/?remaining=true", self.base_url, deck_id)
                } else {
                    format!("{}/{}/shuffle/?remaining=false", self.base_url, deck_id)
                }
            }
            None => format!("{}/{}/shuffle/", self.base_url, deck_id),
        };

        let response = reqwest::get(&url).await?;
        let reshuffle_deck: Deck = response.json().await?;
        Ok(reshuffle_deck)
    }

    /// Creates a brand new deck.
    /// 
    /// # Parameters
    /// - `jokers`: An optional parameter to add two jokers to the deck.
    pub async fn brand_new_deck(&self, jokers: Option<bool>) -> Result<Deck, Error> {
        let url = match jokers {
            Some(joker) => {
                if joker {
                    format!("{}/new/?jokers_enabled=true", self.base_url)
                } else {
                    format!("{}/new/?jokers_enabled=false", self.base_url)
                }
            }
            None => format!("{}/new/", self.base_url),
        };

        let response = reqwest::get(&url).await?;
        let reshuffle_deck: Deck = response.json().await?;
        Ok(reshuffle_deck)
    }

    /// Creates a partial deck with a Vec of provided cards.
    /// 
    /// # Parameters
    /// - `cards`: A Vec of DeckCard.
    pub async fn create_partial_deck(&self, cards: Vec<DeckCard>) -> Result<Deck, Error> {
        let card_codes: Vec<String> = cards.iter().map(|card| card.to_code()).collect();
        let cards_param = card_codes.join(",");
        
        let url = format!(
            "{}/new/shuffle/?cards={}",
            self.base_url,
            cards_param
        );

        let response = reqwest::get(&url).await?;
        let reshuffle_deck: Deck = response.json().await?;
        Ok(reshuffle_deck)
    }

    /// Adds cards to a pile.
    /// 
    /// # Parameters
    /// - `deck_id`: The id of the deck
    /// - `pile_name`: The name of the pile.
    /// - `cards`: A Vec of DeckCard.
    pub async fn add_to_pile(&self, deck_id: &String, pile_name: String, cards: Vec<DeckCard>) -> Result<PileResponse, Error> {
        let card_codes: Vec<String> = cards.iter().map(|card| card.to_code()).collect();
        let cards_param = card_codes.join(",");
        
        let url = format!(
            "{}/{}/pile/{}/add/?cards={}",
            self.base_url,
            deck_id,
            pile_name,
            cards_param
        );

        let response = reqwest::get(&url).await?;
        let add_to_pile_response: PileResponse = response.json().await?;
        Ok(add_to_pile_response)
    }

    /// Shuffle a pile.
    /// 
    /// # Parameters
    /// - `deck_id`: The id of the deck
    /// - `pile_name`: The name of the pile.
    pub async fn shuffle_pile(&self, deck_id: &String, pile_name: String) -> Result<PileResponse, Error> {
        let url = format!(
            "{}/{}/pile/{}/shuffle/",
            self.base_url,
            deck_id,
            pile_name
        );

        let response = reqwest::get(&url).await?;
        let add_to_pile_response: PileResponse = response.json().await?;
        Ok(add_to_pile_response)
    }


}
