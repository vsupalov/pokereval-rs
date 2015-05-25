use super::{lookups};

use cards::card::{Card};

use holdem::{HandRank};
use super::utils::{card_to_deck_number};
use holdem::{CactusKevCard};

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

pub fn eval_5cards_raw_unwrapped_behind_function(c1: &CactusKevCard, c2: &CactusKevCard, c3: &CactusKevCard, c4: &CactusKevCard, c5: &CactusKevCard) -> HandRank {
    let kev_rank = eval_5cards_raw_unwrapped(c1, c2, c3, c4, c5);
    7461 - (kev_rank-1) as HandRank //let's change this to be (0 to 7461 inclusive), with 7461 being the best
}

//TODO: test method
pub fn eval_5cards_raw_unwrapped(c1: &CactusKevCard, c2: &CactusKevCard, c3: &CactusKevCard, c4: &CactusKevCard, c5: &CactusKevCard) -> HandRank {
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

//this delivers a value (1 to 7462 inclusive), where 1 is the best
pub fn eval_5cards_kev(cards: [&CactusKevCard; 5]) -> HandRank {
    let c1 = cards[0];
    let c2 = cards[1];
    let c3 = cards[2];
    let c4 = cards[3];
    let c5 = cards[4];

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

pub fn eval_5cards_raw(cards: [&CactusKevCard; 5]) -> HandRank {
    let kev_rank = eval_5cards_kev(cards);
    7461 - (kev_rank-1) as HandRank //let's change this to be (0 to 7461 inclusive), with 7461 being the best
}

pub fn eval_6cards_raw(cards: [&CactusKevCard; 6]) -> HandRank {
    let mut tmp;
    let mut best = 0;
    for ids in lookups::PERM_6.iter() {
        let subhand : [&CactusKevCard; 5] = [
                cards[ids[0] as usize],
                cards[ids[1] as usize],
                cards[ids[2] as usize],
                cards[ids[3] as usize],
                cards[ids[4] as usize]
         ];

        tmp = eval_5cards_raw(subhand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

pub fn eval_7cards_raw(cards: [&CactusKevCard; 7]) -> HandRank {
    let mut tmp;
    let mut best = 0;
    for ids in lookups::PERM_7.iter() {
        let subhand : [&CactusKevCard; 5] = [
                cards[ids[0] as usize],
                cards[ids[1] as usize],
                cards[ids[2] as usize],
                cards[ids[3] as usize],
                cards[ids[4] as usize]
         ];

        tmp = eval_5cards_raw(subhand);
        if tmp > best {
            best = tmp;
        }
    }

    best
}

pub fn eval_5cards(cards: [&Card; 5]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);

    eval_5cards_raw([&c1, &c2, &c3, &c4, &c5])
}

pub fn eval_6cards(cards: [&Card; 6]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);
    let c6 = card_to_deck_number(cards[5]);

    eval_6cards_raw([&c1, &c2, &c3, &c4, &c5, &c6])
}

pub fn eval_7cards(cards: [&Card; 7]) -> HandRank {
    let c1 = card_to_deck_number(cards[0]);
    let c2 = card_to_deck_number(cards[1]);
    let c3 = card_to_deck_number(cards[2]);
    let c4 = card_to_deck_number(cards[3]);
    let c5 = card_to_deck_number(cards[4]);
    let c6 = card_to_deck_number(cards[5]);
    let c7 = card_to_deck_number(cards[6]);

    eval_7cards_raw([&c1, &c2, &c3, &c4, &c5, &c6, &c7])
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};

    use cards::deck::{Deck};
    use cards::card::{Card};
    use holdem::{HandRank, HandRankClass, hand_rank_to_class};
    use holdem::{CactusKevCard};

    use super::{eval_5cards_raw,eval_5cards};
    use super::super::utils::{card_to_deck_number};

    // TODO: this is not really specific to this evaluation method. It could as well live in the library tests folder
    // the reason it is here, is due to the internal representation hackage which got ditched
    #[test]
    fn evaluate_all_possible_5_card_combinations_quick() {
        let mut deck = Deck::new_unshuffled();
        let mut cards : [CactusKevCard; 52] = [0; 52];
    
        // this could be made faster, by creating a function that works on raw-card-representations and translating
        // the deck cards into it
        for i in 0..52 {
            let card = deck.draw().ok().unwrap();
            cards[i] = card_to_deck_number(&card);
        }
    
        let mut rank_class_count : HashMap<HandRankClass, usize> = HashMap::new();
        let mut rank_count : HashMap<HandRank, bool> = HashMap::new();

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
    
                            let rank = eval_5cards_raw([c1, c2, c3, c4, c5]);

                            // mark the rank in the map
                            rank_count.entry(rank).or_insert(true);
                        }
                    }
                }
            }
        }

        // count distinct ranks for each rank class
        for key in rank_count.keys() {
            let rank_class = hand_rank_to_class(key);

            let count = rank_class_count.entry(rank_class).or_insert(0);
            *count += 1;
        }

        // this is a bit redundant
        // there should be 7462 unique ranks, in accordance with the hand_rank_to_class function
        assert_eq!(rank_count.len(), 7462);
    
        assert_eq!(*rank_class_count.get(&HandRankClass::HighCard).unwrap(), 1277);
        assert_eq!(*rank_class_count.get(&HandRankClass::OnePair).unwrap(), 2860);
        assert_eq!(*rank_class_count.get(&HandRankClass::TwoPair).unwrap(), 858);
        assert_eq!(*rank_class_count.get(&HandRankClass::ThreeOfAKind).unwrap(), 858);
        assert_eq!(*rank_class_count.get(&HandRankClass::Straight).unwrap(), 10);
        assert_eq!(*rank_class_count.get(&HandRankClass::Flush).unwrap(), 1277);
        assert_eq!(*rank_class_count.get(&HandRankClass::FullHouse).unwrap(), 156);
        assert_eq!(*rank_class_count.get(&HandRankClass::FourOfAKind).unwrap(), 156);
        assert_eq!(*rank_class_count.get(&HandRankClass::StraightFlush).unwrap(), 10);
    }

    #[test]
    fn evaluate_all_possible_5_card_combinations() {
        let mut deck = Deck::new_unshuffled();
        let mut cards : Vec<Card> = Vec::new();
    
        for _ in 0..52 {
            let card = deck.draw().ok().unwrap();
            cards.push(card);
        }
    
        let mut rank_class_count : HashMap<HandRankClass, usize> = HashMap::new();
        let mut rank_count : HashMap<HandRank, bool> = HashMap::new();

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

                            // mark the rank in the map
                            rank_count.entry(rank).or_insert(true);
                        }
                    }
                }
            }
        }

        // count distinct ranks for each rank class
        for key in rank_count.keys() {
            let rank_class = hand_rank_to_class(key);

            let count = rank_class_count.entry(rank_class).or_insert(0);
            *count += 1;
        }
    
        assert_eq!(*rank_class_count.get(&HandRankClass::HighCard).unwrap(), 1277);
        assert_eq!(*rank_class_count.get(&HandRankClass::OnePair).unwrap(), 2860);
        assert_eq!(*rank_class_count.get(&HandRankClass::TwoPair).unwrap(), 858);
        assert_eq!(*rank_class_count.get(&HandRankClass::ThreeOfAKind).unwrap(), 858);
        assert_eq!(*rank_class_count.get(&HandRankClass::Straight).unwrap(), 10);
        assert_eq!(*rank_class_count.get(&HandRankClass::Flush).unwrap(), 1277);
        assert_eq!(*rank_class_count.get(&HandRankClass::FullHouse).unwrap(), 156);
        assert_eq!(*rank_class_count.get(&HandRankClass::FourOfAKind).unwrap(), 156);
        assert_eq!(*rank_class_count.get(&HandRankClass::StraightFlush).unwrap(), 10);
    
        // this is a bit redundant
        // there should be 7462 unique ranks, in accordance with the hand_rank_to_class function
        assert_eq!(rank_count.len(), 7462);
    }
}

