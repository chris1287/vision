use anyhow::Result;

mod app;

use app::App;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
