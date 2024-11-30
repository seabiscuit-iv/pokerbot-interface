mod resources;
mod card;
mod hands;

use card::{Card, Deck, Suit, Value};
use hands::get_hand_type;
use rand::{seq::SliceRandom, thread_rng};
use Suit::*;
use Value::*;

fn main() {
    let mut deck = Deck::default();
    deck.0.shuffle(&mut thread_rng());

    let hand : hands::Hand = [
        (Queen, Club).into(),
        (King, Club).into(),
        (Jack, Club).into(),
        (Five, Club).into(),
        (Ace, Club).into(),
    ];

    let t = get_hand_type(hand);
    println!("{:?}", t);
}
