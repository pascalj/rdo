use ratatui::crossterm::event::KeyEvent;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Wrap;

use crate::app::{App, EditField, Mode, Station};

use tui_textarea::TextArea;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Color,
    text::Span,
    widgets::{block::title::Title, Block, BorderType, Borders, Clear, List, Padding, Paragraph},
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
        match app.mode {
            Mode::Add | Mode::Edit(_) => {
                self.focus_edit_field(app);
                self.show_add_edit(app.mode, f);
            }
            Mode::Delete(index) => self.show_confirm_delete(app, f, index),
            _ => {}
        }
    }

    fn show_confirm_delete(&mut self, app: &App, f: &mut ratatui::Frame, index: usize) {
        let area = centered_rect(55, 5, f.area());
        let name = app
            .stations
            .get(index)
            .map(|station| station.name.clone())
            .unwrap_or("Invalid station".to_string());
        let message = format!(
            "Do you really want to delete '{}'? Press <enter> to confirm. Press <esc> to cancel.",
            name
        );
        let block = Paragraph::new(message)
            .block(
                Block::bordered()
                    .title("Delete?")
                    .padding(Padding::new(0, 0, 0, 0)),
            )
            .centered()
            .wrap(Wrap { trim: false });
        f.render_widget(Clear, area);
        f.render_widget(block, area);
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
                Paragraph::new(
                    app.player
                        .current_title
                        .clone()
                        .unwrap_or(" - ".to_string()),
                )
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
                layout[1],
            )
        }
    }

    fn show_add_edit(&mut self, mode: Mode, f: &mut ratatui::Frame) {
        let title = match mode {
            Mode::Add => "Add station",
            Mode::Edit(_) => "Edit station",
            _ => "",
        };
        let popup_block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .padding(ratatui::widgets::Padding::horizontal(5))
            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 8, f.area());

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

    pub fn init_edit(&mut self, station: Option<&Station>) {
        self.name_input.set_cursor_line_style(Style::default());
        self.name_input.set_placeholder_text("Name");
        self.url_input.set_cursor_line_style(Style::default());
        self.url_input
            .set_cursor_style(self.name_input.cursor_line_style());
        self.url_input.set_placeholder_text("URL");
        self.name_input = TextArea::default();
        self.url_input = TextArea::default();
        if let Some(station) = station {
            self.name_input.insert_str(station.name.clone());
            self.url_input.insert_str(station.url.clone());
        }
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

pub fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
