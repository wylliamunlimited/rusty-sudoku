# rusty-sudoku

A terminal-based Sudoku game written in Rust. You play entirely in your
terminal through a text user interface (TUI), and the board keeps itself
honest: every move is checked against the rules of Sudoku before it sticks.

## How it works

- **Play in the terminal.** The board is rendered as a TUI, and you move
  around and place numbers without leaving the terminal.
- **Validated moves.** Each time you place a number, the board is verified
  against the standard Sudoku constraints — no repeats within a row, a
  column, or a 3x3 box.
- **Self-correcting.** If a move would make the board invalid, the game
  reports the error and undoes the move, so the board is always in a legal
  state.

## Getting started

You'll need a recent Rust toolchain (this project uses edition 2024). Install
it via [rustup](https://rustup.rs/) if you haven't already.

```sh
# build
cargo build

# run
cargo run
```

## Project status

Early days — the project is just getting off the ground. It started as a way
to dig into Rust, so expect things to evolve as the game takes shape.

## Roadmap

- [ ] Core board representation and validation
- [ ] TUI rendering and input handling
- [ ] Move/undo handling on invalid input
- [ ] Puzzle generation and difficulty levels
