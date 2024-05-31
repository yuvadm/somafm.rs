mod channels;

fn main() {
    let url = "https://api.somafm.com/channels.json";
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("{res}");
}
