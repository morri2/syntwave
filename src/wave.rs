// LOW FQ - Base    HIGH FQ - Tenor
// Sample freq sould be twice the max frequency, human limit is ca 20_000
use core::time::Duration;
use rodio::Source;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Wave {
    amplitude: f32,
    frequency: f32,
    waveform: Waveform,
}

impl Wave {
    pub fn new(frequency: f32, amplitude: f32, waveform: Waveform) -> Self {
        Self {
            amplitude,
            frequency,
            waveform,
        }
    }

    pub fn sine(frequency: f32, amplitude: f32) -> Self {
        Self::new(frequency, amplitude, Waveform::Sine)
    }

    pub fn square(frequency: f32, amplitude: f32) -> Self {
        Self::new(frequency, amplitude, Waveform::Square)
    }

    pub fn saw(frequency: f32, amplitude: f32) -> Self {
        Self::new(frequency, amplitude, Waveform::Saw)
    }

    pub fn waveform(&self) -> Waveform {
        self.waveform
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn amplitude(&self) -> f32 {
        self.amplitude
    }

    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude
    }

    pub fn frequency(self) -> f32 {
        self.frequency
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency
    }

    pub fn sample(&self, t: f32) -> f32 {
        //println!(" {}:{} ",t,self.amplitude * self.waveform.get_base_sound(t * self.frequency));
        self.amplitude * self.waveform.get_base_sound(t * self.frequency)
    }
}

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    None,
}

impl Waveform {
    fn get_base_sound(&self, t: f32) -> f32 {
        match self {
            Self::Sine => (t * 2.0 * PI).sin(),
            Self::Square => {
                if (t * 2.0 * PI).sin() >= 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Self::Saw => t % 1.0,
            _ => 0.0,
        }
    }
}
