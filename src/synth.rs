use crate::wave::Wave;

pub struct WaveSynth {
    waves: Vec<Wave>,
    volume: f32,
}

impl WaveSynth {

    pub fn new() -> Self {
        Self {
            waves: Vec::with_capacity(3),
            volume: 1.0,
        }
    }

    pub fn waves(&self) -> usize {
        self.waves.len()
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    fn get_sound(&self, t: f32) -> f32 {
        
    }

}


pub enum WaveElement {
    Addative(Wave),
    Subtractive(Wave),
}