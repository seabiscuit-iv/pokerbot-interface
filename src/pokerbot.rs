use crate::{banker::{Banker, Response}, card::Card, game_manager::PlayerState};

pub trait PokerBot {
    fn flop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
    fn turn(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
    fn river(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
    fn preflop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState) -> Response;
    fn observe(&self, player_id: u32, bet: u32);
}


pub struct BasicPokerBot;

impl PokerBot for BasicPokerBot {

    fn observe(&self, player_id: u32, bet: u32) {
        
    }
    
    fn preflop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState) -> Response {
        if active_bet == 0 {
            Response::Call
        } else {
            Response::Fold
        }
    }

    fn flop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        if active_bet > 10 {
            Response::Fold
        } else if active_bet < 5 {
            Response::Raise(5)
        } else {
            Response::Call
        }
    }

    fn turn(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        if active_bet > 10 {
            Response::Fold
        } else if active_bet < 5 {
            Response::Raise(5)
        } else {
            Response::Call
        }
    }

    fn river(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        if active_bet > 10 {
            Response::Fold
        } else if active_bet < 5 {
            Response::Raise(5)
        } else {
            Response::Call
        }
    }
}