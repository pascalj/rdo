use crate::app::App;

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{block::title::Title, Block, BorderType, Borders, List, ListState, Paragraph},
    Terminal,
};
use std::{io, time::Duration};
use tui_textarea::TextArea;

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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

// TODO: factor this out into a UI module
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut list_state = ListState::default();
    let mut name_input = TextArea::default();
    list_state.select(app.current_selection);
    loop {
        let list = app
            .stations
            .iter()
            .enumerate()
            .map(|(i, station)| {
                if Some(i) == app.current_station && app.state() == player::PlayerState::Playing {
                    return Span::styled(
                        &station.name,
                        Style::default().add_modifier(Modifier::BOLD),
                    );
                }
                return Span::raw(&station.name);
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
            let has_title =
                app.player.current_title.is_some() && app.state() == player::PlayerState::Playing;
            let mut constraints = vec![Constraint::Fill(1)];
            if has_title {
                constraints.push(Constraint::Max(3));
            }
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(f.area());
            f.render_stateful_widget(&list, layout[0], &mut list_state);
            if has_title {
                f.render_widget(
                    Paragraph::new(app.player.current_title.clone().unwrap_or(" - ".to_owned()))
                        .block(
                            Block::new()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded),
                        ),
                    layout[1],
                )
            }

            if let Some(station) = app.current_edit.clone() {
                let popup_block = Block::default()
                    .title("Enter a new key-value pair")
                    .borders(Borders::NONE)
                    .style(Style::default().bg(Color::DarkGray));
                let area = centered_rect(60, 25, f.area());

                let popup_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(area);

                // let mut key_block = Block::default().title("Key").borders(Borders::ALL);
                // let mut value_block = Block::default().title("Value").borders(Borders::ALL);

                // let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

                // key_block = key_block.style(active_style);
                // match editing {
                //     CurrentlyEditing::Key => key_block = key_block.style(active_style),
                //     CurrentlyEditing::Value => value_block = value_block.style(active_style),
                // };

                // let key_text = Paragraph::new(station.name).block(key_block);
                f.render_widget(&name_input, popup_chunks[0]);

                // let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
                // frame.render_widget(value_text, popup_chunks[1]);

                f.render_widget(popup_block, area);
            }
        })?;

        app.update_status();

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Some(_) = app.current_edit {
                    if let Event::Key(key) = event::read()? {
                        // Your own key mapping to break the event loop
                        if key.code == KeyCode::Esc {
                            app.current_edit = None;
                            continue;
                        }
                        // `TextArea::input` can directly handle key events from backends and update the editor state
                        name_input.input(key);
                    }
                } else if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.exit();
                                return Ok(());
                            }
                            KeyCode::Char('e') => {
                                app.current_edit = app
                                    .current_selection
                                    .and_then(|i| app.stations.get(i).cloned());
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
            Ok(false) => {}
            Err(_) => {}
        }
    }
}
