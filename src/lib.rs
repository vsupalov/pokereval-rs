extern crate cards;
extern crate holdem;

pub mod types;
mod lookups;
pub mod original;
//mod perfect;
pub mod utils;

use cards::card::{Card};
use holdem::{HandCards, CommunityCards};

use holdem::{HandRank};

pub fn eval_5cards(cards: [&Card; 5]) -> HandRank {
    original::eval_5cards(cards)
}

pub fn eval_6cards(cards: [&Card; 6]) -> HandRank {
    original::eval_6cards(cards)
}

pub fn eval_7cards(cards: [&Card; 7]) -> HandRank {
    original::eval_7cards(cards)
}

//TODO: is panic an acceptable behaviour here? Be more verbose about this function expecting all cards to be present?
pub fn eval_for_player(player_cards: &HandCards, community_cards: &CommunityCards) -> HandRank {
    let cards : [&Card; 7] = [
        &player_cards.0.expect_borrow(),
        &player_cards.1.expect_borrow(),
        &community_cards.0.expect_borrow(),
        &community_cards.1.expect_borrow(),
        &community_cards.2.expect_borrow(),
        &community_cards.3.expect_borrow(),
        &community_cards.4.expect_borrow()
    ];

    eval_7cards(cards)
}

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
