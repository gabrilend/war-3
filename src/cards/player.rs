use super::card::Card;
use super::deck::Deck;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub deck: PlayerDeck,
    pub hand: PlayerHand,
    pub value: PlayerValue,
}

#[derive(Debug, Component)]
pub struct PlayerName(pub String);

#[derive(Debug, Component)]
pub struct PlayerDeck(pub Deck);

#[derive(Debug, Component)]
pub struct PlayerHand(pub Vec<Card>);

#[derive(Debug, Component)]
pub struct PlayerValue(pub u8);

#[derive(Debug, Component)]
pub struct Player{
    pub name: String,
    pub deck: Deck,
    hand: Vec<Card>,
    pub value: u8,
}

impl Player{

    pub fn new(name: Option<&String>,
                    deck: Option<&Deck>,
                    value: u8) -> Player {
        let name = match name {
            Some(name) => name.clone(),
            None => String::from(""),
        };

        let deck = match deck {
            Some(deck) => deck.clone(),
            None => Deck::new(),
        };

        Player {
            name: name,
            deck: deck,
            hand: Vec::new(),
            value: value,
        }
    }

    pub fn draw(&mut self, count: usize) {
        for _i in 0..count {
            self.hand.push(self.deck.draw().unwrap());
        }
    }
    
    // This function first drains the hand and collects the cards into a vector.
    // Then it extends the play_area to hold the new vector of cards.
    pub fn play(&mut self, play_area: &mut Vec<Card>) {
        play_area.extend(self.hand.drain(..).collect::<Vec<Card>>());
    }
}

