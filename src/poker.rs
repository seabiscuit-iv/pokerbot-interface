use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(EnumIter, Clone, Debug)]
pub enum Value {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        use Value::*;
        match value {
            2 => TWO,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGHT,
            9 => NINE,
            10 => TEN,
            11 => JACK,
            12 => QUEEN,
            13 => KING,
            14 => ACE,
            _ => {panic!("ILLEGAL CARD VALUE TYPE DEFINED");}
        }
    }
}



#[derive(EnumIter, Clone, Debug)]
pub enum Suit {
    CLUB,
    SPADE,
    DIAMOND,
    HEART
}

#[derive(Debug)]
pub struct Card {
    pub value: Value,
    pub suit: Suit
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self  {
        Self {
            value,
            suit
        }
    }
}



#[derive(Debug)]
pub struct Deck(pub Vec<Card>);

impl Deck {
    pub fn default() -> Self {
        Self (
            Suit::iter().map(|suit| {
                Value::iter().map(move |value| {
                    Card::new(value.clone(), suit.clone())
                })
            }).flatten().collect::<Vec<Card>>()
        )
    }
}