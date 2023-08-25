use super::deck::Deck;
use super::player::Player;
use crate::cards::card::Card;
use crate::vm::instruction::Instruction::InstLiteral;
use crate::vm::instruction::Instruction::InstSetValue;
use bevy::prelude::*;

pub struct CardDrawEvent {
    pub player_entity: Entity,
}

pub fn trigger_draw_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<CardDrawEvent>,
    player_query: Query<Entity, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for player_entity in player_query.iter() {
            event_writer.send(CardDrawEvent {
                player_entity,
            });
        }
    }
}

pub fn draw_card_system(
    mut event_reader: EventReader<CardDrawEvent>,
    mut deck_query: Query<&mut Deck>,
) {
    for event in event_reader.iter() {
        if let Ok(mut deck) = deck_query.get_mut(event.player_entity) {
            if let Some(card) = deck.draw() {
                println!("Drew card: {:?}", card);
            }
        }
    }
}

pub fn build_decks_system(mut commands: Commands,
                      mut players: Query<&mut Player>) {
        for (i, mut player) in players.iter_mut().enumerate() {
            for _ in 0..10 {
                // Convert `i` to a `char`, then to a `String`
                let card_name = std::char::from_u32(i as u32 + 'A' as u32)
                    .unwrap().to_string();
                println!("Adding card: {} to player {}", card_name, i);
                let card = Card::new(card_name,
                                     vec![InstLiteral(0).to_u8(),
                                          i as u8,
                                          InstSetValue(0).to_u8()],
                                     3);
                player.deck.add(card);
        }
    }
}
