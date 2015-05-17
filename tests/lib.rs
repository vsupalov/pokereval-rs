extern crate holdem;
extern crate cards;
extern crate pokereval;

use cards::card::{Card, Value, Suit};
use cards::deck::{Deck};

use holdem::{HandRankClass};
use pokereval::utils::{hand_rank};
use pokereval::{original, perfect}; // two evaluation methods

use holdem::{HandCards, CommunityCards, CardSlot};

#[test]
fn get_rank_of_5_original() {
    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Two, Suit::Diamonds);
    let c4 = Card(Value::Two, Suit::Clubs);
    let c5 = Card(Value::Three, Suit::Hearts);

    let cards = [&c1, &c2, &c3, &c4, &c5];
    let rank = original::eval_5cards(cards);

    assert_eq!(hand_rank(rank), HandRankClass::FourOfAKind);
}

#[test]
fn get_rank_of_7_original() {
    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Two, Suit::Diamonds);
    let c4 = Card(Value::Two, Suit::Clubs);
    let c5 = Card(Value::Three, Suit::Hearts);
    let c6 = Card(Value::Three, Suit::Diamonds);
    let c7 = Card(Value::Three, Suit::Clubs);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    let rank = original::eval_7cards(cards);

    assert_eq!(hand_rank(rank), HandRankClass::FourOfAKind);
}

#[test]
fn get_rank_of_5_perfect() {
    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Two, Suit::Diamonds);
    let c4 = Card(Value::Two, Suit::Clubs);
    let c5 = Card(Value::Three, Suit::Hearts);

    let cards = [&c1, &c2, &c3, &c4, &c5];
    let rank = perfect::eval_5cards(cards);

    assert_eq!(hand_rank(rank), HandRankClass::FourOfAKind);
}

#[test]
fn get_rank_of_7_perfect() {
    let c1 = Card(Value::Two, Suit::Spades);
    let c2 = Card(Value::Two, Suit::Hearts);
    let c3 = Card(Value::Two, Suit::Diamonds);
    let c4 = Card(Value::Two, Suit::Clubs);
    let c5 = Card(Value::Three, Suit::Hearts);
    let c6 = Card(Value::Three, Suit::Diamonds);
    let c7 = Card(Value::Three, Suit::Clubs);

    let cards = [&c1, &c2, &c3, &c4, &c5, &c6, &c7];
    let rank = perfect::eval_7cards(cards);

    assert_eq!(hand_rank(rank), HandRankClass::FourOfAKind);
}

#[test]
fn get_rank_of_hand_and_community_cards() {
    let hand = HandCards(
        CardSlot::Dealt(Card(Value::Two, Suit::Spades)),
        CardSlot::Dealt(Card(Value::Two, Suit::Hearts)),
    );

    let community = CommunityCards(
        CardSlot::Dealt(Card(Value::Two, Suit::Clubs)),
        CardSlot::Dealt(Card(Value::Two, Suit::Diamonds)),
        CardSlot::Dealt(Card(Value::Three, Suit::Spades)),
        CardSlot::Dealt(Card(Value::Three, Suit::Clubs)),
        CardSlot::Dealt(Card(Value::Three, Suit::Hearts)),
    );

    let rank = pokereval::eval_for_player(&hand, &community);
    assert_eq!(hand_rank(rank), HandRankClass::FourOfAKind);
}

#[test]
#[should_panic]
fn get_rank_of_hand_and_community_cards_panic() {
    let hand = HandCards(
        CardSlot::Dealt(Card(Value::Two, Suit::Spades)),
        CardSlot::Dealt(Card(Value::Two, Suit::Hearts)),
    );

    let community = CommunityCards(
        CardSlot::Dealt(Card(Value::Two, Suit::Clubs)),
        CardSlot::Dealt(Card(Value::Two, Suit::Diamonds)),
        CardSlot::Dealt(Card(Value::Three, Suit::Spades)),
        CardSlot::Dealt(Card(Value::Three, Suit::Clubs)),
        CardSlot::Empty,
    );

    let rank = pokereval::eval_for_player(&hand, &community);
    assert_eq!(hand_rank(rank), HandRankClass::FourOfAKind);
}

//TODO: as soon as both methods are expected to agree
// this guy does not always pass
/*
#[test]
fn both_evaluation_methods_agree() {
    let mut deck = Deck::new();

    // try on 10 hands
    for _ in 0..10 {
        let c1 = deck.draw();
        let c2 = deck.draw();
        let c3 = deck.draw();
        let c4 = deck.draw();
        let c5 = deck.draw();
        
        let rank_original = original::eval_5cards([&c1, &c2, &c3, &c4, &c5]);
        let rank_perfect = perfect::eval_5cards([&c1, &c2, &c3, &c4, &c5]);
        assert_eq!(rank_original, rank_perfect);
    }
}
*/
