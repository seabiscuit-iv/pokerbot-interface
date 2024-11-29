mod resources;
mod poker;

use poker::{Card, Deck, Suit, Value};
use rand::{seq::SliceRandom, thread_rng};
use Suit::*;
use Value::*;

fn main() {
    let mut deck = Deck::default();
    deck.0.shuffle(&mut thread_rng());

    println!("{:?}", deck.0.pop().unwrap());
    println!("{:?}", deck.0.pop().unwrap());
}
