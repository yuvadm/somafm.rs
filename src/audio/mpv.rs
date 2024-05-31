use super::AudioBackend;
use std::process::Command;

pub struct Mpv {}

impl AudioBackend for Mpv {
    fn play(&self, url: &str) {
        Command::new("mpv")
            .args([url])
            .output()
            .expect("Failed to start mpv, make sure it is installed.");
    }
}
