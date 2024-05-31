use spinners::{Spinner, Spinners};

mod channels;

fn main() {
    let mut sp = Spinner::new(Spinners::Dots, "Loading SomaFM channels...".into());
    let channels = channels::get_channels();
    sp.stop();
    let groove_salad = channels.iter().find(|c| c.id == "groovesalad").unwrap();
    println!("\n{:?}", groove_salad);
}
