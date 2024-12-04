mod card;
mod hands;
mod banker;
mod pokerbot;
mod game_manager;
mod basicpokerbot;


use card::{Card, Suit, Value};
use game_manager::Game;
use pokerbot::PokerBot;

use basicpokerbot::BasicPokerBot;

fn main() {

    let mut bots: Vec<Box<dyn PokerBot>> = Vec::new();

    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));

    let mut game = Game::new(bots);

    (0..50).for_each(|i| {
        println!("Round {}", i);
        game.play_round();
        game.print_values();
    });

    // println!("COOKIE");
}



