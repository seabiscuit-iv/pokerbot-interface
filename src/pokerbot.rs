use itertools::Itertools;

use crate::{banker::{Banker, Response}, card::Card, game_manager::PlayerState, hands::{get_hand_type, Hand, HandType}};

pub trait PokerBot {
    fn flop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
    fn turn(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
    fn river(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
    fn preflop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState) -> Response;
    fn observe(&self, player_id: u32, bet: u32);
}
