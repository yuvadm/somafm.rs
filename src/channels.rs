use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Channel {
    id: String,
    title: String,
    description: String,
    image: String,
}

mod tests {
    #[test]
    fn test_channel_parse() {
        let chan_str = r#"{"id":"7soul","title":"Seven Inch Soul","description":"Vintage soul tracks from the original 45 RPM vinyl.","dj":"Dion Watts Garcia","djmail":"dion@somafm.com","genre":"oldies","image":"https://api.somafm.com/img/7soul120.png","largeimage":"https://api.somafm.com/logos/256/7soul256.png","xlimage":"https://api.somafm.com/logos/512/7soul512.png","twitter":"","updated":"1396144686","playlists":[{"url":"https://api.somafm.com/7soul.pls","format":"mp3","quality":"highest"},{"url":"https://api.somafm.com/7soul130.pls","format":"aac","quality":"highest"},{"url":"https://api.somafm.com/7soul64.pls","format":"aacp","quality":"high"},{"url":"https://api.somafm.com/7soul32.pls","format":"aacp","quality":"low"}],"preroll":[],"listeners":"80","lastPlaying":"Durand Jones & The Indications - Is It Any Wonder"}"#;
        let chan: super::Channel = serde_json::from_str(chan_str).unwrap();
        assert_eq!(chan.id, "7soul");
    }
}
