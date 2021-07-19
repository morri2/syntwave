#[cfg(test)]
use super::*;
use super::wave::*;
use super::synth::*;

use rodio::{OutputStream, source::Source};

#[test]
fn main() {
    println!("Hello, music!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let w1 = Wave::square(440.0,0.025);
    let w2 = Wave::saw(80.0,0.025);
    let w3 = Wave::sine(180.0,0.025);

    let mut synth = WaveSynth::new();
    synth.push_addative_wave(w1);
    synth.push_addative_wave(w2);
    synth.push_subtractive_wave(w3);

    stream_handle.play_raw(synth);

    std::thread::sleep(std::time::Duration::from_secs(3));
}
