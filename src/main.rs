use crate::app::App;

use std::{io, time::Duration};

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{block::title::Title, Block, BorderType, Borders, List, ListState, Paragraph},
    Terminal,
};

mod app;
mod player;
mod ui;

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
    list_state.select(app.current_selection);
    loop {
        let list = app
            .stations
            .iter()
            .enumerate()
            .map(|(i, item)| {
                if Some(i) == app.current_station && app.state() == player::PlayerState::Playing {
                    return Span::styled(&item.name, Style::default().add_modifier(Modifier::BOLD));
                }
                return Span::raw(&item.name);
            })
            .collect::<List>()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(Title::from("rdo").alignment(Alignment::Center)),
            )
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .repeat_highlight_symbol(false);

        // terminal.draw(|f| ui(f, app))?;
        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Max(3)])
                .split(f.area());
            f.render_stateful_widget(&list, layout[0], &mut list_state);
            f.render_widget(
                Paragraph::new(app.player.current_title.clone().unwrap_or(" - ".to_owned())).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                layout[1],
            )
        })?;

        match event::poll(Duration::from_millis(50)) {
            Ok(true) => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.exit();
                                return Ok(());
                            }
                            KeyCode::Up => {
                                list_state.select_previous();
                            }
                            KeyCode::Down => {
                                list_state.select_next();
                            }
                            KeyCode::Enter => {
                                if let Some(selected) = list_state.selected() {
                                    app.change_station(selected);
                                };
                            }
                            KeyCode::Char(' ') => {
                                app.stop();
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(false) => {
                app.update_status();
            }
            Err(_) => {}
        }
    }
}
