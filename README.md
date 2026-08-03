# rusty-sudoku

A terminal-based Sudoku game written in Rust. The goal is a locked terminal
view where you navigate an N×N grid, fill and clear cells, and eventually
play with full rule validation — all without leaving the terminal.

![Sample Board TUI](/snapshot.png)

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

The game is playable end to end: it generates a random 9×9 puzzle, opens on the
alternate screen in raw mode, and lets you navigate with arrow keys, type digits,
and clear cells — with every move checked against the rules. **42 tests passing.**

### Grid (`src/grid.rs`)

Shared rendering trait implemented by both `Board` and `Puzzle`:

- `size()`, `box_size()`, `cell_str(row, col)` — the three per-type hooks
- Default methods for all border variants and row formatting, so rendering logic
  lives in exactly one place
- `BorderStyle` — shared border template for top, bottom, thick middle, and thin
  middle lines

### Puzzle (`src/puzzle.rs`)

Generated game state, kept separate from `Board` (the play surface):

- `solution: Vec<Vec<i32>>` — the complete, rule-valid answer key (always fully
  filled, so plain `i32` rather than `Option<i32>`)
- `mask: Vec<Vec<bool>>` — which cells are given/revealed (`true`) vs. hidden
  (`false`) from the player
- `generate(size, box_size)` — random puzzle; the sole entry point for play
- `new(solution, mask)` — deterministic construction, used by tests
- `seed(size, box_size, rng)` — fills a complete solution via **iterative**
  backtracking over an explicit stack (no recursion), trying shuffled candidates
  per cell and unwinding dead ends
- `mask(size, rng)` — randomly reveals `size * size / 2` cells (shuffle all
  coordinates, take the first N)
- `validate(grid, row, col, box_size)` — row/column/box conflict check used
  during generation

### Board (`src/board.rs`)

The play surface. Holds the player's grid plus the `Puzzle` it came from, so it
can answer both "is this legal?" and "is this right?".

- `cells: Vec<Vec<Option<i32>>>` — `None` is empty (no sentinel value)
- `from_puzzle(puzzle)` — seeds revealed clues from the mask; hidden cells start
  empty. `new(size, box_size)` still makes a bare puzzle-less board
- Rule predicates:
  - `is_editable` — is this a player cell, or a given clue?
  - `is_valid_move` — no duplicate in row, column, or box (and the cell is empty)
  - `is_correct_move` — does this match the solution?
- `set_cell_gated` / `clear_cell_gated` — the gated mutators, returning
  `Result<(), OpError>`. All rule enforcement lives here, so illegal state is
  unreachable for any caller
- `OpError` — why a move was refused (`NotEditable`, `Conflicts`, `Incorrect`,
  `Occupied`), with a `Display` impl providing player-facing text
- `first_editable()` — first non-clue cell, for initial cursor placement
- `render(cursor, blink)` — full board with the cursor cell highlighted

### App (`src/app.rs`)

Interaction state, no rendering logic of its own:

- `cursor`, `highlight_on` + `last_blink_time` (blink phase), `last_error`
- `Action` / `Direction` enums — input is translated to intent before it reaches
  the game logic
- `shift_cursor` — clamps at the board edges **and skips over given clues**, so
  the cursor only ever rests where the player can type. Gives up rather than
  looping when only clues remain in that direction
- `step` — pure position-in/position-out helper the cursor logic is built on
- `set_current_cell` / `clear_current_cell` — store the rejection reason in
  `last_error`, cleared on the next successful action or cursor move
- `view()` — board render plus the current error message, if any

### Terminal (`src/tui.rs`)

- `TerminalGuard` — enters the alternate screen and raw mode on construction,
  restores both in `Drop` so every exit path cleans up
- `key_to_action` — arrow keys, `1`–`9`, Backspace, `q`
- `draw` — clear, render, flush

### Dependencies

- `crossterm` — raw mode, alternate screen, key events
- `rand` — puzzle generation
- `ratatui` — declared but **not yet used** (reserved for the widget phase)

## Project structure

```text
src/
  main.rs    — entry point, event loop
  app.rs     — App state, cursor movement, input intent
  board.rs   — play surface, rule enforcement, OpError
  puzzle.rs  — solution generation + clue mask
  grid.rs    — shared rendering trait
  tui.rs     — terminal setup/teardown, key translation
  tests/     — unit tests (test_app, test_board, test_puzzle)
```

