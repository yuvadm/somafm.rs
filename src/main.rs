use crate::channels::Channel;
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
            let _sp = Spinner::new(Spinners::Arrow3, format!("Playing {}", ch.title));
            let url = format!("https://api.somafm.com/{}.pls", ch.id);
            audio::get_backend().play(&url).await
        }
        Err(_e) => {
            println!("\nNo channel selected, exiting.");
        }
    }
}
