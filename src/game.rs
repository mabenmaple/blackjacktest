use super::*;
use card::*;
use std::fmt;

pub fn deal_players(deck: &mut Deck, dealer: &mut dyn Person, player: &mut dyn Person) {
    player.deal_card(deck.get_card());
    dealer.deal_card(deck.get_card());
    player.deal_card(deck.get_card());
}

pub fn get_score(player: & dyn Person) -> Score {
    let hand = &player.get_hand();
    // this is just basically making an integer but its 8 bit and unsigned
    let mut result: u8 = 0;
    for card in hand.iter() {
        match card.rank {
            Rank::Knight => result += 10,
            Rank::Queen => result += 10,
            Rank::King => result += 10,
            Rank::Ace => result += 1,
            x => result += x as u8,
        }
    }

    for card in hand.iter() {
        if let Rank::Ace = card.rank {
            if result + 10 <= 21{
                result += 10;
            }
        }
    }
    
    if result > 21 {
        return Score::Busted;
    }

    if result == 21 && hand.len() == 2{
        return Score::Blackjack
    }

    Score::Points(result)
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Score {
    Busted,
    Points(u8),
    Blackjack,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Score::Blackjack => write!(f, "{:?}", &self),
            Score::Busted => write!(f, "{:?}", &self),
            Score::Points(num) => write!(f, "{}", &num),
        }
    }
}

// the 'a basically means for the lifetime of a, so each variable here has a shared lifetime i.e.
// u want a struct Person to only 'live' as long as its referenced object to stop memory leak
// n shit
pub fn get_winner<'a>(dealer: &'a dyn Person, player: &'a dyn Person) -> Option<&'a dyn Person> {
    let player_score = get_score(player); // Get Scores
    let dealer_score = get_score(dealer);
    if player_score == dealer_score {
        if dealer_score > Score::Points(16) && dealer_score != Score::Blackjack {
            return Some(dealer); // dealer wins from 17 and up
        } else {
            return None;
        }
    };

    if player_score > dealer_score {
        return Some(player);
    } else {
        return Some(dealer) // im being lazy cause rust handles all this shit in the compiler but
        // in python put guard statements n shit
    }
}

#[test]
fn queen_and_ace_is_blackjack() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clubs, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Clubs, Rank::Queen));
    assert_eq!(Score::Blackjack, get_score(&mut player));
}

#[test]
fn seven_and_ace_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clubs, Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    assert_eq!(Score::Points(18), get_score(&mut player));
}

#[test]
fn seven_ace_and_knight_is18() {
    let mut player = Player::new("Test");
    player.deal_card(Card::new(card::Suit::Clubs, Rank::Seven));
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Diamonds, Rank::Knight));
    assert_eq!(Score::Points(18), get_score(&mut player));
}

#[test]
fn five_aces_is15() {
    let mut player = Player::new("Test");
    for _num in 0..5 {
        player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    }
    assert_eq!(Score::Points(15), get_score(&mut player));
}

#[test]
fn blackjack_wins_over_21() {
    let mut player = Player::new("Test");
    let mut dealer = Player::new("Test2");
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));

    dealer.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));
    dealer.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));
    dealer.deal_card(Card::new(card::Suit::Hearts, Rank::Ace));
    let winner = game::get_winner(&dealer, &player).unwrap();
    assert_eq!(player.get_name(), winner.get_name());
}

#[test]
fn aces_shrink_when_needed() {
    let mut player = Player::new("Player");
    player.deal_card(Card::new(card::Suit::Hearts, Rank::Ten));
    player.deal_card(Card::new(card::Suit::Clubs, Rank::Ace));
    player.deal_card(Card::new(card::Suit::Diamonds, Rank::Ace));
    let score = game::get_score(&player);
    assert_eq!(Score::Points(12), score);
}
