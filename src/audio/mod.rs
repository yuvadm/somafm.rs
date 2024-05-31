mod mpv;
mod rodio;

pub trait AudioBackend {
    fn play(&self, url: &str);
}

#[allow(dead_code)]
pub enum AudioBackends {
    Mpv(mpv::Mpv),
    Rodio(rodio::Rodio),
}

pub fn get_backend() -> AudioBackends {
    AudioBackends::Mpv(mpv::Mpv {})
}

impl AudioBackend for AudioBackends {
    fn play(&self, url: &str) {
        match self {
            AudioBackends::Mpv(m) => m.play(url),
            AudioBackends::Rodio(r) => r.play(url),
        }
    }
}
