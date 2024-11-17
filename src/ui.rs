use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::style::{Modifier, Style};

use crate::app::{App, EditMode, Station};

use tui_textarea::TextArea;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color},
    widgets::{block::title::Title, Block, BorderType, Borders, List, ListState, Paragraph},
};

pub struct UI<'a> {
    name_input: TextArea<'a>,
    url_input: TextArea<'a>,
}


impl<'a> UI<'a> {
    pub fn new() -> UI<'a> {
        UI {
            name_input: TextArea::default(),
            url_input: TextArea::default(),
        }
    }

    pub fn update(&mut self, f: &mut ratatui::Frame, app: &App) {
        if let Some(_) = app.current_edit.clone() {
            self.show_edit(f);
        }
    }

    fn show_edit(&mut self, f: &mut ratatui::Frame) {
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

        let lblock = Block::default().borders(Borders::ALL).title("Name");
        let rblock = Block::default().borders(Borders::ALL).title("URL");
        f.render_widget(&lblock, popup_chunks[0]);
        f.render_widget(&self.name_input, lblock.inner(popup_chunks[0]));
        f.render_widget(&rblock, popup_chunks[1]);
        f.render_widget(&self.url_input, rblock.inner(popup_chunks[1]));

        f.render_widget(popup_block, area);
    }

    pub fn begin_edit(&mut self, station: Station) {
        self.name_input.insert_str(station.name);
        self.url_input.insert_str(station.url);
    }

    pub fn handle_edit(&mut self, app: &mut App, event: Event) {
        match event {
            Event::Key(key) if key.code == KeyCode::Enter => self.save_station(app),
            Event::Key(key) if key.code == KeyCode::Tab => self.toggle_edit_field(app),
            Event::Key(key) if key.code == KeyCode::Esc => self.exit_edit_mode(app),
            Event::Key(key) => self.update_textfields(app, key),
            _ => {}
        }
    }

    pub fn save_station(&mut self, app: &mut App) {
        app.save_station();
        self.exit_edit_mode(app);
    }

    fn update_textfields(&mut self, app: &App, key: KeyEvent) {
        match app.edit_mode {
            EditMode::Name => {
                self.name_input.input(key);
            }
            EditMode::Url => {
                self.url_input.input(key);
            }
        };
    }
    fn toggle_edit_field(&mut self, app: &mut App) {
        app.edit_mode = app.edit_mode.toggle();
        match app.edit_mode {
            EditMode::Name => {
                self.url_input
                    .set_cursor_style(self.url_input.cursor_line_style());
                self.name_input
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
            EditMode::Url => {
                self.name_input
                    .set_cursor_style(self.name_input.cursor_line_style());
                self.url_input
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
        };
    }

    fn exit_edit_mode(&mut self, app: &mut App) {
        app.current_edit = None;
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
