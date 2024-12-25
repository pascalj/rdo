use crate::app::Station;

use libmpv2::{events::*, *};

#[derive(PartialEq, Clone)]
pub enum PlayerState {
    Playing(String),
    Stopped,
    Buffering,
}

impl Default for PlayerState {
    fn default() -> PlayerState {
        PlayerState::Stopped
    }
}

// A radio player
pub struct Player {
    state: PlayerState,
    mpv: Mpv,
    event_context: events::EventContext,
    pub current_title: Option<String>,
}

impl Player {
    // Create a new player
    pub fn new() -> Player {
        let mpv = Mpv::with_initializer(|_| Ok(())).unwrap();
        let event_context = EventContext::new(mpv.ctx);
        if let Err(_) = event_context.observe_property("media-title", Format::String, 0) {
            eprintln!("Error getting media title");
        }
        Player {
            state: PlayerState::Stopped,
            mpv,
            event_context,
            current_title: None,
        }
    }

    // Start playing a specific station
    pub fn play(&mut self, station: &Station) {
        if let Ok(_) = self.mpv.command("loadfile", &[&station.url, "replace"]) {
            self.state = PlayerState::Buffering;
        }
    }

    // Stop playing
    pub fn stop(&mut self) {
        self.mpv.command("stop", &[]).unwrap();
        self.state = PlayerState::Stopped;
    }

    // Update the status from the underlying MPV instance
    pub fn update_status(&mut self) -> Option<PlayerState> {
        if let Some(Ok(event)) = self.event_context.wait_event(0f64) {
            let title_or_default = self.current_title.clone().unwrap_or("".to_owned());
            match event {
                Event::StartFile => Some(PlayerState::Playing(title_or_default)),
                Event::PlaybackRestart => Some(PlayerState::Playing(title_or_default)),
                Event::Shutdown => Some(PlayerState::Stopped),
                Event::EndFile(_) => Some(PlayerState::Stopped),
                Event::PropertyChange {
                    name: "media-title",
                    change: PropertyData::Str(title),
                    ..
                } => {
                    self.current_title = Some(title.into());
                    None
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
