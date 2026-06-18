# rusty-sudoku

A terminal-based Sudoku game written in Rust. The goal is a locked terminal
view where you navigate an N×N grid, fill and clear cells, and eventually
play with full rule validation — all without leaving the terminal.

## Getting started

You'll need a recent Rust toolchain (this project uses edition 2024). Install
it via [rustup](https://rustup.rs/) if you haven't already.

```sh
# build
cargo build

# run
cargo run

# test
cargo test
```

## What's built so far

### Board (`src/board.rs`)

- `Board` struct with configurable `size`, `box_size`, and `cells`
- Empty cells represented as `-1` (rendered as blank space)
- `Board::new(size, box_size)` — creates an empty board
- `set_cell` / `clear_cell` — mutate individual cells
- Box-drawing rendering with thick 3×3 separators and thin inner lines
- `BorderStyle` — shared border template for top, bottom, thick middle, and thin middle lines
- `Display` impl — full-board text output via `to_string()`, `format!("{board}")`, or `print!("{board}")`
- Unit tests (6 passing) for borders, row formatting, and full-board rendering

### Main loop (`src/main.rs`)

- In-place redraw prototype: clear screen, render board, wait for input
- Starts with an empty 9×9 board
- Temporary line-based input (`read_line`, requires Enter):
  - `q` — quit
  - `5` — set cell (0, 0) to 5
  - `c` — clear cell (0, 0)
- Hardcoded cell coordinates; no cursor yet

### Dependencies

- `ratatui` — added for the upcoming TUI/event-loop phase (not wired up yet)

## Project structure

```text
src/
  main.rs    — entry point, game loop (in progress)
  board.rs   — Board data, rendering, tests
```

## Architecture

Two layers of state (cursor/App layer coming next):

| Layer | Responsibility |
|---|---|
| `Board` | Puzzle data — cells, size, rendering, mutation |
| `App` (planned) | Interaction — cursor position, input handling, running flag |

`Board` owns the grid and how it looks. `App` will own how the user interacts
with it. Rendering stays on `Board`; input and cursor logic stay out of it.

## Phased build order

### Phase 1 — App + cursor (stdlib, no new libraries)

- [ ] Add `App` struct wrapping `Board` with `cursor_row`, `cursor_col`, `running`
- [ ] Replace hardcoded `(0, 0)` with cursor position for `set_cell` / `clear_cell`
- [ ] Add letter-based navigation (`w`/`a`/`s`/`d` or `h`/`j`/`k`/`l`) via `read_line`
- [ ] Accept any digit 1–9 at cursor, not just a single test value

### Phase 2 — Raw input (crossterm, via ratatui dependency)

- [ ] Replace `read_line` with crossterm event polling
- [ ] Arrow keys and digit keys work without pressing Enter
- [ ] Enable raw mode on startup, restore terminal on exit

### Phase 3 — Cursor visual

- [ ] Show which cell is selected in the render output
- [ ] Full redraw each iteration (board + cursor overlay)

### Phase 4 — Ratatui widgets (optional polish)

- [ ] Swap string rendering for Ratatui layout/widgets
- [ ] Styling, status line, cleaner cursor highlight

### Phase 5 — Game rules (later)

- [ ] Distinguish fixed clue cells from editable cells
- [ ] Move validation (row, column, box constraints)
- [ ] Self-correcting / undo on invalid input
- [ ] Variable board sizes (4×4, 16×16, 25×25, etc.)
- [ ] Puzzle generation and difficulty levels

## Target input spec

| Action | Keys |
|---|---|
| Move | Up/Down/Left/Right or `h`/`j`/`k`/`l` |
| Fill | `1`–`9` at cursor |
| Clear | Backspace, Delete, or `0` |
| Quit | `q` or Esc |

## Done criteria (interactive board MVP)

- [x] Board renders with box-drawing characters
- [x] Board can be created empty and mutated in memory
- [x] In-place redraw loop (clear screen + re-render)
- [ ] Visible cursor navigation
- [ ] Set/clear at cursor position
- [ ] Raw key input (no Enter required)
- [ ] Clean terminal restore on exit

## Vision (not yet implemented)

- Play entirely in the terminal through a TUI
- Every move checked against Sudoku rules before it sticks
- Invalid moves reported and undone so the board stays legal
