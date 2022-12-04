# Rust Tic-Tac-Toe

A simple Tic-Tac-Toe game written in Rust.

To play, clone this repository and run `cargo run` in the root directory.

```
cargo run
```

## AI

The AI uses the minimax algorithm to decide on its best move. The `max_depth` field in `AI` controls how far the AI looks ahead. A higher `max_depth` results in a smarter (but slower) AI.

```
let mut ai = AI::new(4);
```

## Screenshots

![screenshot1](screenshots/screenshot1.png)

![screenshot2](screenshots/screenshot2.png)
