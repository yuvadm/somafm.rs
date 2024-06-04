use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use rodio::{Decoder, OutputStream, Sink};
use std::num::NonZeroUsize;
use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::storage::bounded::BoundedStorageProvider;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};
use tokio::sync::mpsc;

const AUDIO_BUFFER_SECONDS: u32 = 5;

pub struct Rodio {}

impl Rodio {
    pub async fn play(&self, url: &str, tx: mpsc::UnboundedSender<String>) {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();

        // request metadata header
        let client = Client::builder().request_icy_metadata().build().unwrap();

        let stream = HttpStream::new(client, url.parse().unwrap()).await.unwrap();

        let icy_headers = IcyHeaders::parse_from_headers(stream.headers());
        // println!("Icecast headers: {icy_headers:#?}\n");
        // println!("content type={:?}\n", stream.content_type());

        // buffer bitrate (in kilobits) / bits per byte * bytes per kilobyte * N seconds
        let prefetch_bytes = icy_headers.bitrate().unwrap() / 8 * 1024 * AUDIO_BUFFER_SECONDS;

        let reader = StreamDownload::from_stream(
            stream,
            // use bounded storage to keep the underlying size from growing indefinitely
            BoundedStorageProvider::new(
                MemoryStorageProvider,
                // be liberal with the buffer size, you need to make sure it holds enough space to
                // prevent any out-of-bounds reads
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
            Settings::default().prefetch_bytes(prefetch_bytes as u64),
        )
        .await
        .unwrap();

        sink.append(
            Decoder::new(IcyMetadataReader::new(
                reader,
                // Since we requested icy metadata, the metadata interval header should be present in the
                // response. This will allow us to parse the metadata within the stream
                icy_headers.metadata_interval(),
                move |metadata| {
                    if let Ok(md) = metadata {
                        if let Some(tr) = md.stream_title() {
                            let _ = tx.send(tr.to_string());
                        }
                    }
                },
            ))
            .expect("Failed to play stream, check network and try again later"),
        );

        let handle = tokio::task::spawn_blocking(move || {
            sink.sleep_until_end();
        });
        handle.await.unwrap();
    }
}
