// LOW FQ - Base    HIGH FQ - Tenor
// Sample freq sould be twice the max frequency, human limit is ca 20_000
use std::f32::consts::PI as PI;
use rodio::Source;
use core::time::Duration;

pub struct Wave {
    amplitude: f32,
    frequency: f32,
    shape: WaveShape,
    sample_frequency: u32,
    t: u32,
}

impl Wave {

    pub fn sine(frequency: f32, amplitude: f32) -> Self{
        Self {
            amplitude,
            frequency,
            shape: WaveShape::Sine,
            sample_frequency: 48000,
            t: 0,
        }
    }

    pub fn square(frequency: f32, amplitude: f32) -> Self{
        Self {
            amplitude,
            frequency,
            shape: WaveShape::Square,
            sample_frequency: 48000,
            t: 0,
        }
    }

    pub fn saw(frequency: f32, amplitude: f32) -> Self{
        Self {
            amplitude,
            frequency,
            shape: WaveShape::Saw,
            sample_frequency: 48000,
            t: 0,
        }
    }
    
    pub fn offset(mut self, offset: f32) -> Self { // offset in sec
        self.t += (offset*(self.sample_frequency as f32)) as u32;
        return self;
    }

}

impl Iterator for Wave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = (self.t as f32 )/(self.sample_frequency as f32);
        self.t += 1;
        Some(self.amplitude * self.shape.get_base_sound(t * self.frequency))
    }
}

impl Source for Wave {
    fn current_frame_len(&self) -> Option<usize> {None}
    fn channels(&self) -> u16 {1}
    fn sample_rate(&self) -> u32 {self.sample_frequency}
    fn total_duration(&self) -> Option<Duration> {None}
}

pub enum WaveShape {
    Sine,
    Square,
    Saw,
    None,
}

impl WaveShape {
    fn get_base_sound(&self, t: f32) -> f32{
        match self {
            Self::Sine => {
                (t * 2.0 * PI).sin()
            }
            Self::Square => {
                if (t * 2.0 * PI).sin() >= 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Saw => {
                t % 1.0
            },
            Self::None => {
                0.0
            }
        }
    }
}
