extern crate test;

extern crate cards;
extern crate pokereval;

use test::Bencher;
use cards::deck::{Deck};
use pokereval::{original}; // two evaluation methods
//perfect

#[bench]
fn bench_original_evaluation(b: &mut Bencher) {
    let mut deck = Deck::new();

    b.iter(|| {
    // try on 10 hands
        for _ in 0..10 {
            let c1 = deck.draw();
            let c2 = deck.draw();
            let c3 = deck.draw();
            let c4 = deck.draw();
            let c5 = deck.draw();
            
            let _rank_original = original::eval_5cards([&c1, &c2, &c3, &c4, &c5]);
       }
       deck.reset();
    });
}
