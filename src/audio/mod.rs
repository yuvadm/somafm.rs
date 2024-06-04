use tokio::sync::mpsc;

mod mpv;
mod rodio;

#[allow(dead_code)]
pub enum AudioBackends {
    Mpv(mpv::Mpv),
    Rodio(rodio::Rodio),
}

pub fn get_backend() -> AudioBackends {
    // AudioBackends::Mpv(mpv::Mpv {})
    AudioBackends::Rodio(rodio::Rodio {})
}

impl AudioBackends {
    pub async fn play(&self, url: &str, tx: mpsc::UnboundedSender<String>) {
        match self {
            AudioBackends::Mpv(m) => m.play(url, tx).await,
            AudioBackends::Rodio(r) => r.play(url, tx).await,
        }
    }
}
