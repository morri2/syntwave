#[cfg(test)]
use super::*;

use rodio::{OutputStream, source::Source};

#[test]
fn main() {
    println!("Hello, music!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let source = Wave::square(440.0,0.025);

    stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(5));
}