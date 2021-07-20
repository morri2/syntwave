use crate::wave::Wave;
use core::time::Duration;
use rodio::Source;
use std::f32::consts::PI;

pub struct MonoWave {
    wave: Wave,

    //volume settings
    volume: f32,
    volume_cap: f32,

    //sample/playback
    sample_frequency: u32,
    sample_dt: f32,
    sample_head: u32,
}

impl MonoWave {
    pub fn new(wave: Wave) -> Self {
        let sample_frequency = 48000;
        Self {
            wave,

            //volume settings
            volume: 1.0,
            volume_cap: 3.0,

            // sample settings
            sample_frequency,
            sample_dt: 1f32 / (sample_frequency as f32),
            sample_head: 0,
        }
    }

    pub fn with_sample_frequency(mut self, sample_frequency: u32) -> Self {
        self.sample_frequency = sample_frequency;
        self.sample_dt = 1f32 / (sample_frequency as f32);
        self
    }

    // Volume settings
    pub fn amplify_sample(&self, sample: f32) -> f32 {
        f32::min(
            f32::max(sample * self.volume, -self.volume_cap),
            self.volume_cap,
        )
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume
    }

    pub fn volume_cap(&self) -> f32 {
        self.volume
    }

    pub fn set_volume_cap(&mut self, volume_cap: f32) {
        self.volume_cap = volume_cap
    }

    pub fn head_time(&self) -> f32 {
        self.sample_head as f32 * self.sample_dt
    }
}

impl Source for MonoWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_frequency
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for MonoWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.wave.sample(self.head_time());
        self.sample_head += 1;
        Some(self.amplify_sample(sample))
    }
}
