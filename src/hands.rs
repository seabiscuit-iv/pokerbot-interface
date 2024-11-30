use core::panic;

use crate::{card, Card, Suit, Value};

pub type Hand = [Card; 5];

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum HAND_TYPE {
    HighCard(Value),
    Pair(Value),
    TwoPair(Value, Value),
    Trips(Value),
    Straight(Value), //Highest Card
    Flush(Value, Suit), //Highest Card, Suit
    FullHouse(Value, Value), // (trips, pair)
    Quads(Value),
    StraightFlush(Value, Suit) // highest card, suit
}


impl Into<u32> for HAND_TYPE {
    fn into(self) -> u32 {
        use HAND_TYPE::*;

        match self {
            HighCard(_) => 1,
            Pair(_) => 2,
            TwoPair(_, _) => 3,
            Trips(_) => 4,
            Straight(_) => 5,
            Flush(_, _) => 6,
            FullHouse(_, _) => 7,
            Quads(_) => 8,
            StraightFlush(_, _) => 9,
        }
    }
} 


impl std::cmp::Ord for HAND_TYPE {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use HAND_TYPE::*;

        let att_str: u32 = self.clone().into();
        let def_str: u32 = self.clone().into();

        if att_str > def_str {
            return std::cmp::Ordering::Greater
        }
        else if att_str < def_str {
            return std::cmp::Ordering::Less
        } else {
            match (self, other) {
                (HighCard(a), HighCard(d)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type HighCard")
                },
                (Pair(a), Pair(d)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type Pair")
                },
                (TwoPair(a_1, a_2), TwoPair(d_1, d_2)) => {
                    match a_1.partial_cmp(d_1).expect("Error comparing hands of type TwoPair, first pair") {
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {
                            a_2.partial_cmp(d_2).expect("Error comparing hands of type TwoPair, second pair")
                        },
                    }
                },
                (Trips(a), Trips(d)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type Trips")
                },
                (Straight(a), Straight(d)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type Straight")
                },
                (Flush(a, _), Flush(d, _)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type Flush")
                },
                (FullHouse(a_1, a_2), FullHouse(d_1, d_2)) => {
                    match a_1.partial_cmp(d_1).expect("Error comparing hands of type FullHouse, Trips") {
                        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => {
                            a_2.partial_cmp(d_2).expect("Error comparing hands of type FullHouse, Pair")
                        },
                    }
                },
                (Quads(a), Quads(d)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type Quads")
                },
                (StraightFlush(a, _), StraightFlush(d, _)) => {
                    a.partial_cmp(d).expect("Error comparing hands of type StraightFlush")
                },
                _ => panic!("Error Comparing Hands, Unreachable State")
            }
        }
    }
}



pub fn get_hand_type(hand: Hand) -> HAND_TYPE {
    use HAND_TYPE::*;
    use Value::*;
    use Suit::*;

    let mut hand = hand;

    hand.sort();

    let mut hand_type = HighCard(hand[4].value.clone());

    for (i, card) in hand.iter().enumerate().rev() {
        if i == 0 {
            break;
        }

        if card.value == hand[i-1].value {
            hand_type = Pair(hand[i].value.clone());
            break;
        }
    }

    if let Pair(value) = hand_type.clone() {     
        for (i, card) in hand.iter().enumerate().rev() {
            if i == 0 {
                break;
            }

            if card.value == value {
                continue;
            }

            if card.value == hand[i-1].value {
                hand_type = TwoPair(value, card.value.clone());
                break;
            }
        }
    }
    

    for (i, card) in hand.iter().enumerate().rev() {
        if i == 0 || i == 1 {
            break;
        }

        if card.value == hand[i-1].value && card.value == hand[i-2].value {
            hand_type = Trips(hand[i].value.clone());
            break;
        }
    }


    // STRAIGHT
    for (i, card) in hand.iter().enumerate() {
        if i >= 4 {
            hand_type = Straight(card.value.clone());
            break;
        }

        if Into::<u32>::into(card.value.clone()) != Into::<u32>::into(hand[i+1].value.clone()) - 1 {
            break;
        }
    }
    

    // FLUSH
    let first_suit = hand[0].suit;
    let mut flush = true;

    for card in hand.iter() {
        if card.suit != first_suit {
            flush = false;
            break;
        }
    }

    if flush {
        hand_type = Flush(hand[4].value, first_suit);
    }



    // FULL HOUSE
    for (i, card) in hand.iter().enumerate().rev() {
        if i == 0 || i == 1 {
            break;
        }

        if card.value == hand[i-1].value && card.value == hand[i-2].value {
            let value = card.value;
            //PAIR
            for (i, card) in hand.iter().enumerate().rev() {
                if i == 0 {
                    break;
                }
        
                if card.value == hand[i-1].value && card.value != value {
                    hand_type = FullHouse(value, card.value);
                    break;
                }
            }

            break;
        }
    }
    
    // QUADS    
    for (i, card) in hand.iter().enumerate().rev() {
        if i == 0 || i == 1 || i == 2 {
            break;
        }

        if card.value == hand[i-1].value && card.value == hand[i-2].value && card.value == hand[i-3].value {
            hand_type = Quads(hand[i].value);
            break;
        }
    }



    //STRAIGHT FLUSH
    for (i, card) in hand.iter().enumerate() {
        if i >= 4 {
            let first_suit = hand[0].suit;
            let mut flush = true;

            for card in hand.iter() {
                if card.suit != first_suit {
                    flush = false;
                    break;
                }
            }

            if flush {
                hand_type = StraightFlush(hand[4].value, first_suit);
            }
            break;
        }

        if Into::<u32>::into(card.value.clone()) != Into::<u32>::into(hand[i+1].value.clone()) - 1 {
            break;
        }
    }


    hand_type
}
