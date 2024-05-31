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
    pub async fn play(&self, url: &str) {
        match self {
            AudioBackends::Mpv(m) => m.play(url).await,
            AudioBackends::Rodio(r) => r.play(url).await,
        }
    }
}
