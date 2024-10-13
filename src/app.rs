use ratatui::widgets::ListItem;

use crate::player::{Player, PlayerState};

#[derive(Debug)]
pub struct Station {
    pub name: String,
    pub url: String,
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
pub struct App {
    pub stations: Vec<Station>,
    pub player: Player,
    current_station: Option<usize>,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            stations: vec![
                Station::new("Radio Paradise", "http://stream.radioparadise.com/aac-128"),
                Station::new(
                    "detektor.fm",
                    "https://streams.detektor.fm/wort/mp3-256/website/",
                ),
            ],
            player: Player::new(),
            current_station: None,
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn change_station(&mut self, station_index: usize) {
        if let Some(station) = self.stations.get(station_index) {
            self.current_station = Some(station_index);
            self.player.play(station)
        }
    }

    pub fn stop(&mut self) {
        self.player.stop()
    }

    pub fn update_status(&mut self) {
        self.player.update_status()
    }

    pub fn state(&self) -> PlayerState {
        self.player.state()
    }
}
