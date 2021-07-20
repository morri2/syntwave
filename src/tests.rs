#[cfg(test)]
use super::*;
use super::wave::*;
use super::synth::*;

use rodio::{OutputStream, source::Source};

#[test]
fn main() {
    println!("Hello, music!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut synth = WaveSynth::new();

    synth.push_addative_wave(Wave::sine(440.0,0.015));
    synth.push_addative_wave(Wave::sine(660.0,0.015));
    synth.push_addative_wave(Wave::sine(550.0,0.015));
    synth.push_addative_wave(Wave::saw(4.0,0.045));
    synth.push_addative_wave(Wave::saw(1.0,0.090));
    synth.push_addative_wave(Wave::saw(1.0,0.045).offset(0.1));

    stream_handle.play_raw(synth);

    std::thread::sleep(std::time::Duration::from_secs(7));
}
