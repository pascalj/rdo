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
    // name_input.set_cursor_line_style(Style::default());
    // name_input.set_placeholder_text("Name");
    // url_input.set_cursor_line_style(Style::default());
    // url_input.set_cursor_style(name_input.cursor_line_style());
    // url_input.set_placeholder_text("URL");
    // list_state.select(app.current_selection);
    loop {
        app.update_status();

        terminal.draw(|f| {
            ui.update(f, &app);
        })?;

        match event::poll(Duration::from_millis(100)) {
            Ok(true) => {
                if ui.is_editing() {
                    ui.handle_edit(&mut app, event::read()?);
                } else if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.exit();
                            return Ok(());
                        }
                        KeyCode::Char('e') => {
                            ui.selected_index()
                                .and_then(|i| app.stations.get(i).cloned())
                                .map(|station| ui.begin_edit(&station));
                        }
                        KeyCode::Up => ui.select_previous(),
                        KeyCode::Down => {
                            ui.select_next();
                        }
                        KeyCode::Enter => {
                            if let Some(selected) = ui.selected_index() {
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
            Ok(false) => {}
            Err(_) => {}
        }
    }
}
