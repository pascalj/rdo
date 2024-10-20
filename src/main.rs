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
    let mut url_input = TextArea::default();
    name_input.set_cursor_line_style(Style::default());
    name_input.set_placeholder_text("Name");
    url_input.set_cursor_line_style(Style::default());
    url_input.set_cursor_style(name_input.cursor_line_style());
    url_input.set_placeholder_text("URL");
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
                    .title("Edit station")
                    .title_alignment(Alignment::Center)
                    .padding(ratatui::widgets::Padding::horizontal(5))
                    .style(Style::default().bg(Color::DarkGray));
                let area = centered_rect(60, 18, f.area());

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
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
                let lblock = Block::default().borders(Borders::ALL).title("Name");
                let rblock = Block::default().borders(Borders::ALL).title("URL");
                f.render_widget(&lblock, popup_chunks[0]);
                f.render_widget(&name_input, lblock.inner(popup_chunks[0]));
                f.render_widget(&rblock, popup_chunks[1]);
                f.render_widget(&url_input, rblock.inner(popup_chunks[1]));

                // let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
                // frame.render_widget(value_text, popup_chunks[1]);

                f.render_widget(popup_block, area);
            }
        })?;

        app.update_status();

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Some(_) = app.current_edit {
                    ui.handle_edit(app, event::read()?);
                    if let Event::Key(key) = event::read()? {
                        // Your own key mapping to break the event loop
                        match key.code {
                            KeyCode::Tab => {
                                app.edit_mode = app.edit_mode.toggle();
                                match app.edit_mode {
                                    app::EditMode::Name => {
                                        url_input.set_cursor_style(url_input.cursor_line_style());
                                        name_input.set_cursor_style(
                                            Style::default().add_modifier(Modifier::REVERSED),
                                        );
                                    }
                                    app::EditMode::Url => {
                                        name_input.set_cursor_style(name_input.cursor_line_style());
                                        url_input.set_cursor_style(
                                            Style::default().add_modifier(Modifier::REVERSED),
                                        );
                                    }
                                };
                            }
                            KeyCode::Esc => {
                                app.abort_edit()
                            }
                            KeyCode::Enter => {
                                app.update_current()
                            }
                            _ => {
                                UI.handle_edit(app, event)
                            }
                        }
                    }
                } else if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.exit();
                                return Ok(());
                            }
                            KeyCode::Char('e') => {
                                if let Some(to_edit) = app
                                    .current_selection
                                    .and_then(|i| app.stations.get(i).cloned())
                                {
                                    name_input.insert_str(to_edit.name.clone());
                                    url_input.insert_str(to_edit.url.clone());
                                    app.current_edit = Some(to_edit)
                                }
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
