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
- [ ] Add arrow-key navigation (Up/Down/Left/Right) via crossterm
- [ ] Accept any digit 1–9 at cursor, not just a single test value

### Phase 2 — Raw input (crossterm, via ratatui dependency)

- [ ] Replace `read_line` with crossterm event polling
- [ ] Arrow keys and digit keys work without pressing Enter
- [ ] Enable raw mode on startup, restore terminal on exit

### Phase 3 — Cursor visual

- [x] Show which cell is selected in the render output (reverse-video highlight on the cursor cell)
- [x] Full redraw each iteration (board + cursor overlay)
- [ ] **Blinking cursor** — make the highlight pulse on its own, on a clock
  - Drive the blink from wall-clock time, not loop iterations, so the rate
    stays steady regardless of how fast keys are pressed
  - Keep two independent pieces of state: cursor *position* (moves on arrow
    keys) and blink *phase* (toggles on a ~500ms timer)
  - Each frame renders highlight-on or highlight-off depending on the phase;
    reuse the existing highlighted vs. plain render paths
  - Poll on a shorter interval than the blink period for smoother timing
- [ ] **No scrollback / locked view** — switch to the terminal's alternate
  screen so redraws don't stack up in scrollback
  - Enter alternate screen + raw mode on startup; hide the hardware cursor
  - Restore everything on exit (show cursor, disable raw mode, leave alt
    screen) on *every* exit path, not just the happy one

### Phase 4 — Ratatui widgets (optional polish)

- [ ] Swap string rendering for Ratatui layout/widgets
- [ ] Styling, status line, cleaner cursor highlight

### Phase 4 — Seed cells + locking (next pickup)

Goal: mount given numbers on the board at startup and prevent the user from
editing them. Design decision already made — use a **parallel mask** on
`Board`, not an enum cell type.

- [ ] Add an `editable: Vec<Vec<bool>>` field to `Board`, same shape as `cells`
  - Polarity: `editable[r][c] == true` → user may change it; seed cells `false`
  - `Board::new` initializes all cells `true` (empty board, nothing locked)
  - Fix the test struct literals (`sample_board`, `test_board`) — adding a
    field breaks every `Board { .. }` literal until they include `editable`
- [ ] Enforce locking **inside `Board`**, not in the input handler
  - `set_cell` / `clear_cell` consult `editable` and refuse on locked cells
    (return `bool` for now: applied vs. rejected — upgrade to `Result` later)
  - Rationale: "a fixed clue can't change" is a board invariant; guarding the
    mutators makes the illegal state unreachable for any caller
  - `clear_cell` respects the lock too — Backspace must not erase a given
- [ ] Add `set_given(row, col, value)` — writes the value *and* locks the cell
  in one call, so the two parallel arrays can never drift out of sync
