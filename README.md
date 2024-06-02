![50bfd8bc-027a-452c-b974-50d24aa2527a](https://github.com/shadmanrakib/patzer-gambit/assets/64807913/6b255289-253b-4fd6-8f76-99684e8202c5)


# Patzer Gambit

A partial UCI chess engine that's a whole lot worse than Drunk Magnus but magnitudes better than me.

## Building & Running The Engine

You will need to install rust, rustc, and cargo. You can install it here: https://www.rust-lang.org/.

To build the engine, clone the repo:
```
git clone https://github.com/shadmanrakib/patzer-gambit.git
```

Change your directory to /engine:
```
cd engine
```

Build and run the engine via cargo:
```
cargo run --release
```

To setup, the lichess bot after building, cd out of engine and copy the executable (engine/target/release/engine) to lichess-bot/engines/engine
```
cp engine/target/release/engine lichess-bot/engines/engine
```

Then, in the config.yml add your lichess OAuth token. Follow, the README in the lichess-bot directory for more instructions.


## Features

**Implemented:**
* Search
  * Negamax 
  * Iterative Deepening
  * Quiescence Search
  * Magic Bitboard Move Generation
  * Transposition Table Indexed By Zobrist Hashes
* Pruning
  * Alpha Beta Pruning
  * Null Move Pruning
  * Principal Variation Search
  * Late Move Reduction
  * Aspiration Windows
* Move Ordering
  * Transposition Table Refutation/Best Move
  * Most Valuable Victim Least Valuable Attacker
  * Killer Move Heuristic
  * Static Exchange Evaluation
* Evaluation
  * Tapered Piece Square Tables
  * 50 Half-Moves, 3 Fold Repeition, Insufficient Material Draws (evaluated in negamax, not scoring function)

**Planned:**
* Factor Pawn Structure, Mobility, and King Safety in Evaluation

## Resources

These resources have been invaluable references & inspiration:

* https://chessprogramming.org/
* [Bitboard CHESS ENGINE in C by Code Monkey King](https://youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs&si=d3DL6pOnZ5XY4FxE)
* [Rustic Chess Engine](https://github.com/mvanthoor/rustic)
* [Crabby Chess Engine](https://github.com/Johnson-A/Crabby)
* https://analog-hors.github.io/site/magic-bitboards/
