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

impl<'a> From<&Station> for ListItem<'a> {
    fn from(val: &Station) -> Self {
        ListItem::new(val.name.clone())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Normal,
    Edit,
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

#[derive(Debug)]
pub struct App {
    pub stations: Vec<Station>,
    pub player: Player,
    pub current_station: Option<usize>,
    pub mode: Mode,
    pub edit_field: EditField,
    pub list_state: ListState,
    exit: bool,
}

fn config_file() -> std::option::Option<PathBuf> {
    config_local_dir().map(|config_dir| config_dir.join("rdo").join("stations.csv"))
}

fn load_stations() -> Option<std::vec::Vec<Station>> {
    config_file()
        .and_then(|file_path| std::fs::read_to_string(file_path).ok())
        .map(|stations_str| {
            csv::Reader::from_reader(stations_str.as_bytes())
                .deserialize()
                .collect::<Result<Vec<Station>, csv::Error>>()
                .unwrap_or(Vec::new())
        })
}

impl App {
    pub fn new() -> Self {
        let stations = load_stations().unwrap_or(vec![]);
        let mut list_state = ListState::default();
        list_state.select(if stations.is_empty() { None } else { Some(0) });
        App {
            stations,
            player: Player::new(),
            current_station: None,
            mode: Mode::Normal,
            edit_field: EditField::Name,
            list_state,
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn change_station(&mut self) {
        self.selected_index()
            .and_then(|i| self.stations.get(i))
            .map(|station| self.player.play(station));
    }

    pub fn stop(&mut self) {
        self.player.stop()
    }

    pub fn update_status(&mut self) {
        self.player.update_status()
    }

    pub fn update_station(&mut self, index: usize, name: String, url: String) {
        self.stations.get_mut(index).map(|station| {
            station.name = name;
            station.url = url;
        });
        self.save_stations()
    }

    fn save_stations(&self) {
        config_file().and_then(|file_path| {
            let mut writer = csv::Writer::from_path(file_path).ok()?;

            for station in self.stations.clone() {
                writer.serialize(station).ok()?;
            }

            writer.flush().ok()?;
            Some(())
        });
    }

    pub fn is_playing(&self) -> bool {
        self.player.state() == PlayerState::Playing
    }

    pub fn is_edit_mode(&self) -> bool {
        self.mode == Mode::Edit
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn selected_index(&self) -> std::option::Option<usize> {
        self.list_state.selected()
    }
}
