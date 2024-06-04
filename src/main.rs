use crate::channels::Channel;
use channels::get_stream_url;
use inquire::{InquireError, Select};
use spinners::{Spinner, Spinners};
use tokio::sync::mpsc;

mod audio;
mod channels;

#[tokio::main]
async fn main() {
    let mut sp = Spinner::new(Spinners::Dots, "Loading SomaFM channels...".into());
    let channels = channels::get_channels().await;
    sp.stop_with_newline();

    let ans: Result<Channel, InquireError> =
        Select::new("Select channel from list:", channels).prompt();

    let (tx, mut rx) = mpsc::unbounded_channel();

    match ans {
        Ok(ch) => {
            let mut sp = Spinner::new(Spinners::Dots, "Fetching channel streams...".into());
            let playlist = ch.get_playlist();
            let url = get_stream_url(&playlist).await.unwrap();
            sp.stop_with_newline();

            sp = Spinner::new(Spinners::Arrow3, format!("Playing {}", ch.title));
            tokio::spawn(async move {
                while let Some(title) = rx.recv().await {
                    sp.stop_with_newline();
                    sp = Spinner::new(Spinners::Arrow3, title);
                }
            });
            audio::get_backend().play(&url, tx).await
        }
        Err(_e) => {
            println!("\nNo channel selected, exiting.");
        }
    }
}
