use std::process::Command;
use tokio::sync::mpsc;

pub struct Mpv {}

impl Mpv {
    pub async fn play(&self, url: &str, _tx: mpsc::UnboundedSender<String>) {
        Command::new("mpv")
            .args([url])
            .output()
            .expect("Failed to start mpv, make sure it is installed.");
    }
}
