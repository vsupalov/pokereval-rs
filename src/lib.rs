//! pokereval-rs currently contains a single way of evaluating poker hands (5,6 or 7 cards)
//! to a HandRank, which is a number from 0 to 7461 inclusive, the higher the better the hand.
//! Inside the modules, there are more efficient methods that don't need to convert cards
//! to internal representations first.

extern crate cards;
extern crate holdem;

mod lookups;
pub mod original;
//mod perfect;
pub mod utils;

use cards::card::{Card};
use holdem::{HandRank};
use utils::{card_to_deck_number};

/// Evalate a hand consisting of 5 cards. The cards are grouped in an array.
/// This is quite inefficient, due to the arrays that need to be created. But convenient.
pub fn eval_5cards(cards: &[&Card; 5]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);

    let converted_cards = [&c1, &c2, &c3, &c4, &c5];
    original::eval_5cards_kev_array(&converted_cards)
}

/// Evalate a hand consisting of 6 cards. The cards are grouped in an array.
/// This is quite inefficient, due to the arrays that need to be created. But convenient.
pub fn eval_6cards(cards: &[&Card; 6]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);
    let c6 = card_to_deck_number(cards[5]);

    let converted_cards = [&c1, &c2, &c3, &c4, &c5, &c6];
    original::eval_6cards_kev_array(&converted_cards)
}

/// Evalate a hand consisting of 7 cards. The cards are grouped in an array.
/// This is quite inefficient, due to the arrays that need to be created. But convenient.
pub fn eval_7cards(cards: &[&Card; 7]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);
    let c6 = card_to_deck_number(cards[5]);
    let c7 = card_to_deck_number(cards[6]);

    let converted_cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    original::eval_7cards_kev_array(&converted_cards)
}

//TODO: this will be relevant, once the "perfect hash" method works

//use cards::deck::{Deck};
//use pokereval::{original, perfect}; // two evaluation methods

/*
//TODO: as soon as both methods are expected to agree
// this guy does not always pass
#[test]
fn both_evaluation_methods_agree() {
    let mut deck = Deck::new();

    // try on 10 hands
    for _ in 0..10 {
        let c1 = deck.draw();
        let c2 = deck.draw();
        let c3 = deck.draw();
        let c4 = deck.draw();
        let c5 = deck.draw();

        let rank_original = original::eval_5cards([&c1, &c2, &c3, &c4, &c5]);
        let rank_perfect = perfect::eval_5cards([&c1, &c2, &c3, &c4, &c5]);
        assert_eq!(rank_original, rank_perfect);
    }
}
*/
