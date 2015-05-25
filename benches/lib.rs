#![feature(test)]
extern crate test;

extern crate pokereval;
extern crate cards;
extern crate holdem;

use test::Bencher;

use cards::deck::Deck;
use pokereval::original::{eval_5cards, eval_5cards_raw, eval_5cards_raw_unwrapped, eval_5cards_raw_unwrapped_behind_function};
use pokereval::utils::{card_to_deck_number};
use holdem::{CactusKevCard};

#[bench]
fn bench_card(b: &mut Bencher) {
    let mut deck = Deck::new_shuffled();

    b.iter(|| {
        for i in 0..10 {
            let c1 = deck.draw().ok().unwrap();
            let c2 = deck.draw().ok().unwrap();
            let c3 = deck.draw().ok().unwrap();
            let c4 = deck.draw().ok().unwrap();
            let c5 = deck.draw().ok().unwrap();

            eval_5cards([&c1, &c2, &c3, &c4, &c5]);
        }
        deck.reset_unshuffled();
    });
}


#[bench]
fn bench_card_raw(b: &mut Bencher) {
    let mut deck = Deck::new_shuffled();

    let mut cards :[CactusKevCard; 52] = [0; 52];

    for i in 0..52 {
        cards[i] = card_to_deck_number(&deck.draw().ok().unwrap());
    }

    b.iter(|| {
        for i in 0..10 {
            let c1 = cards[i*5 + 0];
            let c2 = cards[i*5 + 1];
            let c3 = cards[i*5 + 2];
            let c4 = cards[i*5 + 3];
            let c5 = cards[i*5 + 4];

            eval_5cards_raw([&c1, &c2, &c3, &c4, &c5]);
        }
    });
}

#[bench]
fn bench_card_raw_unwrapped(b: &mut Bencher) {
    let mut deck = Deck::new_shuffled();

    let mut cards :[CactusKevCard; 52] = [0; 52];

    for i in 0..52 {
        cards[i] = card_to_deck_number(&deck.draw().ok().unwrap());
    }

    b.iter(|| {
        for i in 0..10 {
            let c1 = cards[i*5 + 0];
            let c2 = cards[i*5 + 1];
            let c3 = cards[i*5 + 2];
            let c4 = cards[i*5 + 3];
            let c5 = cards[i*5 + 4];

            eval_5cards_raw_unwrapped(&c1, &c2, &c3, &c4, &c5);
        }
    });
}

#[bench]
fn bench_card_raw_unwrapped_behind_function(b: &mut Bencher) {
    let mut deck = Deck::new_shuffled();

    let mut cards :[CactusKevCard; 52] = [0; 52];

    for i in 0..52 {
        cards[i] = card_to_deck_number(&deck.draw().ok().unwrap());
    }

    b.iter(|| {
        for i in 0..10 {
            let c1 = cards[i*5 + 0];
            let c2 = cards[i*5 + 1];
            let c3 = cards[i*5 + 2];
            let c4 = cards[i*5 + 3];
            let c5 = cards[i*5 + 4];

            eval_5cards_raw_unwrapped_behind_function(&c1, &c2, &c3, &c4, &c5);
        }
    });
}
