use super::lookups;
use cards::card::{Card};
use super::{HandRank};
use super::utils::{card_to_deck_number};

#[allow(dead_code)]
pub fn find_fast(something: usize) -> usize {
    let mut u : usize = something;

    u += 0xe91aaa35;
    u ^= u >> 16;
    u += u << 8;
    u ^= u >> 4;
    let b = (u >> 8) & 0x1ff;
    let a = (u + (u << 2)) >> 19;

    a ^ (lookups::HASH_ADJUST[b] as usize)
}

#[allow(dead_code)]
pub fn eval_5cards(cards: [&Card; 5]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]) as i32;
    let c2 = card_to_deck_number(cards[1]) as i32;
    let c3 = card_to_deck_number(cards[2]) as i32;
    let c4 = card_to_deck_number(cards[3]) as i32;
    let c5 = card_to_deck_number(cards[4]) as i32;

    let q : usize = ((c1 | c2 | c3 | c4 | c5) as usize) >> 16;

    if (c1 & c2 & c3 & c4 & c5 & 0xf000) != 0 {
        return lookups::FLUSHES[q] as HandRank;
    }
    let s = lookups::UNIQUE_5[q] as HandRank;
    if s != 0 {
        return s;
    }

    //TODO: FIXME
    // version: perfect hash. Not working currently
    let lookup = find_fast(
        ((c1 & 0xff) * (c2 & 0xff) * (c3 & 0xff) * (c4 & 0xff) * (c5 & 0xff)) as usize
        );
    lookups::HASH_VALUES[lookup] as HandRank
}

#[allow(dead_code)]
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
