use crate::app::Station;
use std::fmt;

use libmpv2::{events::*, *};

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Playing,
    Stopped,
    Buffering,
}

pub struct Player {
    state: PlayerState,
    mpv: Mpv,
    event_context: events::EventContext,
    pub current_title: Option<String>
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

    pub fn play(&mut self, station: &Station) {
        if let Ok(_) = self.mpv.command("loadfile", &[&station.url, "replace"]) {
            self.state = PlayerState::Buffering;
        }
    }

    pub fn stop(&mut self) {
        self.mpv.command("stop", &[]).unwrap();
        self.state = PlayerState::Stopped;
    }

    pub fn update_status(&mut self) {
        if let Some(Ok(event)) = self.event_context.wait_event(0f64) {
            match event {
                Event::StartFile => self.state = PlayerState::Playing,
                Event::PlaybackRestart => self.state = PlayerState::Playing,
                Event::Shutdown => self.state = PlayerState::Stopped,
                Event::EndFile(_) => self.state = PlayerState::Stopped,
                Event::PropertyChange {
                    name: "media-title",
                    change: PropertyData::Str(title),
                    ..
                } => {
                    self.current_title = Some(title.into())
                }
                _ => {}
            }
        }
    }

    pub fn state(&self) -> PlayerState {
        self.state
    }
}