## Architecture

| Layer | Responsibility |
|---|---|
| `Puzzle` | Generated truth — solution grid and which cells are revealed |
| `Board` | Play surface — cells, rendering, and **all rule enforcement** |
| `App` | Interaction — cursor, blink phase, last error, input intent |
| `TerminalGuard` | Terminal mode and raw key events |

`Board` owns the grid, how it looks, and what moves are legal. `App` owns how the
user interacts with it. Rules never live in the input handler: `App` asks `Board`
and reports the answer, so any future caller gets the same enforcement.

## Phased build order

### Phase 1 — App + cursor ✅

- [x] `App` struct wrapping `Board` with cursor state
- [x] Replace hardcoded `(0, 0)` with cursor position for set/clear
- [x] Arrow-key navigation (Up/Down/Left/Right) via crossterm
- [x] Accept any digit 1–9 at cursor

### Phase 2 — Raw input ✅

- [x] Replace `read_line` with crossterm event polling
- [x] Arrow keys and digit keys work without pressing Enter
- [x] Enable raw mode on startup, restore terminal on exit

### Phase 3 — Cursor visual ✅

- [x] Show which cell is selected in the render output
- [x] Full redraw each iteration (board + cursor overlay)
- [x] **Blinking cursor** — driven by wall-clock time (`Instant`, ~500ms), with
  cursor *position* and blink *phase* kept as independent state
- [x] **No scrollback / locked view** — alternate screen + raw mode, restored in
  `Drop` so every exit path cleans up

### Phase 4 — Puzzle generation ✅

- [x] `Puzzle` struct — solution grid + reveal mask
- [x] `seed()` — complete solution via iterative backtracking
- [x] `mask()` — random clue selection at a fixed clue count
- [x] `Board::from_puzzle` — mount generated clues at startup
- [ ] Uniqueness check — the mask currently removes cells at random, so a
  generated puzzle is **not guaranteed to have exactly one solution**
- [ ] Difficulty levels (clue count is hardcoded at `size * size / 2`)

### Phase 5 — Game rules ✅

- [x] Move validation (row, column, box constraints)
- [x] Clue locking — givens can't be overwritten or erased, enforced inside
  `Board` rather than in the input handler
- [x] Answer checking — `is_correct_move` against the solution
- [x] `Result<(), OpError>` on the gated mutators, so callers get a *reason*, not
  just a refusal, and can't silently ignore the outcome
- [x] Cursor skips locked cells during navigation
- [ ] Split `OpError::Occupied` out of `Conflicts` — `is_valid_move` currently
  collapses "cell is full" and "duplicate in row/col/box" into one `false`
- [ ] Variable board sizes (`Board` is size-generic; `main` hardcodes 9×9)

### Phase 6 — Game state (next pickup)

The rules layer is complete; nothing yet tracks the *game*. This is the gap:

- [ ] Solved detection — with the correctness gate in place the board can only
  hold correct values, so "solved" reduces to "no cell is `None`"
- [ ] Win state and end-of-game handling (currently `q` is the only exit)
- [ ] Optional: mistake counter / strike limit. Note this conflicts with the
  current design, where wrong digits are refused outright rather than recorded —
  allowing mistakes to land would mean dropping `is_correct_move` from the gate
- [ ] Timer / move count

### Phase 7 — Ratatui widgets (optional polish)

- [ ] Swap string rendering for Ratatui layout/widgets (dependency is declared
  but unused)
- [ ] Styling, status line, cleaner cursor highlight
- [ ] Visually distinguish given clues from player entries (dim/bold)

## Sudoku solution generation (proposal)

Design note for how to produce valid complete grids and derive playable puzzles.
**Partially implemented** — solution generation is done (`Puzzle::seed`, via
iterative backtracking rather than the recursive form sketched below). Clue
selection is done but *naive*: `Puzzle::mask` removes cells at random with no
uniqueness check, so a generated puzzle may admit more than one solution. The
uniqueness and difficulty sections below remain unbuilt.

### Two separate problems

