
use crate::{banker::Banker, card::{Card, Deck}, hands::display_cards};

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
        self.deck.reset();
        self.deck.shuffle();

        let mut game_state = GameState::new(self.num_players);

        game_state.player_states.resize_with(self.num_players as usize, || PlayerState::new([self.deck.draw(), self.deck.draw()]));

        for (i, player) in game_state.player_states.iter().enumerate() {
            println!("Player {}: {}", i, display_cards(&player.cards));
        }
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


#[derive(Clone, Copy)]
pub struct PlayerState {
    cards: [Card; 2]
}

impl PlayerState {
    pub fn new(cards: [Card; 2]) -> Self {
        Self { cards }
    }
}
