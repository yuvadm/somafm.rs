use std::num::NonZeroUsize;

use rodio::{Decoder, OutputStream, Sink};
use stream_download::{
    http::{reqwest::Client, HttpStream},
    source::SourceStream,
    storage::{
        bounded::BoundedStorageProvider, memory::MemoryStorageProvider, temp::TempStorageProvider,
    },
    Settings, StreamDownload,
};

pub struct Rodio {}

impl Rodio {
    pub async fn play(&self, url: &str) {
        let reader = StreamDownload::new_http(
            "https://ice6.somafm.com/groovesalad-256-mp3"
                .parse()
                .unwrap(),
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

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let stream = HttpStream::<Client>::create(url.parse().unwrap())
            .await
            .unwrap();

        println!("content type={:?}", stream.content_type());
        let bitrate: u64 = stream.header("Icy-Br").unwrap().parse().unwrap();
        println!("bitrate={bitrate}");

        // buffer 5 seconds of audio
        // bitrate (in kilobits) / bits per byte * bytes per kilobyte * 5 seconds
        let prefetch_bytes = bitrate / 8 * 1024 * 5;
        println!("prefetch bytes={prefetch_bytes}");

        let reader = StreamDownload::from_stream(
            stream,
            // use bounded storage to keep the underlying size from growing indefinitely
            BoundedStorageProvider::new(
                MemoryStorageProvider,
                // be liberal with the buffer size, you need to make sure it holds enough space to
                // prevent any out-of-bounds reads
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
            Settings::default().prefetch_bytes(prefetch_bytes),
        )
        .await
        .unwrap();
        sink.append(rodio::Decoder::new(reader).unwrap());

        let handle = tokio::task::spawn_blocking(move || {
            sink.sleep_until_end();
        });
        handle.await.unwrap()
    }
}
