use super::AudioBackend;

pub struct Rodio {}

impl AudioBackend for Rodio {
    fn play(&self, _url: &str) {
        todo!()
    }
}
