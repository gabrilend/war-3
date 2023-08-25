use super::card::Card;
use bevy::prelude::*;
use rand;

#[derive(Debug, Clone, Component)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub discard: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            cards: Vec::new(),
            discard: Vec::new(),
        }
    }

    pub fn shuffle(&mut self) {
        for i in 0..self.cards.len() {
            let j = rand::random::<usize>() % self.cards.len();
            self.cards.swap(i, j);
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        if self.cards.len() == 0 {
            self.cards = self.discard.clone();
            self.discard = Vec::new();
            self.shuffle();
        }

        return self.cards.pop()
    }

    pub fn discard(&mut self, card: Card) {
        self.discard.push(card);
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_many(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.add(card);
        }
    }
}

