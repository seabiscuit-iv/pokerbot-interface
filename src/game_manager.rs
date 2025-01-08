
use crate::{banker::{Banker, Response}, card::{Card, Deck}, hands::{best_hand, compare_hands, display_cards}, pokerbot::PokerBot};


enum ROUND {
    PREFLOP,
    FLOP,
    TURN,
    RIVER
}


pub struct Game {
    deck: Deck,
    num_players: u32,
    banker: Banker,
    players: Vec<Box<dyn PokerBot>>,
}

impl Game {
    pub fn new(players: Vec<Box<dyn PokerBot>>) -> Self{
        Self {
            deck: Deck::default(),
            num_players: players.len() as u32,
            banker: Banker::new(players.len() as u32, 1000),
            players,
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
        if self.bet_rounds(&mut game_state, self.num_players, ROUND::PREFLOP).inspect_err(|id| {
            self.banker.win(*id);

            println!("Winner: Player {}, all folds", id);
            return;
        }).is_err() {
            return;
        };
        println!("Pot is now {}\n", self.banker.pot);

        //Flop
        game_state.board.append(&mut [
            self.deck.draw(),
            self.deck.draw(),
            self.deck.draw()
        ].to_vec());

        println!("Flop: {}", display_cards(&game_state.board));

        //Flop Bets
        if self.bet_rounds(&mut game_state, self.num_players, ROUND::FLOP).inspect_err(|id| {
            self.banker.win(*id);

            println!("Winner: Player {}, all folds", id);
            return;
        }).is_err() {
            return;
        };
        println!("Pot is now {}\n", self.banker.pot);


        //Turn
        game_state.board.push(self.deck.draw());
        println!("Turn: {}", display_cards(&game_state.board));

        //Turn Bets
        if self.bet_rounds(&mut game_state, self.num_players, ROUND::TURN).inspect_err(|id| {
            self.banker.win(*id);

            println!("Winner: Player {}, all folds", id);
            return;
        }).is_err() {
            return;
        };
        println!("Pot is now {}\n", self.banker.pot);


        //River
        game_state.board.push(self.deck.draw());
        println!("River: {}", display_cards(&game_state.board));

        //River Bets
        if self.bet_rounds(&mut game_state, self.num_players, ROUND::RIVER).inspect_err(|id| {
            self.banker.win(*id);

            println!("Winner: Player {}, all folds", id);
            return;
        }).is_err() {
            return;
        };
        println!("Pot is now {}\n", self.banker.pot);
        
        //Showdown
        let mut states = game_state.player_states;
        states.sort_by(|a, d| {
            if !a.in_game {
                return std::cmp::Ordering::Less
            } else if !d.in_game {
                return std::cmp::Ordering::Greater
            }

            let board: [Card; 5]  = game_state.board.clone().try_into().unwrap();
            compare_hands(best_hand(&a.cards, board).0, best_hand(&d.cards, board).0)
        });

        let winner = states.last().unwrap();
        self.banker.win(winner.id);

        println!("Winner: Player {} with a {:?}\n", winner.id, best_hand(&winner.cards, game_state.board.clone().try_into().unwrap()).1);
    }

    fn bet_rounds(&mut self, game_state: &mut GameState, num_players: u32, round: ROUND) -> std::result::Result<u32, u32> {
        let mut turn: u32 = 0;
        let mut active_bet: u32 = 0;
        let mut bet_starter: u32 = 0;

        game_state.player_states.iter_mut().for_each(|p| {
            p.total_amt_bet = 0;
        });

        loop {
            if !game_state.player_states[turn as usize].in_game {
                turn += 1;
                turn = turn % num_players; 

                if bet_starter == turn {
                    break;
                } else {
                    continue;
                }
            }

            let resp = match round {
                ROUND::PREFLOP => self.players[turn as usize].preflop(0, &self.banker, &game_state.player_states[turn as usize]),
                ROUND::FLOP => self.players[turn as usize].flop(active_bet, &self.banker, &game_state.player_states[turn as usize], &(game_state.board)),
                ROUND::TURN => self.players[turn as usize].turn(active_bet, &self.banker, &game_state.player_states[turn as usize], &(game_state.board)),
                ROUND::RIVER => self.players[turn as usize].river(active_bet, &self.banker, &game_state.player_states[turn as usize], &(game_state.board)),
            };


            active_bet = match resp {
                Response::Raise(price) => {
                    if price <= active_bet {
                        panic!("Attempting to Raise less than or equal to active bet");
                    } else {
                        bet_starter = turn;
                        self.banker.bet(turn, price - game_state.player_states[turn as usize].total_amt_bet);
                        game_state.player_states[turn as usize].total_amt_bet = price;
                        println!("Player {} raises to {}", turn, price);
                        price
                    }
                },
                Response::Call => {
                    self.banker.bet(turn, active_bet - game_state.player_states[turn as usize].total_amt_bet);
                    game_state.player_states[turn as usize].total_amt_bet = active_bet;
                    if active_bet == 0 {
                        println!("Player {} checks", turn);
                    } else {
                        println!("Player {} calls", turn);
                    }
                    active_bet
                },
                Response::Fold => {
                    game_state.player_states[turn as usize].in_game = false;
                    println!("Player {} folds", turn);
                    active_bet
                },
            };

            turn += 1;
            turn = turn % num_players; 

            let x : Vec<&PlayerState> = game_state.player_states.iter().filter(|p| p.in_game).collect();

            if x.len() == 1 {
                return Err(x[0].id);
            }

            if bet_starter == turn {
                break;
            }
        };

        Ok(0)
    }

    pub fn print_values(&self) {
        println!("");

        for player in self.banker.money.iter().enumerate() {
            println!("Player {} has {}", player.0, player.1);
        }
        println!("");
    }


    pub fn get_player_money(&self, i: usize) -> u32{
        self.banker.money[i]
    }
}



struct GameState {
    player_states: Vec<PlayerState>,
    board: Vec<Card>
}

impl GameState {
    fn new(num_players: u32) -> Self {
        Self {
            player_states: Vec::with_capacity(num_players as usize),
            board: Vec::new()
        }
    }
}


#[derive(Clone, Copy)]
pub struct PlayerState {
    id: u32,
    pub cards: [Card; 2],
    in_game: bool,
    pub total_amt_bet: u32
}

impl PlayerState {
    pub fn new(id: u32, cards: [Card; 2]) -> Self {
        Self { id, cards, in_game: true, total_amt_bet: 0 }
    }
}
