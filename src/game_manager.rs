
use crate::{banker::Banker, card::{Card, Deck}, hands::{best_hand, compare_hands, display_cards}};

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

        let mut i = 0;
        game_state.player_states.resize_with(self.num_players as usize, || {
            let t = PlayerState::new(i, [self.deck.draw(), self.deck.draw()]);
            i += 1;
            t
        });

        //Dealing Cards
        println!("Dealing Cards");
        for (i, player) in game_state.player_states.iter().enumerate() {
            println!("Player {}: {}", i, display_cards(&player.cards));
        }

        //Pre-Flop Bets
        self.bet_rounds();

        //Flop
        game_state.board.append(&mut [
            self.deck.draw(),
            self.deck.draw(),
            self.deck.draw()
        ].to_vec());

        println!("Flop: {}", display_cards(&game_state.board));

        //Flop Bets
        self.bet_rounds();


        //Turn
        game_state.board.push(self.deck.draw());
        println!("Turn: {}", display_cards(&game_state.board));

        //Turn Bets
        self.bet_rounds();


        //River
        game_state.board.push(self.deck.draw());
        println!("River: {}", display_cards(&game_state.board));

        //River Bets
        self.bet_rounds();

        
        //Showdown
        let mut states = game_state.player_states;
        states.sort_by(|a, d| {
            let board: [Card; 5]  = game_state.board.clone().try_into().unwrap();
            compare_hands(best_hand(&a.cards, board).0, best_hand(&d.cards, board).0)
        });

        let winner = states.last().unwrap();

        println!("Winner: Player {} with a {:?}", winner.id, best_hand(&winner.cards, game_state.board.clone().try_into().unwrap()).1)
    }

    pub fn bet_rounds(&mut self) {

    }
}



struct GameState {
    player_states: Vec<PlayerState>,
    board: Vec<Card>,
    banker: Banker
}

impl GameState {
    fn new(num_players: u32) -> Self {
        Self {
            player_states: Vec::with_capacity(num_players as usize),
            board: Vec::new(),
            banker: Banker::new(num_players, 100)
        }
    }
}


#[derive(Clone, Copy)]
pub struct PlayerState {
    id: u32,
    cards: [Card; 2]
}

impl PlayerState {
    pub fn new(id: u32, cards: [Card; 2]) -> Self {
        Self { id, cards }
    }
}
