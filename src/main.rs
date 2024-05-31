use inquire::{InquireError, Select};
use spinners::{Spinner, Spinners};

use crate::channels::Channel;

mod channels;

fn main() {
    let mut sp = Spinner::new(Spinners::Dots, "Loading SomaFM channels...".into());
    let channels = channels::get_channels();
    sp.stop();

    let ans: Result<Channel, InquireError> = Select::new("Select a channel", channels).prompt();

    println!("\n{:?}", ans);
}
