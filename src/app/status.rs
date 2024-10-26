use ratatui::{
    prelude::*,
    widgets::{Block, Padding, Paragraph, Wrap},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Unknown,
    Start,
    Success,
    Failure,
}

impl Widget for &mut Status {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = match self {
            Status::Start => "Press 'space' to guess the next position, 'q' to quit",
            Status::Unknown => "",
            Status::Success => "Yes!",
            Status::Failure => "No",
        };
        let text = Span::styled(text, Style::default()).fg(Color::White);
        Paragraph::new(text)
            .block(Block::default().padding(Padding::new(1, 1, 1, 1)))
            .centered()
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
