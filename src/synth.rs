use crate::wave::Wave;
use core::time::Duration;
use rodio::Source;
use std::usize;

#[derive(Clone)]
pub struct CompoundWave {
    waves: Vec<Wave>,
    wave_ops: Vec<WaveOperation>,
    volume: f32,
    volume_cap: f32,
    sample_frequency: u32,
    sample_dt: f32,
    sample_head: u32,
}

impl CompoundWave {
    pub fn new() -> Self {
        let sample_frequency = 48000;
        Self {
            waves: Vec::with_capacity(5),
            wave_ops: Vec::with_capacity(5),

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

    pub fn wave_count(&self) -> usize {
        self.waves.len()
    }

    // volume functions (could be refractord)
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

    pub fn push_addative_wave(&mut self, wave: Wave) {
        self.waves.push(wave);
        self.wave_ops.push(WaveOperation::Addative);
    }

    pub fn push_subtractive_wave(&mut self, wave: Wave) {
        self.waves.push(wave);
        self.wave_ops.push(WaveOperation::Subtractive);
    }

    pub fn head_time(&self) -> f32 {
        self.sample_head as f32 * self.sample_dt
    }

    //wave edit
    pub fn replace_wave(&mut self, wave: Wave,wave_index: usize) {
        if wave_index >= self.wave_count() {panic!("wave_index oob!")}
        self.waves[wave_index] = wave;
    }
    pub fn frequency(&self, wave_index: usize) {
        if wave_index >= self.wave_count() {panic!("wave_index oob!")}
        self.waves[wave_index].frequency();
    }
    pub fn set_frequency(&mut self, frequency: f32, wave_index: usize) {
        if wave_index >= self.wave_count() {panic!("wave_index oob!")}
        self.waves[wave_index].set_frequency(frequency);
    }
    pub fn amplitude(&mut self, wave_index: usize) {
        if wave_index >= self.wave_count() {panic!("wave_index oob!")}
        self.waves[wave_index].amplitude();
    }
    pub fn set_amplitude(&mut self, amplitude: f32, wave_index: usize) {
        if wave_index >= self.wave_count() {panic!("wave_index oob!")}
        self.waves[wave_index].set_amplitude(amplitude);
    }
}

impl Iterator for CompoundWave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut synt_sample = 0.0;
        for i in 0..self.wave_count(){
            match self.wave_ops[i] {
                WaveOperation::Addative => synt_sample += self.waves[i].sample(self.head_time()),
                WaveOperation::Subtractive => synt_sample -= self.waves[i].sample(self.head_time()),
                _ => {},
            }
        }
        self.sample_head += 1;
        Some(self.amplify_sample(synt_sample))
    }
}

impl Source for CompoundWave {
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

#[derive(Copy,Clone)]
pub enum WaveOperation {
    Addative,
    Subtractive,
    Mute,
}

impl WaveOperation {

}
