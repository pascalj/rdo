use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    layout::Alignment,
    style::{Modifier, Style},
    widgets::{block::title::Title, Block, List, ListState},
    Terminal,
};
use std::io;

mod app;
mod ui;

use crate::{app::App, ui::ui};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    enable_raw_mode()?;
    terminal.clear()?;
    let mut app = app::App::new();
    let app_result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    disable_raw_mode()?;
    app_result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut list_state = ListState::default();
    let list = app
        .stations
        .iter()
        .collect::<List>()
        .block(Block::bordered().title(Title::from("rdo").alignment(Alignment::Center)))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(false);
    list_state.select_first();

    loop {
        // terminal.draw(|f| ui(f, app))?;
        terminal.draw(|f| f.render_stateful_widget(&list, f.area(), &mut list_state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Up => {
                        list_state.select_previous();
                    }
                    KeyCode::Down => {
                        list_state.select_next();
                    }
                    _ => {}
                }
            }
        }
    }
}
