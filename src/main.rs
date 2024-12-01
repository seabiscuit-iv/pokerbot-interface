mod card;
mod hands;
mod banker;
mod pokerbot;
mod game_manager;


use card::{Card, Deck, Suit, Value};
use hands::{best_hand_varsize, compare_hands, display_cards};

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

    println!("Cards Dealt");
    println!("Player 1: {}", display_cards(&player_one));
    println!("Player 2: {}\n", display_cards(&player_two));


    let mut board: Vec<Card> = vec![
        deck.draw(),
        deck.draw(),
        deck.draw()
    ];

    //FLOP
    println!("Flop: {}", display_cards(&board));

    let best_hand_one = best_hand_varsize(player_one.iter().chain(board.iter()).map(|x| *x).collect());
    let best_hand_two = best_hand_varsize(player_two.iter().chain(board.iter()).map(|x| *x).collect());

    println!("Player 1 has a {:?}", best_hand_one.1);
    println!("Player 2 has a {:?}\n", best_hand_two.1);


    //TURN
    board.push(deck.draw());
    println!("Turn: {}", display_cards(&board));

    let best_hand_one = best_hand_varsize(player_one.iter().chain(board.iter()).map(|x| *x).collect());
    let best_hand_two = best_hand_varsize(player_two.iter().chain(board.iter()).map(|x| *x).collect());

    println!("Player 1 has a {:?}", best_hand_one.1);
    println!("Player 2 has a {:?}\n", best_hand_two.1);


    //River
    board.push(deck.draw());
    println!("River: {}", display_cards(&board));

    let best_hand_one = best_hand_varsize(player_one.iter().chain(board.iter()).map(|x| *x).collect());
    let best_hand_two = best_hand_varsize(player_two.iter().chain(board.iter()).map(|x| *x).collect());

    println!("Player 1 has a {:?}", best_hand_one.1);
    println!("Player 2 has a {:?}", best_hand_two.1);
    
    match compare_hands(best_hand_one.0, best_hand_two.0) {
        std::cmp::Ordering::Less => println!("Player Two Wins"),
        std::cmp::Ordering::Greater => println!("Player One Wins"),
        std::cmp::Ordering::Equal => println!("Tie, Chop"),
    }

}



