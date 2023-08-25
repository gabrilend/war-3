use super::player::Player;
use super::card::Card;
use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

#[derive(Component)]
pub struct Game {
    players: Vec<Player>,
    play_area: Vec<Card>,
}

impl Game {
    pub fn new(players: Option<Vec<Player>>) -> Game {
        let players = players.unwrap_or_default();
        println!("Creating a new game with {} players", players.len());

        Game {
            players,
            play_area: vec![],
        }
    }

    fn play(&mut self) {
        self.deal();
        let winner = self.resolve();
        self.end();
    }

    fn deal(&mut self) {
        for player in self.players.iter() {
            // player.draw(1);
            // player.play(&mut self.play_area);
        }

    }
    
    fn resolve(&mut self) -> &Player {
        let mut highest: &Card = &self.play_area[0];
        for card in &self.play_area {
            if card.value > highest.value {
                highest = &card;
            }
            else if card.value == highest.value {
                self.war(4);
            }
        }
        // *highest.owner.unwrap()
        return &self.players[0];
    }

    fn war(&mut self, count: isize) {
        for _i in 0..count {
            self.deal();
        }
        self.resolve();
    }

    fn end(&mut self) {
        println!("Play area before distributing winnings: {:?}", self.play_area);
        for card in &self.play_area {
             // FIXME
        }
        println!("After distributing winnings: {:?}", self.play_area);
        return
    }
}
