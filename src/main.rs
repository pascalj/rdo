use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
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
    let app_result = run_app(&mut terminal);
    ratatui::restore();
    disable_raw_mode()?;
    app_result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = app::App::new();
    let mut ui = ui::UI::new();
    loop {
        app.update_status();

        terminal.draw(|f| {
            ui.update(f, &mut app);
        })?;

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Event::Key(key) = event::read()? {
                    if app.is_edit_mode() {
                        match key.code {
                            KeyCode::Enter => {
                                app.selected_index()
                                    .map(|i| app.update_station(i, ui.name(), ui.url()));
                                app.mode = app::Mode::Normal
                            }
                            KeyCode::Tab => {
                                app.edit_field = app.edit_field.toggle();
                            }
                            KeyCode::Esc => app.mode = app::Mode::Normal,
                            _ => ui.update_textfields(&app, key),
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.exit();
                                return Ok(());
                            }
                            KeyCode::Char('e') => {
                                app.selected_index()
                                    .and_then(|i| app.stations.get(i).cloned())
                                    .map(|station| ui.begin_edit(&station));
                                app.mode = app::Mode::Edit
                            }
                            KeyCode::Up => app.select_previous(),
                            KeyCode::Down => {
                                app.select_next();
                            }
                            KeyCode::Enter => {
                                if let Some(selected) = app.selected_index() {
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
