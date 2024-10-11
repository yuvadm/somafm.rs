use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Channel {
    pub id: String,
    pub title: String,
    pub description: String,
    #[allow(dead_code)]
    pub image: String,
}

impl Channel {
    pub fn get_playlist(&self) -> String {
        format!("https://api.somafm.com/{}.pls", self.id)
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.description)
    }
}

#[derive(Deserialize, Debug)]
pub struct Channels {
    pub channels: Vec<Channel>,
}

pub async fn get_channels() -> Vec<Channel> {
    let url = "https://api.somafm.com/channels.json";
    let res = reqwest::get(url).await.unwrap().text().await.unwrap();
    let channels: Channels = serde_json::from_str(&res).unwrap();
    channels.channels
}

pub async fn get_stream_url(pls: &str) -> Option<String> {
    let res = reqwest::get(pls).await.unwrap().text().await.unwrap();
    for line in res.lines() {
        let url = line.split_once("File1=");
        if url.is_some() {
            return Some(url.unwrap().1.to_string());
        }
    }
    None
}

mod tests {
    #[test]
    fn test_channel_parse() {
        let chan_str = r#"{"id":"7soul","title":"Seven Inch Soul","description":"Vintage soul tracks from the original 45 RPM vinyl.","dj":"Dion Watts Garcia","djmail":"dion@somafm.com","genre":"oldies","image":"https://api.somafm.com/img/7soul120.png","largeimage":"https://api.somafm.com/logos/256/7soul256.png","xlimage":"https://api.somafm.com/logos/512/7soul512.png","twitter":"","updated":"1396144686","playlists":[{"url":"https://api.somafm.com/7soul.pls","format":"mp3","quality":"highest"},{"url":"https://api.somafm.com/7soul130.pls","format":"aac","quality":"highest"},{"url":"https://api.somafm.com/7soul64.pls","format":"aacp","quality":"high"},{"url":"https://api.somafm.com/7soul32.pls","format":"aacp","quality":"low"}],"preroll":[],"listeners":"80","lastPlaying":"Durand Jones & The Indications - Is It Any Wonder"}"#;
        let chan: super::Channel = serde_json::from_str(chan_str).unwrap();
        assert_eq!(chan.id, "7soul");
    }
}