- [ ] Seed generation — fill some `Given` cells at startup via `set_given`
  (see [Sudoku solution generation (proposal)](#sudoku-solution-generation-proposal))
- [ ] Rendering unchanged for now (locked cells look the same; dim/bold the
  givens is later polish)

### Phase 5 — Game rules (later)

- [ ] Move validation (row, column, box constraints)
- [ ] Self-correcting / undo on invalid input
- [ ] Variable board sizes (4×4, 16×16, 25×25, etc.)
- [ ] Puzzle generation and difficulty levels
  (see [Sudoku solution generation (proposal)](#sudoku-solution-generation-proposal))

## Sudoku solution generation (proposal)

Design note for how to produce valid complete grids and derive playable puzzles.
Not implemented yet — this section records the intended approach before writing
code.

### Two separate problems

| Problem | Output | Used for |
|---|---|---|
| **Solution generation** | A fully filled, rule-valid grid | Hidden answer; source of truth for givens |
| **Puzzle seeding** | A partial grid with locked clue cells | What the player sees at startup |

Validity (each row, column, and box contains 1–N exactly once) is necessary
but not sufficient for a good game. Random digits in random cells almost never
works. The reliable pipeline is:

```text
generate full solution  →  remove clues + verify uniqueness  →  set_given() for survivors
```

Phase 4 (`set_given`, `editable` mask) handles the last step. Phase 5 adds move
validation. This proposal covers the first two steps.

### Recommended approach: diagonal boxes + backtracking

**Primary method for `rusty-sudoku`.** Fast enough for startup generation on a
9×9 board, easy to test, and generalizes to other square box sizes.

**Step 1 — seed the diagonal boxes.** On a 9×9 grid, the three boxes at
`(0,0)`, `(1,1)`, and `(2,2)` share no rows or columns with each other. Fill
each with a random permutation of 1–9. This gives a large valid partial state
with zero backtracking.

**Step 2 — complete the grid.** Walk remaining empty cells in a fixed order
(e.g. row-major). For each cell, try digits 1–9 in shuffled order. Skip any
digit that conflicts with the current row, column, or box. Recurse; backtrack on
dead ends. The first complete assignment is a valid solution.

**Step 3 — (optional) diversify.** To avoid generating the same shape every
run, apply random **validity-preserving transforms** to the finished grid:

- Relabel digits (e.g. swap all 1s and 7s)
- Swap rows within a band, or swap entire bands
- Swap columns within a stack, or swap entire stacks
- Rotate or mirror the board

Any combination of these yields another legal solution without re-running search.

**Why this over naive backtracking from an empty board?** Diagonal seeding
eliminates most early conflicts, so completion typically finishes in
milliseconds. Pure random fill-from-scratch is correct but noticeably slower and
less predictable.

### Alternative: transform a canonical solution

Keep one hardcoded solved grid (or load it once). Apply the transforms above to
produce variety. Cheapest runtime cost — no search at all — but every puzzle
shares the same underlying pattern unless transforms are applied aggressively.
Good as a fallback or for tests; less ideal as the only production generator.

### Alternative: naive backtracking from empty

Fill cells in order, try shuffled digits, backtrack on conflict. Correct and
simple to implement, but the search space is larger than diagonal seeding.
Useful as a reference implementation and for verifying other generators.

### Puzzle seeding (givens from a solution)

A full solution is not a puzzle. To produce startup givens:

1. Start from the complete solution (hidden from the player).
2. Remove values from cells one at a time (or in symmetric pairs for aesthetics).
3. After each removal, ask: **does this partial grid have exactly one
   solution?** If yes, keep the removal; if ambiguous or unsolvable, put the
   clue back.
4. Stop at a target clue count or difficulty threshold.
5. Call `set_given(row, col, value)` for every surviving clue.

**Uniqueness check** requires a solver that can count solutions (stop at 2).
Reuse the same backtracking engine from solution generation, but run it on the
partial grid with givens treated as fixed.

**Difficulty** (later) is not the same as clue count. Two puzzles with 28 givens
can differ sharply. Grading by required techniques (singles, pairs, X-wing, …)
is out of scope for the first generator; start with a fixed clue count (e.g.
30–40 for 9×9) and add grading once validation and seeding work.

### Validation dependency

Solution generation and puzzle seeding both need **conflict detection**: can
digit `d` go at `(row, col)` given current cell values? This is the same check
Phase 5 will use for player moves. Build it once on `Board`:

```text
is_valid_placement(row, col, digit) -> bool
```

Generation, seeding, and live input validation should all call the same function
so the rules never drift apart.

### Proposed module layout

Keep generation out of the TUI loop and out of rendering:

```text
src/
  board.rs       — grid, rendering, is_valid_placement, set_given
  generator.rs   — generate_solution(), seed_puzzle(solution, clue_count)
  main.rs        — App loop; calls generator at startup (or loads a preset)
```

`Board::seed()` (if kept) should delegate to `generator` rather than embed
search logic on the struct.

### Phased implementation order

| Step | Delivers | Depends on |
|---|---|---|
| 1. `is_valid_placement` | Shared rule check | Phase 5 validation |
| 2. `generate_solution` | Full legal grid | Step 1 |
| 3. `count_solutions` / uniqueness | Puzzle safety check | Step 2 |
| 4. `seed_puzzle` | Partial grid + clue count | Steps 2–3, Phase 4 `set_given` |
| 5. Transforms + difficulty | Variety and grading | Steps 2–4 |

**MVP shortcut:** ship a few hand-authored puzzles (hardcoded givens) while
Steps 1–4 are in progress. Swap in runtime generation once uniqueness checks
pass tests.

### Testing strategy

- **`is_valid_placement`:** known valid/invalid placements on a partially filled
  board.
- **`generate_solution`:** output passes full-grid validation (every row, col,
  box is a permutation of 1–9); run N times and assert not all identical.
- **`seed_puzzle`:** seeded partial grid has exactly one solution; all givens
  match the source solution.
- **Regression:** solver completes a known published puzzle to the expected
  answer.

### Non-goals (for now)

- Minimum-clue puzzles (17-givens exist but are often unpleasant to play)
- Technique-based difficulty rating
- 16×16 / 25×25 generation (same algorithms apply once box size generalizes)
- Persistent puzzle databases or daily-puzzle APIs

## Target input spec

| Action | Keys |
|---|---|
| Move | Up/Down/Left/Right arrow keys |
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
