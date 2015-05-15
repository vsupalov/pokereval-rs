extern crate cards;
extern crate holdem;

mod types;
mod lookups;
mod original;
mod perfect;
pub mod utils;

use cards::card::{Card};
use holdem::{HandCards, CommunityCards};

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
