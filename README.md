#pokereval-rs

A Rust library, implementating a poker hand evaluator. Based on [the original work](http://suffecool.net/poker/evaluator.html) by Kevin Suffecool (aka Cactus Kev).

The method based on optimizations contributed by Paul D. Senzee's with his hashing based [Optimized Hand Evaluator](http://www.paulsenzee.com/2006/06/some-perfect-hash.html) is currently not functional (tries to access out-of-bounds array values) and has been disabled while the code still remains in *src/perfect.rs*.

The crate is called `pokereval` and you can depend on it via cargo:

```ini
[dependencies.pokereval]
git = "https://github.com/th4t/pokereval-rs.git"
```

## Related Crates
* [cards-rs](https://github.com/th4t/cards-rs)
* [holdem-rs](https://github.com/th4t/holdem-rs)
* **pokereval-rs**
