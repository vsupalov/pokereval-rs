#![feature(test)]
extern crate test;

extern crate pokereval;
extern crate cards;
extern crate holdem;

use test::Bencher;

use cards::card::Card;
use cards::deck::Deck;
use holdem::{CactusKevCard};
use pokereval::eval_5cards;
use pokereval::original::{eval_5cards_kev_array, eval_5cards_kev};
use pokereval::utils::{card_to_deck_number};

#[bench]
fn bench_eval_card(b: &mut Bencher) {
    let mut deck = Deck::new_shuffled();

    let mut cards : Vec<Card> = Vec::new();

    for i in 0..52 {
        cards.push(deck.draw().ok().unwrap());
    }

    b.iter(|| {
        for i in 0..10 {
            let c1 = &cards[i*5 + 0];
            let c2 = &cards[i*5 + 1];
            let c3 = &cards[i*5 + 2];
            let c4 = &cards[i*5 + 3];
            let c5 = &cards[i*5 + 4];

            let subhand = [c1, c2, c3, c4, c5];
            eval_5cards(&subhand);
        }
        deck.reset_unshuffled();
    });
}


#[bench]
fn bench_eval_kev_array(b: &mut Bencher) {
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

            let subhand = [&c1, &c2, &c3, &c4, &c5];
            eval_5cards_kev_array(&subhand);
        }
    });
}

#[bench]
fn bench_eval_kev_array_no_creation(b: &mut Bencher) {
    let mut deck = Deck::new_shuffled();

    let mut cards :[CactusKevCard; 52] = [0; 52];

    for i in 0..52 {
        cards[i] = card_to_deck_number(&deck.draw().ok().unwrap());
    }

    let mut subhand = [&cards[0]; 5];

    b.iter(|| {
        for i in 0..10 {
            for j in 0..5 {
                subhand[j] = &cards[i*5 + j];
            }

            eval_5cards_kev_array(&subhand);
        }
    });
}

#[bench]
fn bench_eval_kev_no_array(b: &mut Bencher) {
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

            eval_5cards_kev(&c1, &c2, &c3, &c4, &c5);
        }
    });
}
