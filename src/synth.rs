use crate::wave::Wave;
use rodio::Source;
use core::time::Duration;

pub struct WaveSynth {
    waves: Vec<WaveElement>,
    volume: f32,
    sample_frequency: u32,
}

impl WaveSynth {

    pub fn new() -> Self {
        Self {
            waves: Vec::with_capacity(3),
            volume: 1.0,
            sample_frequency: 48000,
        }
    }

    pub fn waves(&self) -> usize {
        self.waves.len()
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn push_addative_wave(&mut self, wave: Wave) {
        self.waves.push(WaveElement::Addative(wave));
    }

    pub fn push_subtractive_wave(&mut self, wave: Wave) {
        self.waves.push(WaveElement::Subtractive(wave));
    }
}

impl Iterator for WaveSynth {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut synt = 0.0;
        for wave_elem in &self.waves {
            match wave_elem {
                WaveElement::Addative(wave) => synt += wave.next()?,
                WaveElement::Subtractive(wave) => synt += wave.next()?,
            }
        }
        Some(synt * self.volume)
    }
}

impl Source for WaveSynth {
    fn current_frame_len(&self) -> Option<usize> {None}
    fn channels(&self) -> u16 {1}
    fn sample_rate(&self) -> u32 {self.sample_frequency}
    fn total_duration(&self) -> Option<Duration> {None}
}

pub enum WaveElement {
    Addative(Wave),
    Subtractive(Wave),
}