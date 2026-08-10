use crate::board::Board;
use crate::game::{Direction, Game};

/// A keypress, stripped of any meaning that depends on what's on screen.
///
/// `tui` maps physical keys to these and stops there; deciding what `Up` *does*
/// is `App`'s job, because that answer changes per screen and belongs somewhere
/// testable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Digit(i32),
    Erase,
    Confirm,
    Back,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Menu,
    Game,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuItem {
    NewGame,
    Continue,
    Quit,
}

/// What `App` needs `main` to do once an input has been handled.
///
/// A plain `bool` can't carry this any more: "back to the menu" and "exit the
/// program" are both non-quit outcomes, and `NewGame` is a request for an
/// *effect* (RNG) that `App` deliberately can't perform itself.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Request {
    Continue,
    NewGame,
    Exit,
}

pub struct App {
    // Private, and only ever changed through the transition methods below. The
    // pair (Screen::Game, game: None) is representable but invalid, so it gets
    // gated in one place - the same move as Board::set_cell_gated.
    screen: Screen,
    game: Option<Game>,
    selected: MenuItem,
}

impl MenuItem {
    const ALL: [MenuItem; 3] = [MenuItem::NewGame, MenuItem::Continue, MenuItem::Quit];

    fn label(self) -> &'static str {
        match self {
            MenuItem::NewGame => "New Game",
            MenuItem::Continue => "Continue",
            MenuItem::Quit => "Quit",
        }
    }
}

impl App {
    /// Matches the board's rendered width, so the menu and the grid line up.
    const WIDTH: usize = 37;

    pub fn new() -> Self {
        App {
            screen: Screen::Menu,
            game: None,
            selected: MenuItem::NewGame,
        }
    }

    // --- Read-only views of the gated state ---

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn selected(&self) -> MenuItem {
        self.selected
    }

    pub fn game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    pub fn has_game(&self) -> bool {
        self.game.is_some()
    }

    // --- Transitions: the only things allowed to touch screen/game ---

    pub fn start_game(&mut self, board: Board) {
        self.game = Some(Game::new(board));
        self.screen = Screen::Game;
    }

    pub fn resume(&mut self) {
        if self.game.is_some() {
            self.screen = Screen::Game;
        }
    }

    pub fn open_menu(&mut self) {
        self.screen = Screen::Menu;
    }

    // --- Input ---

    pub fn handle_input(&mut self, input: Input) -> Request {
        match self.screen {
            Screen::Menu => self.handle_menu_input(input),
            Screen::Game => self.handle_game_input(input),
        }
    }

    fn handle_menu_input(&mut self, input: Input) -> Request {
        match input {
            Input::Up => {
                self.shift_selection(Direction::Up);
                Request::Continue
            }
            Input::Down => {
                self.shift_selection(Direction::Down);
                Request::Continue
            }
            Input::Confirm => match self.selected {
                // App can't generate a puzzle without reaching for the RNG, so
                // it asks main to do it and hand the board back instead.
                MenuItem::NewGame => Request::NewGame,
                MenuItem::Continue => {
                    self.resume();
                    Request::Continue
                }
                MenuItem::Quit => Request::Exit,
            },
            Input::Back => Request::Exit,
            _ => Request::Continue,
        }
    }

    fn handle_game_input(&mut self, input: Input) -> Request {
        // Handled before the borrow below, because open_menu needs &mut self
        // and self.game.as_mut() would still be holding a piece of it.
        if input == Input::Back {
            self.open_menu();
            return Request::Continue;
        }

        let Some(game) = self.game.as_mut() else {
            // Unreachable via the transition methods; recover instead of panic.
            self.screen = Screen::Menu;
            return Request::Continue;
        };

        match input {
            Input::Up => game.move_cursor(Direction::Up),
            Input::Down => game.move_cursor(Direction::Down),
            Input::Left => game.move_cursor(Direction::Left),
            Input::Right => game.move_cursor(Direction::Right),
            Input::Digit(val) => game.set_current_cell(val),
            Input::Erase => game.clear_current_cell(),
            Input::Confirm | Input::Back => {}
        }

        Request::Continue
    }

    /// Whether an item can be landed on. `Continue` is dead until a game exists.
    fn is_selectable(&self, item: MenuItem) -> bool {
        match item {
            MenuItem::Continue => self.game.is_some(),
            _ => true,
        }
    }

    /// One step through the item list, clamped at both ends.
    fn step(&self, item: MenuItem, op: Direction) -> MenuItem {
        let i = MenuItem::ALL.iter().position(|&m| m == item).unwrap_or(0);
        let next = match op {
            Direction::Up => i.saturating_sub(1),
            Direction::Down => (i + 1).min(MenuItem::ALL.len() - 1),
            _ => i,
        };
        MenuItem::ALL[next]
    }

    /// Same shape as `Game::shift_cursor`: step, skip what can't be landed on,
    /// and give up rather than loop when the rest of that direction is dead.
    fn shift_selection(&mut self, op: Direction) {
        let mut candidate = self.step(self.selected, op);
        while !self.is_selectable(candidate) {
            let next = self.step(candidate, op);
            if next == candidate {
                return;
            }
            candidate = next;
        }
        self.selected = candidate;
    }

    // --- Output ---

    pub fn tick(&mut self) {
        if let Some(game) = self.game.as_mut() {
            game.tick();
        }
    }

    pub fn view(&self) -> String {
        match (self.screen, &self.game) {
            (Screen::Game, Some(game)) => game.view(),
            _ => self.menu_view(),
        }
    }

    fn menu_view(&self) -> String {
        let inner = Self::WIDTH - 2;
        let rule = "═".repeat(inner);

        let mut out = format!("╔{rule}╗\n");
        out.push_str(&Self::centered("RUSTY SUDOKU", inner));
        out.push_str(&format!("╚{rule}╝\n\n"));

        for item in MenuItem::ALL {
            let marker = if item == self.selected { '▸' } else { ' ' };
            out.push_str(&format!("   {marker} {}", item.label()));
            if !self.is_selectable(item) {
                out.push_str("  ·  no game in progress");
            }
            out.push('\n');
        }

        out.push_str("\n↑↓ move · Enter select · q quit\n");
        out
    }

    fn centered(text: &str, width: usize) -> String {
        // chars().count(), not len() - len() is bytes, and these are box-drawing
        // characters that take more than one byte each.
        let pad = width.saturating_sub(text.chars().count());
        let left = pad / 2;
        format!("║{}{text}{}║\n", " ".repeat(left), " ".repeat(pad - left))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
