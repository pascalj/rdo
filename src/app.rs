use ratatui::widgets::ListItem;

#[derive(Debug)]
pub struct Station {
    name: String,
    url: String,
}

impl<'a> From<&Station> for ListItem<'a> {
    fn from(val: &Station) -> Self {
        ListItem::new(val.name.clone())
    }
}

impl Station {
    fn new(name: &str, url: &str) -> Self {
        Self {
            name: String::from(name),
            url: String::from(url),
        }
    }
}

#[derive(Debug)]
pub enum PlayerState {
    Playing,
    Stopped,
    Buffering,
}

#[derive(Debug)]
pub struct App {
    selected_index: usize,
    player_state: PlayerState,
    pub stations: Vec<Station>,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            selected_index: 0,
            player_state: PlayerState::Stopped,
            stations: vec![Station::new("name", "url"), Station::new("name2", "url2")],
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn select_up(&mut self) {
        self.selected_index = std::cmp::min(self.stations.len() - 1, self.selected_index + 1)
    }

    pub fn select_down(&mut self) {
        self.selected_index = std::cmp::max(0, self.selected_index - 1);
    }
}
