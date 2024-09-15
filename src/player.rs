use crate::app::Station;
use std::fmt;

use libmpv2::{events::*, mpv_node::MpvNode, *};

#[derive(Debug)]
pub enum PlayerState {
    Playing,
    Stopped,
    Buffering,
}

pub struct Player {
    state: PlayerState,
    mpv: Mpv,
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Player")
            .field("state", &self.state)
            .finish()
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            state: PlayerState::Stopped,
            mpv: Mpv::with_initializer(|_| { Ok(()) }).unwrap(),
        }
    }

    pub fn play(&mut self, station: &Station) {
        self.mpv.command("loadfile", &[&station.url, "replace"]).unwrap();
    }

    pub fn stop(&mut self) {
        self.mpv.command("stop", &[]).unwrap();
    }
}
