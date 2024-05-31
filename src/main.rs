use inquire::{InquireError, Select};
use spinners::{Spinner, Spinners};

use crate::channels::Channel;

mod channels;

fn main() {
    let mut sp = Spinner::new(Spinners::Dots, "Loading SomaFM channels...".into());
    let channels = channels::get_channels();
    sp.stop_with_newline();

    let ans: Result<Channel, InquireError> =
        Select::new("Select channel from list:", channels).prompt();

    match ans {
        Ok(ch) => {
            println!("{:?}", ch);
        }
        Err(_e) => {
            println!("\nNo channel selected, exiting.");
        }
    }
}
