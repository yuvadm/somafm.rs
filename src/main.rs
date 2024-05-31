mod channels;

fn main() {
    let channels = channels::get_channels();
    let groove_salad = channels.iter().find(|c| c.id == "groovesalad").unwrap();
    println!("{:?}", groove_salad);
}
