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
}