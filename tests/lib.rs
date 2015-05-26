extern crate holdem;
extern crate cards;
extern crate pokereval;

use cards::card::{Card, Value, Suit};

use holdem::{HandRankClass};
use holdem::{hand_rank_to_class};

#[test]
fn get_rank_of_5() {
    let c1 = Card::new(Value::Two, Suit::Spades);
    let c2 = Card::new(Value::Two, Suit::Hearts);
    let c3 = Card::new(Value::Two, Suit::Diamonds);
    let c4 = Card::new(Value::Two, Suit::Clubs);
    let c5 = Card::new(Value::Three, Suit::Hearts);

    let cards = [&c1, &c2, &c3, &c4, &c5];
    let rank = pokereval::eval_5cards(&cards);

    assert_eq!(hand_rank_to_class(&rank), HandRankClass::FourOfAKind);
}

#[test]
fn get_rank_of_6() {
    let c1 = Card::new(Value::Two, Suit::Spades);
    let c2 = Card::new(Value::Two, Suit::Hearts);
    let c3 = Card::new(Value::Two, Suit::Diamonds);
    let c4 = Card::new(Value::Two, Suit::Clubs);
    let c5 = Card::new(Value::Three, Suit::Hearts);
    let c6 = Card::new(Value::Three, Suit::Diamonds);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6];
    let rank = pokereval::eval_6cards(&cards);

    assert_eq!(hand_rank_to_class(&rank), HandRankClass::FourOfAKind);
}

#[test]
fn get_rank_of_7() {
    let c1 = Card::new(Value::Two, Suit::Spades);
    let c2 = Card::new(Value::Two, Suit::Hearts);
    let c3 = Card::new(Value::Two, Suit::Diamonds);
    let c4 = Card::new(Value::Two, Suit::Clubs);
    let c5 = Card::new(Value::Three, Suit::Hearts);
    let c6 = Card::new(Value::Three, Suit::Diamonds);
    let c7 = Card::new(Value::Three, Suit::Clubs);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    let rank = pokereval::eval_7cards(&cards);

    assert_eq!(hand_rank_to_class(&rank), HandRankClass::FourOfAKind);
}
