use dirs::config_local_dir;
use std::path::PathBuf;

use ratatui::widgets::{ListItem, ListState};
use serde::{Deserialize, Serialize};

use crate::player::{Player, PlayerState};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Station {
    pub name: String,
    pub url: String,
}

impl Station {
    pub fn new(name: String, url: String) -> Self {
        Station { name, url }
    }
}

impl<'a> From<&Station> for ListItem<'a> {
    fn from(val: &Station) -> Self {
        ListItem::new(val.name.clone())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Normal,
    Edit,
    Add,
    Exit,
}

#[derive(Clone, Copy, Debug)]
pub enum EditField {
    Url,
    Name,
}

impl EditField {
    pub fn toggle(&self) -> Self {
        use EditField::*;
        match self {
            Url => Name,
            Name => Url,
        }
    }
}

pub struct App {
    pub stations: Vec<Station>,
    pub player: Player,
    pub current_station: Option<usize>,
    pub mode: Mode,
    pub edit_field: EditField,
    pub list_state: ListState,
}

// Load stations from the configuration
fn load_stations(path: PathBuf) -> Option<std::vec::Vec<Station>> {
    std::fs::read_to_string(path)
        .map(|stations_str| {
            csv::Reader::from_reader(stations_str.as_bytes())
                .deserialize()
                .collect::<Result<Vec<Station>, csv::Error>>()
                .unwrap_or(vec![])
        })
        .ok()
}

// Path to station.csv
pub fn station_file_path() -> std::option::Option<PathBuf> {
    config_local_dir().map(|config_dir| config_dir.join("rdo").join("stations.csv"))
}

impl Default for App {
    fn default() -> Self {
        App {
            stations: vec![],
            player: Player::new(),
            current_station: None,
            mode: Mode::Normal,
            edit_field: EditField::Name,
            list_state: ListState::default(),
        }
    }
}

// The app state and operations
impl App {
    // Create a new app and load stations from `stations_path`.
    pub fn new(stations_path: PathBuf) -> Self {
        let stations = load_stations(stations_path).unwrap_or(vec![]);
        let mut list_state = ListState::default();
        list_state.select_first();
        App {
            stations,
            player: Player::new(),
            current_station: None,
            mode: Mode::Normal,
            edit_field: EditField::Name,
            list_state,
        }
    }

    // Change the currently playing station
    pub fn change_station(&mut self) {
        self.selected_index()
            .and_then(|i| self.stations.get(i))
            .map(|station| self.player.play(station));
        self.current_station = self.selected_index();
    }

    // Stop playing
    pub fn stop(&mut self) {
        self.player.stop()
    }

    // Update the player's status from the underlying player
    pub fn update_status(&mut self) {
        self.player.update_status()
    }

    // Update a station with a new name
    pub fn update_station(&mut self, index: usize, name: String, url: String) {
        self.stations.get_mut(index).map(|station| {
            station.name = name;
            station.url = url;
        });
        self.save_stations()
    }

    // Add a new station at the end of the list
    pub fn add_station(&mut self, station: Station) {
        self.stations.push(station);
        self.save_stations()
    }

    // Save the current set of stations to the station file path
    fn save_stations(&self) {
        station_file_path().and_then(|file_path| {
            let mut writer = csv::Writer::from_path(file_path).ok()?;

            for station in self.stations.clone() {
                writer.serialize(station).ok()?;
            }

            writer.flush().ok()?;
            Some(())
        });
    }

    // Is the player currently playing?
    pub fn is_playing(&self) -> bool {
        self.player.state() == PlayerState::Playing
    }

    // Is the app in edit mode?
    pub fn is_edit_mode(&self) -> bool {
        self.mode == Mode::Edit
    }

    // Is the app in add mode?
    pub fn is_add_mode(&self) -> bool {
        self.mode == Mode::Add
    }

    // Select the previous station
    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    // Select the next station
    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    // Return the currently selected station's index
    pub fn selected_index(&self) -> std::option::Option<usize> {
        self.list_state.selected()
    }
}
