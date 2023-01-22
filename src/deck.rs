use crate::Card;
use crate::Rank;
use crate::Suit;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

pub struct Deck {
    // vectors are basically arrays in rust it is not an actual math vector they just have
    // a non-fixed size and contents are allocated in heap (better way to access things in memory
    // cause rust is a system language)
    cards: Vec<Card>,
}

impl Deck {
    // btw rust has something called a borrow checker where it checks if things are mutable or
    // immutable i.e. they can be changed from their original value or not so it's important to
    // specify something as mutable or immutable to other functions (&mut means other functions
    // can mutate it)
    pub fn add_card(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit.clone(), rank.clone()))
            }
        }

        cards.push(Card::new(Suit::Clubs, Rank::Ace));
        Self { cards }
    }

    pub fn get_card(&mut self) -> Card {
        let card = self.cards.pop();
        match card {
            None => panic!("Not enough cards in deck"),
            Some(card) => card,
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        &self.cards.shuffle(&mut rng);
    }
}
