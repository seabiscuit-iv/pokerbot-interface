use crate::{banker::{Banker, Response}, card::Card, game_manager::PlayerState, hands::{best_hand_varsize, get_hand_type, Hand, HandType}, pokerbot::PokerBot};


pub struct TUIPokerBot;

fn next_int() -> u32{
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Resp: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let num: u32 = s.parse().expect("Illegal Argument Supplied");
    num
}

fn next_char() -> char{
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Resp: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let c: char = s.parse().expect("Illegal Argument Supplied");
    c
}

impl PokerBot for TUIPokerBot {

    fn observe(&self, player_id: u32, bet: u32) {
        
    }
    
    fn preflop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState) -> Response {
        println!("\nActive bet is {active_bet}\n");
        let c = next_char();

        match c {
            'c' => { Response::Call },
            'f' => { Response::Fold },
            'r' => { 
                let bet = next_int();
                Response::Raise(bet)
            },
            _ => panic!("Illegal Char Supplied")
        }
        // Response::Raise(400)
    }

    fn flop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        println!("\nActive bet is {active_bet}\n");
        let c = next_char();

        match c {
            'c' => { Response::Call },
            'f' => { Response::Fold },
            'r' => { 
                let bet = next_int();
                Response::Raise(bet)
            },
            _ => panic!("Illegal Char Supplied")
        }
        
        // Response::Raise(400)
    }

    fn turn(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        println!("\nActive bet is {active_bet}\n");
        let c = next_char();

        match c {
            'c' => { Response::Call },
            'f' => { Response::Fold },
            'r' => { 
                let bet = next_int();
                Response::Raise(bet)
            },
            _ => panic!("Illegal Char Supplied")
        }
    }

    fn river(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response {
        println!("\nActive bet is {active_bet}\n");
        let c = next_char();

        match c {
            'c' => { Response::Call },
            'f' => { Response::Fold },
            'r' => { 
                let bet = next_int();
                Response::Raise(bet)
            },
            _ => panic!("Illegal Char Supplied")
        }
    }
}