use crate::channels::Channel;
use channels::get_stream_url;
use inquire::{InquireError, Select};
use spinners::{Spinner, Spinners};

mod audio;
mod channels;

#[tokio::main]
async fn main() {
    let mut sp = Spinner::new(Spinners::Dots, "Loading SomaFM channels...".into());
    let channels = channels::get_channels().await;
    sp.stop_with_newline();

    let ans: Result<Channel, InquireError> =
        Select::new("Select channel from list:", channels).prompt();

    match ans {
        Ok(ch) => {
            let mut sp = Spinner::new(Spinners::Dots, "Fetching channel streams...".into());
            let pls = format!("https://api.somafm.com/{}.pls", ch.id);
            let url = get_stream_url(&pls).await.unwrap();
            sp.stop_with_newline();

            let _sp = Spinner::new(Spinners::Arrow3, format!("Playing {}", ch.title));
            audio::get_backend().play(&url).await
        }
        Err(_e) => {
            println!("\nNo channel selected, exiting.");
        }
    }
}
