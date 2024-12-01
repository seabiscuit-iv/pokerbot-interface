use core::fmt;
use std::ops::Deref;

use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(EnumIter, Clone, Debug, PartialEq, PartialOrd, Eq, Copy)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen ,
    King,
    Ace
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        use Value::*;
        match value {
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven ,
            8 => Eight,
            9 => Nine,
            10 => Ten,
            11 => Jack,
            12 => Queen,
            13 => King,
            14 => Ace,
            _ => {panic!("ILLEGAL CARD VALUE TYPE DEFINED");}
        }
    }
}



impl Into<u32> for Value {
    fn into(self) -> u32 {
        (self as u32) + 2
    }
}



#[derive(EnumIter, Clone, Debug, PartialEq, PartialOrd, Eq, Copy)]
pub enum Suit {
    Club,
    Spade,
    Diamond,
    Heart
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub struct Card {
    pub value: Value,
    pub suit: Suit
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self {
            value,
            suit
        }
    }
}

impl From<(Value, Suit)> for Card {
    fn from(value: (Value, Suit)) -> Self {
        Self {
            value: value.0,
            suit: value.1
        }
    }
}

impl From<(u32, Suit)> for Card {
    fn from(value: (u32, Suit)) -> Self {
        (Into::<Value>::into(value.0), value.1).into()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num: u32 = self.value.into();
        let c: char = match num {
            2..=9 => char::from_digit(num, 10).unwrap(),
            10 => 'T',
            11 => 'J',
            12 => 'Q',
            13 => 'K',
            14 => 'A',
            _ => panic!("Error destructuring value of Card")
        };

        let suit: char = match self.suit {
            Suit::Club => 'C',
            Suit::Spade => 'S',
            Suit::Diamond => 'D',
            Suit::Heart => 'H',
        };

        write!(f, "{}-{}", c, suit)
    }
}



impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let m : u32 = self.value.clone().into();
        let th : u32 = other.value.clone().into();
        if m > th {
            std::cmp::Ordering::Greater
        } else if m == th {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Less
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

    pub fn _reset(&mut self) {
        *self = Deck::default()
    }

    pub fn shuffle(&mut self){
        self.0.shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self) -> Card {
        self.0.remove(0)
    }

    pub fn _count(&self) -> usize {
        self.0.len()
    }
}


impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}