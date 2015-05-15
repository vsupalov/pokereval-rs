use super::lookups;
use cards::card::{Card};
use super::{HandRank};
use super::utils::{card_to_deck_number};

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

