# PokerBot Interface
A Rust interface for creating and testing different types of Poker algorithms, or PokerBots.

**Still in early development, unstable**

## Files
- `main.rs` - Main executable file
- `pokerbot.rs` - Holds the `Pokerbot` trait for creating bots
- `card.rs` - The struct and helper functions for the `Card`, `Value`, `Suit`, and `Deck` types
- `hands.rs` - Types and functions for recognizing and comparing generic poker hands
- `game_manager.rs` - The main poker game logic, as well as structs for holding and passing player data to PokerBots
- `banker.rs` - A helper object used by `game_manager.rs` for managing money
- `basicpokerbot.rs` - A generic PokerBot that follows simple logic showcasing how to use the `Pokerbot` trait and functions from `hands.rs`

## Usage
To create a bot, create a struct that implements the `Pokerbot` trait. This trait requires four straightforward functions: `preflop`, `flop`, `turn`, `river`, and `observe`(unused).

Each of these functions will be called in their respective rounds of the game. Note that one of these functions may be called multiple times in the case that your bot is raised, and must make an additional move. 

The `preflop`, `flop`, `turn`, and `river` functions all have the same signature:

```rust
fn flop(&self, active_bet: u32, bank: &Banker, player_state: &PlayerState, board: &Vec<Card>) -> Response;
```

- `active_bet: u32` - This represents the current active bet. Your bot is expected to either call this bet, raise the active bet, or fold the hand. 
- `bank: &Banker` - An object that holds money information about all the players at the table. Reflects an actual poker game, where everyone's balance at the table is visible, from which decisions can be made.
- `player_state: &PlayerState` - An object that contains data about the "Player" the Pokerbot is representing. Currently only stores the Player's cards and the total amount they've bet that round.
- `board: &Vec<Card>` - The cards on the board. Three for flop, four for turn, five for river
- `Response` - An enum in `banker.rs` representing a response in poker. You can `Raise(x)`, `Call`, or `Fold`. Note that to check, you just need to call an active bet of 0, even if your bot is the first to act. 

**Note: Your bot cannot raise a value less than or equal to the active bet. If this happens, the program will panic.** 

`Observe` is a function currently **unused**, but will eventually be called on every response for every bot at the table except the one responding. The goal is to give bots the ability to potentially store data about player behavior and use it in decision making.

After creating a Pokerbot object with the trait, you can add it to the table in `main.rs`. Since a Pokerbot's size cannot be determined at compile, it must be boxed and pushed to the heap, like `Box::new(BasicPokerBot)`. A game can then be instantiated by calling `let game = Game::new(bots : Vec<Box<dyn PokerBot>>)`. Game rounds can then be simulated by calling `game.play_round()`, and the banker values can be printed with `game.print_values()`. Example code for this is already included in `main.rs`

## TODO
Features that need to be implemented 

- [ ] **All In:** Logic to handle bots going all in, or if they do not have enough money to call the active bet. 
- [ ] **Variable Starting Money:** Currently all bots start with 8000. This needs to be refactored to be controlled from `main.rs`
- [x] **EGUI Graphing and Local UI**
- [ ] **Dynamic Money Distribution:** Logic for chopping or handling side pots.