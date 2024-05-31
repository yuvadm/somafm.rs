use rodio::{Decoder, OutputStream, Sink};
use stream_download::{storage::temp::TempStorageProvider, Settings, StreamDownload};

pub struct Rodio {}

impl Rodio {
    pub async fn play(&self, url: &str) {
        let reader = StreamDownload::new_http(
            url.parse().unwrap(),
            TempStorageProvider::new(),
            Settings::default(),
        )
        .await
        .unwrap();

        let _res = tokio::task::spawn_blocking(move || {
            let (_stream, handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&handle).unwrap();
            sink.append(Decoder::new(reader).unwrap());
            sink.sleep_until_end();
        })
        .await;
    }
}
