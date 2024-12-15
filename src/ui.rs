use ratatui::crossterm::event::KeyEvent;
use ratatui::style::{Modifier, Style};

use crate::app::{App, EditField, Station};

use tui_textarea::TextArea;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    text::Span,
    widgets::{block::title::Title, Block, BorderType, Borders, List, Paragraph},
};

// Representation of the UI elements
pub struct UI<'a> {
    name_input: TextArea<'a>,
    url_input: TextArea<'a>,
}

impl<'a> UI<'a> {
    // Create a new blank UI element
    pub fn new() -> UI<'a> {
        UI {
            name_input: TextArea::default(),
            url_input: TextArea::default(),
        }
    }

    // Update the UI given an app and draw it into a frame
    pub fn update(&mut self, f: &mut ratatui::Frame, app: &mut App) {
        self.show_list(f, app);
        if app.is_add_mode() || app.is_edit_mode() {
            self.focus_edit_field(app);
            self.show_edit(f);
        }
    }

    // Show the list of stations. Uses the app's list state to select items.
    fn show_list(&mut self, f: &mut ratatui::Frame, app: &mut App) {
        let list = app
            .stations
            .iter()
            .enumerate()
            .map(|(i, station)| {
                if Some(i) == app.current_station && app.is_playing() {
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
        let has_title = app.player.current_title.is_some() && app.is_playing();
        let mut constraints = vec![Constraint::Fill(1)];
        if has_title {
            constraints.push(Constraint::Max(3));
        }
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(f.area());

        f.render_stateful_widget(&list, layout[0], &mut app.list_state);

        if has_title {
            f.render_widget(
                Paragraph::new(app.player.current_title.clone().unwrap_or(" - ".to_owned())).block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                layout[1],
            )
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

    pub fn init_edit(&mut self) {
        self.name_input.set_cursor_line_style(Style::default());
        self.name_input.set_placeholder_text("Name");
        self.url_input.set_cursor_line_style(Style::default());
        self.url_input
            .set_cursor_style(self.name_input.cursor_line_style());
        self.url_input.set_placeholder_text("URL");
        self.name_input = TextArea::default();
        self.url_input = TextArea::default();
    }

    pub fn fill_edit_form(&mut self, station: &Station) {
        self.name_input.insert_str(station.name.clone());
        self.url_input.insert_str(station.url.clone());
    }

    pub fn update_textfields(&mut self, app: &App, key: KeyEvent) {
        match app.edit_field {
            EditField::Name => {
                self.name_input.input(key);
            }
            EditField::Url => {
                self.url_input.input(key);
            }
        };
    }

    pub fn focus_edit_field(&mut self, app: &App) {
        match app.edit_field {
            EditField::Name => {
                self.url_input
                    .set_cursor_style(self.url_input.cursor_line_style());
                self.name_input
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
            EditField::Url => {
                self.name_input
                    .set_cursor_style(self.name_input.cursor_line_style());
                self.url_input
                    .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            }
        };
    }

    pub fn name(&self) -> String {
        self.name_input.lines().join("")
    }

    pub fn url(&self) -> String {
        self.url_input.lines().join("")
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
