use crate::{banker::{Banker, Response}, card::Card, game_manager::PlayerState, hands::{best_hand_varsize, get_hand_type, Hand, HandType}, pokerbot::PokerBot};



pub struct BasicPokerBot;

impl PokerBot for BasicPokerBot {

    fn observe(&self, player_id: u32, bet: u32) {
        
    }
    
    fn preflop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState) -> Response {
        if active_bet <= 25 {
            Response::Call
        } else {
            Response::Fold
        }
    }

    fn flop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {

        let cards: Hand =  board.iter().chain(player_state.cards.iter()).map(|x| *x).collect::<Vec<Card>>().try_into().unwrap();
        let best_hand = get_hand_type(cards);

        if best_hand > HandType::HighCard(crate::card::Value::Ace) {      
            if active_bet > 50 {
                Response::Fold
            } else if active_bet < 25 {
                Response::Raise(25)
            } else {
                Response::Call
            }
        } else {
            if active_bet <= 0 {
                Response::Call
            } else {
                Response::Fold
            }
        }

    }

    fn turn(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {

        let cards=  board.iter().chain(player_state.cards.iter()).map(|x| *x).collect::<Vec<Card>>();
        let best_hand = best_hand_varsize(cards).1;

        if best_hand > HandType::Pair(crate::card::Value::Ten) {
            if active_bet < 100 {
                Response::Raise(100)
            } else {
                Response::Call
            }
        }
        else if best_hand > HandType::HighCard(crate::card::Value::Ace) {      
            if active_bet > 60 {
                Response::Fold
            } else if active_bet < 25 {
                Response::Raise(50)
            } else {
                Response::Call
            }
        }
        else {
            if active_bet <= 0 {
                Response::Call
            } else {
                Response::Fold
            }
        }

    }

    fn river(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        let cards=  board.iter().chain(player_state.cards.iter()).map(|x| *x).collect::<Vec<Card>>();
        let best_hand = best_hand_varsize(cards).1;

        if best_hand > HandType::HighCard(crate::card::Value::Ace) {      
            if active_bet > 100 {
                Response::Fold
            } else if active_bet < 25 {
                Response::Raise(25)
            } else {
                Response::Call
            }
        } else {
            if active_bet <= 0 {
                Response::Call
            } else {
                Response::Fold
            }
        }
    }
}