| Problem | Output | Used for |
|---|---|---|
| **Solution generation** | A fully filled, rule-valid grid | Hidden answer; source of truth for givens |
| **Puzzle seeding** | A partial grid with locked clue cells | What the player sees at startup |

Validity (each row, column, and box contains 1–N exactly once) is necessary
but not sufficient for a good game. Random digits in random cells almost never
works. The reliable pipeline is:

```text
generate full solution  →  remove clues + verify uniqueness  →  mask survivors
```

The first and last steps are built (`Puzzle::seed`, `Puzzle::mask`, and
`Board::from_puzzle` mounting the survivors). The middle step — verifying that
the remaining clues admit exactly one solution — is the missing piece.

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
5. Mark every surviving clue in the mask.

**Uniqueness check** requires a solver that can count solutions (stop at 2).
Reuse the same backtracking engine from solution generation, but run it on the
partial grid with givens treated as fixed.

**Difficulty** (later) is not the same as clue count. Two puzzles with 28 givens
can differ sharply. Grading by required techniques (singles, pairs, X-wing, …)
is out of scope for the first generator; start with a fixed clue count (e.g.
30–40 for 9×9) and add grading once validation and seeding work.

### Validation dependency

Solution generation and puzzle seeding both need **conflict detection**: can
digit `d` go at `(row, col)` given current cell values?

This ended up as two functions rather than one, because the two layers hold
different data: `Puzzle::validate` works on a dense `Vec<Vec<i32>>` with `0` as
the generation-time empty sentinel, while `Board::is_valid_move` works on
`Vec<Vec<Option<i32>>>` and additionally rejects already-occupied cells. Same
rule, different representations — worth revisiting if they ever drift.

### Module layout

Generation lives outside the TUI loop and outside rendering, as intended, though
it sits on `Puzzle` rather than a separate `generator` module — the struct is
small enough that a third module wasn't earning its keep.

### Phased implementation order

| Step | Delivers | Status |
|---|---|---|
| 1. Conflict check | Shared rule check | ✅ `Puzzle::validate` / `Board::is_valid_move` |
| 2. Solution generation | Full legal grid | ✅ `Puzzle::seed` (iterative backtracking) |
| 3. `count_solutions` / uniqueness | Puzzle safety check | ❌ not started |
| 4. Clue selection | Partial grid + clue count | ⚠️ `Puzzle::mask`, random, no uniqueness guarantee |
| 5. Transforms + difficulty | Variety and grading | ❌ not started |

Step 3 is the meaningful gap: until a solution counter exists, Step 4 can produce
puzzles with multiple valid answers. In practice the game hides this, because
moves are checked against the stored solution rather than solved independently —
a player deducing a *different* legal answer would be told they're wrong.

### Testing strategy

- [x] **Conflict detection:** known valid/invalid placements on a partially
  filled board (`test_validate`, `test_is_valid_move_*`).
- [x] **Solution generation:** output passes full-grid validation — every row,
  column, and box is a permutation of 1–9 (`test_seed`,
  `test_generate_solution_valid`).
- [x] **Clue mask:** correct shape and exact revealed-cell count, invariants that
  hold regardless of the random shuffle (`test_mask`).
- [ ] **Uniqueness:** seeded partial grid has exactly one solution — blocked on a
  solution counter.
- [ ] **Regression:** solver completes a known published puzzle to the expected
  answer.

Deterministic tests use `Puzzle::new(solution, mask)` to build a fixed puzzle
rather than going through `generate`, so assertions don't depend on the RNG.
`seed` and `mask` still take a `ThreadRng` directly, which can't be seeded — if
generation itself ever needs reproducible tests, those signatures would need to
take a generic `impl Rng`.

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

## Done criteria (interactive board MVP) ✅

- [x] Board renders with box-drawing characters
- [x] Board can be created empty and mutated in memory
- [x] In-place redraw loop (clear screen + re-render)
- [x] Visible cursor navigation
- [x] Set/clear at cursor position
- [x] Raw key input (no Enter required)
- [x] Clean terminal restore on exit

## Vision

- [x] Play entirely in the terminal through a TUI
- [x] Every move checked against Sudoku rules before it sticks
- [x] Invalid moves reported and refused so the board stays legal
- [ ] Recognize a completed puzzle and end the game
