// LOW FQ - Base    HIGH FQ - Tenor
// Sample freq sould be twice the max frequency, human limit is ca 20_000
use std::f32::consts::PI as PI;
use rodio::Source;
use core::time::Duration;

pub struct Wave {
    amplitude: f32,
    frequenzy: f32,
    shape: WaveShape,
    sample_frequenzy: u32,
    timestamp: u32,
}

impl Wave {

    pub fn sine(frequenzy: f32, amplitude: f32) -> Self{
        Self {
            amplitude,
            frequenzy,
            shape: WaveShape::Sine,
            sample_frequenzy: 48000,
            timestamp: 0,
        }
    }

    pub fn square(frequenzy: f32, amplitude: f32) -> Self{
        Self {
            amplitude,
            frequenzy,
            shape: WaveShape::Square,
            sample_frequenzy: 48000,
            timestamp: 0,
        }
    }

    pub fn saw(frequenzy: f32, amplitude: f32) -> Self{
        Self {
            amplitude,
            frequenzy,
            shape: WaveShape::Saw,
            sample_frequenzy: 48000,
            timestamp: 0,
        }
    }

}


impl Iterator for Wave {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let t = (self.timestamp as f32 )/(self.sample_frequenzy as f32);
        self.timestamp += 1;
        Some(self.amplitude * self.shape.get_base_sound(t * self.frequenzy) + self.amplitude)
    }

}

impl Source for Wave {
    fn current_frame_len(&self) -> Option<usize> {None}
    fn channels(&self) -> u16 {1}
    fn sample_rate(&self) -> u32 {self.sample_frequenzy}
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
