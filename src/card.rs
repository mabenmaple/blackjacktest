use std::fmt;
use strum_macros::EnumIter;

// basically lets any card and its children (suit rank) be cloned 
#[derive(Clone)]
// pub means anyone can use this structure 
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// impl defines the implementation of the card structure
impl Card {
    // creates new public function that allows Self (using -> notation) to access the elements of
    // the Card structure
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self {
            suit,
            rank,
        }
    }
}

impl fmt::Display for Card {
    // Lets the format print the suit and rank of a given card
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", &self.rank, &self.suit)
    }
}

// if u did c++ this is kinda like calling namespace std but instead of it being global (cringe)
// you call it locally for the public enum which is much more nicer and readable 
#[derive(EnumIter, Debug, PartialEq, Clone)]
#[allow(unused_variables, dead_code)]
pub enum Suit {
    Hearts, 
    Diamonds,
    Clubs,
    Spades
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
#[allow(unused_variables, dead_code)]
pub enum Rank {
    Ace, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, Eight = 8, Nine = 9, Ten = 10, Knight, Queen, King
}
