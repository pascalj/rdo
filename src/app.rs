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
    Edit(usize),
    Add,
    Delete(usize),
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
    pub player_state: PlayerState,
}

// Load stations from the configuration
fn load_stations(path: PathBuf) -> std::io::Result<std::vec::Vec<Station>> {
    std::fs::read_to_string(path).map(|stations_str| {
        csv::Reader::from_reader(stations_str.as_bytes())
            .deserialize()
            .collect::<Result<Vec<Station>, csv::Error>>()
            .unwrap_or(vec![])
    })
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
            player_state: PlayerState::default(),
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
            player_state: PlayerState::default(),
        }
    }

    // Change the currently playing station
    pub fn change_station(&mut self, index: usize) {
        self.stations
            .get(index)
            .map(|station| self.player.play(station));
        self.current_station = self.selected_index();
    }

    // Stop playing
    pub fn stop(&mut self) {
        self.player.stop()
    }

    // Update the player's status from the underlying player
    pub fn update_status(&mut self) {
        if let Some(state) = self.player.update_status() {
            self.player_state = state;
        }
    }

    // Update a station with a new name
    pub fn update_station(
        &mut self,
        index: usize,
        name: String,
        url: String,
    ) -> std::io::Result<()> {
        self.stations.get_mut(index).map(|station| {
            station.name = name;
            station.url = url;
        });
        self.save_stations()
    }

    // Add a new station at the end of the list
    pub fn add_station(&mut self, station: Station) -> std::io::Result<()> {
        self.stations.push(station);
        self.save_stations()
    }

    // Delete a station persistently
    pub fn delete_station(&mut self, index: usize) -> std::io::Result<()> {
        self.stations.remove(index);
        self.save_stations()
    }

    // Save the current set of stations to the station file path
    fn save_stations(&self) -> std::io::Result<()> {
        let file_path =
            station_file_path().ok_or(std::io::Error::other("Could not find station path"))?;

        if let Some(missing_dir) = file_path.parent().filter(|dir| !dir.exists()) {
            std::fs::create_dir_all(missing_dir)?;
        }

        let mut writer = csv::Writer::from_path(file_path)?;
        for station in self.stations.clone() {
            writer.serialize(station)?;
        }

        writer.flush()?;
        Ok(())
    }

    // Is the player currently playing?
    pub fn is_playing(&self) -> bool {
        match self.player_state {
            PlayerState::Playing(_) => true,
            _ => false,
        }
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

    // Swap two stations
    pub fn swap_station(&mut self, index: usize, with: usize) -> std::io::Result<()> {
        if self.stations.get(index).is_some() && self.stations.get(with).is_some() {
            self.current_station = match self.current_station {
                Some(i) if i == index => Some(with),
                Some(i) if i == with => Some(index),
                _ => self.current_station,
            };
            self.stations.swap(index, with);
            self.save_stations()?;
        }
        Ok(())
    }
}
