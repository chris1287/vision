use rand::seq::SliceRandom;
use ratatui::{
    prelude::*,
    widgets::{Block, Padding},
};

#[derive(Debug, Default)]
pub struct ChessBoard {
    cells: Vec<Cell>,
    position: String,
}

#[derive(Debug, Default)]
pub struct Cell {
    x: u16,
    y: u16,
    fg: Color,
    bg: Color,
    highlight: bool,
    identifier: String,
}

impl ChessBoard {
    pub fn new() -> Self {
        let light = Color::Rgb(238, 238, 210);
        let dark = Color::Rgb(118, 150, 86);
        let mut grid = Vec::new();
        for x in (0..8).rev() {
            for y in 0..8 {
                let mut cell = Cell {
                    highlight: false,
                    x,
                    y,
                    ..Default::default()
                };
                if (x + y) % 2 == 0 {
                    cell.fg = light;
                } else {
                    cell.fg = dark;
                }
                cell.identifier = match (x, y) {
                    (0, 0) => "a8",
                    (1, 0) => "b8",
                    (2, 0) => "c8",
                    (3, 0) => "d8",
                    (4, 0) => "e8",
                    (5, 0) => "f8",
                    (6, 0) => "g8",
                    (7, 0) => "h8",
                    (0, 1) => "a7",
                    (1, 1) => "b7",
                    (2, 1) => "c7",
                    (3, 1) => "d7",
                    (4, 1) => "e7",
                    (5, 1) => "f7",
                    (6, 1) => "g7",
                    (7, 1) => "h7",
                    (0, 2) => "a6",
                    (1, 2) => "b6",
                    (2, 2) => "c6",
                    (3, 2) => "d6",
                    (4, 2) => "e6",
                    (5, 2) => "f6",
                    (6, 2) => "g6",
                    (7, 2) => "h6",
                    (0, 3) => "a5",
                    (1, 3) => "b5",
                    (2, 3) => "c5",
                    (3, 3) => "d5",
                    (4, 3) => "e5",
                    (5, 3) => "f5",
                    (6, 3) => "g5",
                    (7, 3) => "h5",
                    (0, 4) => "a4",
                    (1, 4) => "b4",
                    (2, 4) => "c4",
                    (3, 4) => "d4",
                    (4, 4) => "e4",
                    (5, 4) => "f4",
                    (6, 4) => "g4",
                    (7, 4) => "h4",
                    (0, 5) => "a3",
                    (1, 5) => "b3",
                    (2, 5) => "c3",
                    (3, 5) => "d3",
                    (4, 5) => "e3",
                    (5, 5) => "f3",
                    (6, 5) => "g3",
                    (7, 5) => "h3",
                    (0, 6) => "a2",
                    (1, 6) => "b2",
                    (2, 6) => "c2",
                    (3, 6) => "d2",
                    (4, 6) => "e2",
                    (5, 6) => "f2",
                    (6, 6) => "g2",
                    (7, 6) => "h2",
                    (0, 7) => "a1",
                    (1, 7) => "b1",
                    (2, 7) => "c1",
                    (3, 7) => "d1",
                    (4, 7) => "e1",
                    (5, 7) => "f1",
                    (6, 7) => "g1",
                    (7, 7) => "h1",
                    _ => panic!("Invalid position"),
                }
                .to_string();
                grid.push(cell);
            }
        }
        Self {
            cells: grid,
            position: "".to_string(),
        }
    }

    pub fn position(&self) -> String {
        self.position.clone()
    }

    pub fn pick_random_cell(&mut self) {
        let mut rng = rand::thread_rng();
        self.cells.iter_mut().for_each(|x| x.highlight = false);
        if let Some(x) = self.cells.choose_mut(&mut rng) {
            x.highlight = true;
            self.position = x.identifier.clone();
        }
    }
}

impl Widget for &mut ChessBoard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width / 8;
        let height = width / 2;
        let block = Block::default().padding(Padding::new(1, 1, 1, 1));
        let inner = block.inner(area);
        block.render(area, buf);
        for cell in &self.cells {
            let x = inner.left() + (cell.x * width);
            let y = inner.top() + (cell.y * height);
            for w in 0..width {
                for h in 0..height {
                    if let Some(c) = buf.cell_mut((x + w, y + h)) {
                        let symbol = '█';
                        // let symbol = {
                        //     if w%2 == 0 {
                        //         cell.identifier.chars().nth(0).unwrap()
                        //     } else {
                        //         cell.identifier.chars().nth(1).unwrap()
                        //     }
                        // };
                        c.set_char(symbol);
                        c.fg = if cell.highlight { Color::Red } else { cell.fg };
                        c.bg = cell.bg;
                    }
                }
            }
        }
    }
}
