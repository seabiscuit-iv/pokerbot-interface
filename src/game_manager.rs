use core::num;

use crate::{banker::Banker, card::{Card, Deck}};

pub struct Game {
    deck: Deck,
    num_players: u32
}

impl Game {
    pub fn new(num_players: u32) -> Self{
        Self {
            deck: Deck::default(),
            num_players
        }
    }

    pub fn play_round(&mut self) {
        self.deck.shuffle();

    }
}



struct GameState {
    player_states: Vec<PlayerState>,
    banker: Banker
}

impl GameState {
    fn new(num_players: u32) -> Self {
        Self {
            player_states: Vec::with_capacity(num_players as usize),
            banker: Banker::new(num_players, 100)
        }
    }
}


pub struct PlayerState {
    cards: [Card; 2]
}

impl PlayerState {
    pub fn new(cards: [Card; 2]) -> Self {
        Self { cards }
    }
}
