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
                    match app.mode {
                        Mode::Add | Mode::Edit(_) => handle_edit_mode(&mut app, &mut ui, key)?,
                        Mode::Delete(_) => handle_delete_mode(&mut app, &mut ui, key)?,
                        _ => handle_normal_mode(&mut app, &mut ui, key)?,
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
                Mode::Edit(i) => app.update_station(i, ui.name(), ui.url())?,
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
    match (key.code, app.selected_index()) {
        (KeyCode::Char('e'), Some(i)) => {
            if let Some(station) = app.stations.get(i) {
                ui.init_edit();
                ui.fill_edit_form(&station);
                app.mode = Mode::Edit(i);
            }
        }
        (KeyCode::Char('d'), Some(i)) => app.mode = Mode::Delete(i),
        (KeyCode::Char('q'), _) => app.mode = Mode::Exit,
        (KeyCode::Char('n'), _) => app.mode = Mode::Add,
        (KeyCode::Char('k'), _) => app.select_previous(),
        (KeyCode::Char('j'), _) => app.select_next(),
        (KeyCode::Char(' '), _) => app.stop(),
        (KeyCode::Enter, Some(i)) => app.change_station(i),
        (KeyCode::Up, _) => app.select_previous(),
        (KeyCode::Down, _) => app.select_next(),
        _ => {}
    };
    Ok(())
}

fn handle_delete_mode(app: &mut app::App, _: &mut ui::UI, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Enter => {
            if let Some(index) = app.selected_index() {
                app.delete_station(index)?;
            }
            app.mode = Mode::Normal
        }
        _ => {}
    };
    Ok(())
}
