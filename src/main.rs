mod app;
mod player;
mod ui;

use app::{station_file_path, App, Mode};
use log::error;
use ui::UI;

use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    Terminal,
};
use std::{io, time::Duration};

// Main function
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    enable_raw_mode()?;
    terminal.clear()?;
    let app_result = run_loop(&mut terminal);
    ratatui::restore();
    disable_raw_mode()?;
    if let Err(err) = app_result.as_ref() {
        error!("Error in main loop: {err}");
    }
    app_result
}

// Run the main application loop (read inputs, update, display)
fn run_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = station_file_path()
        .map(|path| App::new(path))
        .unwrap_or(App::default());
    let mut ui = UI::new();

    loop {
        app.update_status();

        if app.mode == Mode::Exit {
            return Ok(());
        }

        terminal.draw(|f| {
            ui.update(f, &mut app);
        })?;

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Event::Key(key) = event::read()? {
                    if app.is_add_mode() || app.is_edit_mode() {
                        handle_edit_mode(&mut app, &mut ui, key)?;
                    } else {
                        handle_normal_mode(&mut app, &mut ui, key)?;
                    }
                }
            }
            Ok(false) => {}
            // Unhandled errors
            Err(_) => {}
        }
    }
}

// Handle inputs in the edit mode and update the app state accordingly
fn handle_edit_mode(app: &mut app::App, ui: &mut ui::UI, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Enter => {
            match app.mode {
                Mode::Add => app.add_station(app::Station::new(ui.name(), ui.url()))?,
                Mode::Edit => app
                    .selected_index()
                    .ok_or(std::io::Error::other("Index not found"))
                    .and_then(|i| app.update_station(i, ui.name(), ui.url()))?,
                _ => (),
            }

            app.mode = Mode::Normal;
            Ok(())
        }
        KeyCode::Tab => {
            app.edit_field = app.edit_field.toggle();
            Ok(())
        }
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            Ok(())
        }
        _ => {
            ui.update_textfields(&app, key);
            Ok(())
        }
    }
}

// Handle inputs in normal mode and update the app state accordingly
fn handle_normal_mode(app: &mut app::App, ui: &mut ui::UI, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Char('e') => {
            app.selected_index()
                .and_then(|i| app.stations.get(i).cloned())
                .map(|station| {
                    ui.init_edit();
                    ui.fill_edit_form(&station)
                });
            app.mode = Mode::Edit
        }
        KeyCode::Char('q') => app.mode = Mode::Exit,
        KeyCode::Char('n') => app.mode = Mode::Add,
        KeyCode::Char('k') => app.select_previous(),
        KeyCode::Char('j') => app.select_next(),
        KeyCode::Char(' ') => app.stop(),
        KeyCode::Enter => app.change_station(),
        KeyCode::Up => app.select_previous(),
        KeyCode::Down => app.select_next(),
        _ => {}
    };
    Ok(())
}
