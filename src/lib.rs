extern crate cards;
extern crate holdem;

mod types;
mod lookups;
mod original;
//mod perfect;
pub mod utils;

use cards::card::{Card};
use holdem::{HandCards, CommunityCards};

use holdem::{HandRankClass};

pub type HandRank = u32;

pub fn eval_5cards(cards: [&Card; 5]) -> HandRank {
    original::eval_5cards(cards)
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

pub fn hand_rank_to_class(val: &HandRank) -> HandRankClass {
    match *val {
        x if x > 6185 => HandRankClass::HighCard,
        x if x > 3325 => HandRankClass::OnePair,
        x if x > 2467 => HandRankClass::TwoPair,
        x if x > 1609 => HandRankClass::ThreeOfAKind,
        x if x > 1599 => HandRankClass::Straight,
        x if x > 322  => HandRankClass::Flush,
        x if x > 166  => HandRankClass::FullHouse,
        x if x > 10   => HandRankClass::FourOfAKind,
        x if x > 0    => HandRankClass::StraightFlush,
        _             => panic!("Unexpected hand rank value!")
    }
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
