use super::mono::*;
use super::synth::*;
use super::wave::*;
#[cfg(test)]
use super::*;

use rodio::{source::Source, OutputStream};

#[test]
fn synthwave() {
    println!("Hello, synthwave!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut synth = SynthWave::new();

    synth.push_addative_wave(Wave::sine(440.0, 0.015));
    synth.push_addative_wave(Wave::sine(660.0, 0.015));
    synth.push_addative_wave(Wave::sine(550.0, 0.015));
    synth.push_addative_wave(Wave::saw(4.0, 0.045));
    synth.push_addative_wave(Wave::saw(1.0, 0.090));
    synth.push_addative_wave(Wave::saw(1.0, 0.045));

    stream_handle.play_raw(synth);

    std::thread::sleep(std::time::Duration::from_secs(7));
}

#[test]
fn monowave() {
    println!("Hello, monowave!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut mono = MonoWave::new(Wave::sine(440.0, 0.115));

    stream_handle.play_raw(mono);

    std::thread::sleep(std::time::Duration::from_secs(7));
}
