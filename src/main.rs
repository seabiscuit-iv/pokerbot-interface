mod resources;
mod card;
mod hands;

use card::{Card, Deck, Suit, Value};
use hands::{compare_hands, get_hand_type, Hand};
use Suit::*;
use Value::*;

fn main() {
    let mut deck = Deck::default();

    deck.shuffle();

    let player_one = [
        deck.draw(),
        deck.draw()
    ];

    let player_two = [
        deck.draw(),
        deck.draw()
    ];

    let board = [
        deck.draw(),
        deck.draw(),
        deck.draw(),
    ];

    let one_hand =
        player_one
        .iter()
        .chain(board.iter())
        .map(|x| *x)
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap();
    let two_hand =
        player_two
        .iter()
        .chain(board.iter())
        .map(|x| *x)
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap();

    let one_hand_type = get_hand_type(player_one
        .iter()
        .chain(board.iter())
        .map(|x| *x)
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap()
    );

    let two_hand_type = get_hand_type(player_two
        .iter()
        .chain(board.iter())
        .map(|x| *x)
        .collect::<Vec<Card>>()
        .try_into()
        .unwrap()
    );

    let winner = match compare_hands(one_hand, two_hand) {
        std::cmp::Ordering::Greater => "Player One Wins!",
        std::cmp::Ordering::Less => "Player Two Wins!",
        std::cmp::Ordering::Equal => "Tie, Chop!",
    };

    println!("Player 1: {}", player_one.iter().map(|s| format!("{}", s)).collect::<Vec<String>>().join(" "));
    println!("Player 2: {}", player_two.iter().map(|s| format!("{}", s)).collect::<Vec<String>>().join(" "));
    println!("Board: {}\n", board.iter().map(|s| format!("{}", s)).collect::<Vec<String>>().join(" "));
    println!("Player 1 draws {:?}", one_hand_type);
    println!("Player 2 draws {:?}", two_hand_type);
    println!("{}", winner);
}
