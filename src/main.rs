mod vm;
mod cards {
    pub mod card;
    pub mod deck;
    pub mod game;
    pub mod player;
    pub mod deck_systems;
}
mod map;
mod traits;
mod personality;
mod ui;
mod utilities;

use cards::game::Game as Game;
use cards::player::{PlayerName, PlayerDeck, PlayerHand, PlayerValue};
use cards::player::PlayerBundle;
use cards::deck::Deck;
use crate::ui::character_sheet::setup_ui;
use crate::cards::deck_systems::CardDrawEvent;
use crate::cards::deck_systems::trigger_draw_system;
use crate::cards::deck_systems::draw_card_system;
use crate::cards::deck_systems::build_decks_system;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupBevy)
        .add_plugin(SetupWar)
        .run();
}

pub struct SetupBevy;
impl Plugin for SetupBevy {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_egui::EguiPlugin)
           .add_startup_system(setup_ui);
    }
}

pub struct SetupWar;
impl Plugin for SetupWar {
    fn build(&self, app: &mut App) {
        app.add_system(trigger_draw_system)
           .add_system(draw_card_system)
           .add_event::<CardDrawEvent>()
           .add_startup_system(setup_game)
           .add_startup_system(build_decks_system);
    }
}

fn setup_game(mut commands: Commands) {
    let game = Game::new(None);

    let player1 = PlayerBundle {
        name: PlayerName("Player 1".to_string()),
        deck: PlayerDeck(Deck::new()),
        hand: PlayerHand(Vec::new()),
        value: PlayerValue(10),
    };

    let player2 = PlayerBundle {
        name: PlayerName("Player 2".to_string()),
        deck: PlayerDeck(Deck::new()),
        hand: PlayerHand(Vec::new()),
        value: PlayerValue(10),
    };

    commands.spawn(game);
    commands.spawn(player1);
    commands.spawn(player2);


}

/*
fn setup_game(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
*/
