use dirs::config_local_dir;
use ratatui::widgets::ListItem;

use serde::{Deserialize, Serialize};

use crate::player::{Player, PlayerState};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Station {
    pub name: String,
    pub url: String,
}

impl<'a> From<&Station> for ListItem<'a> {
    fn from(val: &Station) -> Self {
        ListItem::new(val.name.clone())
    }
}

#[derive(Debug)]
pub struct App {
    pub stations: Vec<Station>,
    pub player: Player,
    pub current_station: Option<usize>,
    pub current_selection: Option<usize>,
    pub current_edit: Option<Station>,
    exit: bool,
}

fn load_stations() -> Option<std::vec::Vec<Station>> {
    let config_dir = config_local_dir()?;
    let file_contents = std::fs::read_to_string(config_dir.join("rdo").join("stations.csv"));
    if let Ok(stations_str) = file_contents {
        return csv::Reader::from_reader(stations_str.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Station>, csv::Error>>()
            .ok();
    }
    None
}

impl App {
    pub fn new() -> Self {
        App {
            stations: load_stations().unwrap_or(vec![]),
            player: Player::new(),
            current_station: None,
            current_selection: Some(1),
            current_edit: None,
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
