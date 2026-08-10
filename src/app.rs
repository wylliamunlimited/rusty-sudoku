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

pub struct App {
    pub(crate) game: Game,
}

impl App {
    pub fn new(board: Board) -> Self {
        App {
            game: Game::new(board),
        }
    }

    pub fn handle_input(&mut self, input: Input) -> bool {
        match input {
            Input::Up => self.move_cursor(Direction::Up),
            Input::Down => self.move_cursor(Direction::Down),
            Input::Left => self.move_cursor(Direction::Left),
            Input::Right => self.move_cursor(Direction::Right),
            Input::Digit(val) => self.game.set_current_cell(val),
            Input::Erase => self.game.clear_current_cell(),
            Input::Confirm => {}
            Input::Back => return false, // Terminate
        }
        true
    }

    fn move_cursor(&mut self, d: Direction) {
        self.game.last_error = None;
        self.game.shift_cursor(d);
    }

    pub fn tick(&mut self) {
        self.game.tick();
    }

    pub fn view(&self) -> String {
        self.game.view()
    }
}
