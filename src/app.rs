use crate::board::Board;
use crate::game::{Direction, Game};

pub struct App {
    pub(crate) game: Game,
}

pub enum Action {
    Move(Direction),
    SetDigit(i32),
    ClearCell,
    Quit,
}

impl App {
    pub fn new(board: Board) -> Self {
        App {
            game: Game::new(board),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> bool {
        match action {
            Action::Move(d) => {
                self.game.last_error = None;
                self.game.shift_cursor(d);
                true
            }
            Action::SetDigit(val) => {
                self.game.set_current_cell(val);
                true
            }
            Action::ClearCell => {
                self.game.clear_current_cell();
                true
            }
            Action::Quit => {
                false // Terminate
            }
        }
    }

    pub fn tick(&mut self) {
        self.game.tick();
    }

    pub fn view(&self) -> String {
        self.game.view()
    }
}
