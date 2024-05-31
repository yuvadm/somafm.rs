use crate::channels::Channel;
use audio::AudioBackend;
use inquire::{InquireError, Select};
use spinners::{Spinner, Spinners};

mod audio;
mod channels;

fn main() {
    let mut sp = Spinner::new(Spinners::Dots, "Loading SomaFM channels...".into());
    let channels = channels::get_channels();
    sp.stop_with_newline();

    let ans: Result<Channel, InquireError> =
        Select::new("Select channel from list:", channels).prompt();

    match ans {
        Ok(ch) => {
            let _sp = Spinner::new(Spinners::Arrow3, format!("Playing {}", ch.title));
            let url = format!("https://api.somafm.com/{}.pls", ch.id);
            audio::get_backend().play(&url);
        }
        Err(_e) => {
            println!("\nNo channel selected, exiting.");
        }
    }
}
