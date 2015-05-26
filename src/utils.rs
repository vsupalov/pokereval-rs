use super::lookups;
use holdem::{CactusKevCard};
use cards::card::{Card, Value, Suit};

/// Converts a card to a cactusKevCard, which is a convenient binary representation:
///
///  +--------+--------+--------+--------+
///  |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
///  +--------+--------+--------+--------+
///
///  p = prime number of value (deuce=2,trey=3,four=5,five=7,...,ace=41)
///  r = value of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
///  cdhs = suit of card
///  b = bit turned on depending on value of card
pub fn card_to_deck_number(card: &Card) -> CactusKevCard {
    let value : u32  = match card.value {
        Value::Two => 0,
        Value::Three => 1,
        Value::Four => 2,
        Value::Five => 3,
        Value::Six => 4,
        Value::Seven => 5,
        Value::Eight => 6,
        Value::Nine => 7,
        Value::Ten => 8,
        Value::Jack => 9,
        Value::Queen => 10,
        Value::King => 11,
        Value::Ace => 12,
    };
    let prime : u32 = lookups::PRIMES[value as usize] as u32;
    let suit : u32  = match card.suit {
        Suit::Hearts => 0x1000,
        Suit::Spades => 0x2000,
        Suit::Diamonds => 0x4000,
        Suit::Clubs => 0x8000,
    };
    let bits : u32 = 1 << (16+value);

    prime | value << 8 | suit | bits
}
