mod chessboard;
mod status;
mod userinput;

use anyhow::Result;
use chessboard::ChessBoard;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{prelude::*, DefaultTerminal};
use status::Status;
use userinput::{UserInput, UserInputState};

#[derive(Debug)]
pub struct App {
    chess_board: ChessBoard,
    user_input: UserInput,
    status: Status,
    is_running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            chess_board: ChessBoard::new(),
            user_input: UserInput::new(),
            status: Status::Start,
            is_running: true,
        }
    }

    pub fn run(mut self, mut tui: DefaultTerminal) -> Result<()> {
        while self.is_running {
            self.draw(&mut tui)?;
            self.handle_input()?;
        }

        Ok(())
    }

    fn draw(&mut self, tui: &mut DefaultTerminal) -> Result<()> {
        tui.draw(|frame| {
            frame.render_widget(self, frame.area());
        })?;
        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => self.is_running = false,
                    KeyCode::Char(' ') => {
                        self.user_input.reset();
                        self.chess_board.pick_random_cell();
                        self.status = Status::Unknown;
                    }
                    KeyCode::Char(c) => {
                        self.user_input.enter_data(c);
                        if self.user_input.state() == UserInputState::Column {
                            let guessed = self.user_input.data();
                            let position = self.chess_board.position();
                            if guessed == position {
                                self.status = Status::Success;
                            } else {
                                self.status = Status::Failure;
                            }
                            self.user_input.reset();
                            self.chess_board.pick_random_cell();
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50), // Chessboard
                Constraint::Percentage(20), // Status
            ]);
        let [l1, l2] = main_layout.areas(area);
        self.chess_board.render(l1, buf);
        self.status.render(l2, buf);
    }
}
