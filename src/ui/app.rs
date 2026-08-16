use crate::sudoku::Board;
use crate::ui::{Cloud, Direction, Game};

use std::time::{Duration, Instant};

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
    Click(u16, u16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Menu,
    Game,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuItem {
    NewGame,
    Size,
    Continue,
    Quit,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Request {
    Continue,
    NewGame,
    Exit,
}

pub struct App {
    screen: Screen,
    game: Option<Game>,
    selected: MenuItem,
    box_size: usize,
    logo: Cloud,
    frame: u64,
    last_frame_time: Instant,
}

impl MenuItem {
    const ALL: [MenuItem; 4] = [
        MenuItem::NewGame,
        MenuItem::Size,
        MenuItem::Continue,
        MenuItem::Quit,
    ];

    fn label(self) -> &'static str {
        match self {
            MenuItem::NewGame => "New Game",
            MenuItem::Size => "Size",
            MenuItem::Continue => "Continue",
            MenuItem::Quit => "Quit",
        }
    }
}

impl App {
    const WIDTH: usize = 37;
    const LOGO_HEIGHT: usize = 8;
    const BOX_SIZES: [usize; 3] = [2, 3, 4];
    const DEFAULT_BOX_SIZE: usize = 3;
    const FRAME_INTERVAL: Duration = Duration::from_millis(80);

    pub fn new() -> Self {
        App {
            screen: Screen::Menu,
            game: None,
            selected: MenuItem::NewGame,
            box_size: Self::DEFAULT_BOX_SIZE,
            logo: Cloud::cube(14),
            frame: 0,
            last_frame_time: Instant::now(),
        }
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn selected(&self) -> MenuItem {
        self.selected
    }

    pub fn box_size(&self) -> usize {
        self.box_size
    }

    pub fn board_size(&self) -> usize {
        self.box_size * self.box_size
    }

    pub fn game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    pub fn has_game(&self) -> bool {
        self.game.is_some()
    }

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
            Input::Left => {
                self.shift_box_size(Direction::Left);
                Request::Continue
            }
            Input::Right => {
                self.shift_box_size(Direction::Right);
                Request::Continue
            }
            Input::Confirm => match self.selected {
                MenuItem::NewGame => Request::NewGame,
                MenuItem::Size => {
                    self.cycle_box_size();
                    Request::Continue
                }
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
        if input == Input::Back {
            self.open_menu();
            return Request::Continue;
        }

        let Some(game) = self.game.as_mut() else {
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
            Input::Click(line, column) => game.click(line as usize, column as usize),
            Input::Confirm | Input::Back => {}
        }

        Request::Continue
    }

    fn is_selectable(&self, item: MenuItem) -> bool {
        match item {
            MenuItem::Continue => self.game.is_some(),
            _ => true,
        }
    }

    fn step(&self, item: MenuItem, op: Direction) -> MenuItem {
        let i = MenuItem::ALL.iter().position(|&m| m == item).unwrap_or(0);
        let next = match op {
            Direction::Up => i.saturating_sub(1),
            Direction::Down => (i + 1).min(MenuItem::ALL.len() - 1),
            _ => i,
        };
        MenuItem::ALL[next]
    }

    fn box_size_index(&self) -> usize {
        Self::BOX_SIZES
            .iter()
            .position(|&b| b == self.box_size)
            .unwrap_or(0)
    }

    fn shift_box_size(&mut self, op: Direction) {
        if self.selected != MenuItem::Size {
            return;
        }
        let i = self.box_size_index();
        let next = match op {
            Direction::Left => i.saturating_sub(1),
            Direction::Right => (i + 1).min(Self::BOX_SIZES.len() - 1),
            _ => i,
        };
        self.box_size = Self::BOX_SIZES[next];
    }

    fn cycle_box_size(&mut self) {
        let next = (self.box_size_index() + 1) % Self::BOX_SIZES.len();
        self.box_size = Self::BOX_SIZES[next];
    }

    fn size_label(&self) -> String {
        let size = self.board_size();
        format!("{size}×{size}")
    }

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

    pub fn tick(&mut self) {
        if self.last_frame_time.elapsed() >= Self::FRAME_INTERVAL {
            self.frame = self.frame.wrapping_add(1);
            self.last_frame_time = Instant::now();
        }
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

        let mut out = self.logo.render(self.frame, Self::WIDTH, Self::LOGO_HEIGHT);
        out.push_str(&format!("╔{rule}╗\n"));
        out.push_str(&Self::centered("RUSTY SUDOKU", inner));
        out.push_str(&format!("╚{rule}╝\n\n"));

        for item in MenuItem::ALL {
            let marker = if item == self.selected { '▸' } else { ' ' };
            out.push_str(&format!("   {marker} {}", item.label()));
            match item {
                MenuItem::Size => out.push_str(&format!("  ‹ {} ›", self.size_label())),
                _ if !self.is_selectable(item) => out.push_str("  ·  no game in progress"),
                _ => {}
            }
            out.push('\n');
        }

        out.push_str("\n↑↓ move · ←→ size · Enter select · q quit\n");
        out
    }

    fn centered(text: &str, width: usize) -> String {
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
