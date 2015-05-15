use super::lookups;
use super::{HandRank};
use super::types::{InternalCardRepresentation};
use holdem::{HandRankClass};
use cards::card::{Card, Value, Suit};

pub fn hand_rank(val: HandRank) -> HandRankClass {
    if val > 6185 {
        return HandRankClass::HighCard        // 1277 high card
    } else if val > 3325 {
        return HandRankClass::OnePair         // 2860 one pair
    } else if val > 2467 {
        return HandRankClass::TwoPair         //  858 two pair
    } else if val > 1609 {
        return HandRankClass::ThreeOfAKind    //  858 three-kind
    } else if val > 1599 {
        return HandRankClass::Straight        //   10 straights
    } else if val > 322 {
        return HandRankClass::Flush           // 1277 flushes
    } else if val > 166 {
        return HandRankClass::FullHouse       //  156 full house
    } else if val > 10 {
        return HandRankClass::FourOfAKind     //  156 four-kind
    } else {
        HandRankClass::StraightFlush          //   10 straight-flushes
    }
}

//   +--------+--------+--------+--------+
//   |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
//   +--------+--------+--------+--------+
//
//   p = prime number of value (deuce=2,trey=3,four=5,five=7,...,ace=41)
//   r = value of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
//   cdhs = suit of card
//   b = bit turned on depending on value of card
pub fn card_to_deck_number(card: &Card) -> InternalCardRepresentation {
    let &Card(ref card_value, ref card_suit) = card;

    let value : u32  = match card_value {
        &Value::Two => 0,
        &Value::Three => 1,
        &Value::Four => 2,
        &Value::Five => 3,
        &Value::Six => 4,
        &Value::Seven => 5,
        &Value::Eight => 6,
        &Value::Nine => 7,
        &Value::Ten => 8,
        &Value::Jack => 9,
        &Value::Queen => 10,
        &Value::King => 11,
        &Value::Ace => 12,
    };
    let prime : u32 = lookups::PRIMES[value as usize] as u32;
    let suit : u32  = match card_suit {
        &Suit::Hearts => 0x1000,
        &Suit::Spades => 0x2000,
        &Suit::Diamonds => 0x4000,
        &Suit::Clubs => 0x8000,
    };
    let bits : u32 = 1 << (16+value);

    prime | value << 8 | suit | bits
}
