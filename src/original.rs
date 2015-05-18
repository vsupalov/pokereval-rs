use std::collections::{HashMap};

use super::{lookups};
use super::{hand_rank};

use cards::card::{Card};
use cards::deck::{Deck};

use super::{HandRank};
use super::types::{InternalCardRepresentation};
use super::utils::{card_to_deck_number};

use holdem::{HandRankClass};

fn findit(key: usize) -> usize {
    let mut low = 0;
    let mut high = 4887;
    let mut mid;

    while low <= high {
        mid = (high+low) >> 1; // divide by two

        if key < lookups::PRODUCTS[mid] as usize {
            high = mid - 1;
        } else if key > lookups::PRODUCTS[mid] as usize {
            low = mid + 1;
        } else {
            return mid;
        }
    }

    panic!("No match found")
}

pub fn eval_5cards(cards: [&Card; 5]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);

    let q : usize = ((c1 | c2 | c3 | c4 | c5) as usize) >> 16;

    if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        return lookups::FLUSHES[q] as HandRank;
    }
    let s = lookups::UNIQUE_5[q] as HandRank;
    if s != 0 {
        return s;
    }

    let q = 
        ((c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff))
        as usize;
    let lookup = findit(q);
    lookups::VALUES[lookup] as HandRank
}

pub fn eval_7cards(cards: [&Card; 7]) -> HandRank {
    let mut tmp;
    let mut best = 9999;
    for ids in lookups::PERM_7.iter() {
        let subhand : [&Card; 5] = [
                cards[ids[0] as usize],
                cards[ids[1] as usize],
                cards[ids[2] as usize],
                cards[ids[3] as usize],
                cards[ids[4] as usize]
         ];

        tmp = eval_5cards(subhand);
        if tmp < best {
            best = tmp;
        }
    }

    best
}

#[test]
fn evaluate_all_possible_5_card_combinations() {
    let mut deck = Deck::new();
    let mut cards : Vec<Card> = Vec::new();

    // this could be made faster, by creating a function that works on raw-card-representations and translating
    // the deck cards into it
    for _ in 0..52 {
        let card = deck.draw();
        cards.push(card);
    }

    //let mut rank_class_count : HashMap<HandRankClass, usize> = HashMap::new();
    //let mut rank_count : HashMap<HandRank, bool> = HashMap::new();

    // 2,598,960 unique poker hands
    for i1 in 0..52 {
        for i2 in (i1+1)..52 {
            for i3 in (i2+1)..52 {
                for i4 in (i3+1)..52 {
                    for i5 in (i4+1)..52 {
                        let c1 = &cards[i1];
                        let c2 = &cards[i2];
                        let c3 = &cards[i3];
                        let c4 = &cards[i4];
                        let c5 = &cards[i5];

                        let rank = eval_5cards([c1, c2, c3, c4, c5]);
                        let rank_class = hand_rank(rank); //TODO: is this a move?

                        //rank_count.insert(rank, true);

                        //if !rank_class_count.contains_key(&rank_class) {
                        //    rank_class_count.insert(rank_class, 1);
                        //} else {
                        //    let cur_count = rank_class_count.get(rank_class);
                        //    rank_class_count.insert(rank_class, cur_count+1);
                        //}
                    }
                }
            }
        }
    }

    //assert_eq!(rank_class_count.get(HandRankClass::HighCard).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::OnePair).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::TwoPair).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::ThreeOfAKind).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::Straight).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::Flush).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::FullHouse).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::FourOfAKind).unwrap(), 0);
    //assert_eq!(rank_class_count.get(HandRankClass::StraightFlush).unwrap(), 0);

    // there should be 7462 unique ranks, according to the hand_rank function
    //TODO: rank_count len == 7462
}
