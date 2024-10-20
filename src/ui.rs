use ratatui::style::{Style, Modifier};
use ratatui::crossterm::event::Event;

use crate::app::{App, EditMode};

use tui_textarea::TextArea;

pub struct UI<'a> {
    name_input: TextArea<'a>,
    url_input: TextArea<'a>,
}


impl<'a> UI<'a> {
    pub fn handle_edit(&mut self, app: &App, event: Event) {
        match (app.edit_mode, event) {
            (EditMode::Name, Event::Key(key))  => {
                self.name_input.input(key);
                self.url_input.set_cursor_style(self.url_input.cursor_line_style());
                self.name_input.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
            (EditMode::Url, Event::Key(key)) => {
                self.url_input.input(key);
                self.name_input.set_cursor_style(self.name_input.cursor_line_style());
                self.url_input.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
            _ => {}
        };
    }
}
