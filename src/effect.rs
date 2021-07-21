use rand::random;

/// A simple ADSR envelope, see https://en.wikipedia.org/wiki/Envelope_(music)
pub struct ADSR {
    attack: f32,           // time to reach full amplitude, from the moment of the key press
    attack_amplitude: f32, // the amplitude to reach, as a fraction of the signal, should be 0.0 - 1.0
    decay: f32,            // time to reach the sustain amplitude
    sustain: f32, // amplitude to sustain the signal at when key is held, as a fraction of the signal, should be 0.0 - 1.0
    release: f32, // time to reach amplitude 0.0, from the moment of key release
    time: f32,    // time since key-press/key-release
    key_on: bool, // is the key pressed?
}

impl Default for ADSR {
    fn default() -> Self {
        Self {
            attack: 0.01,
            attack_amplitude: 1.,
            decay: 0.0,
            sustain: 1.,
            release: 0.01,
            time: 0.,
            key_on: false,
        }
    }
}

impl ADSR {
    pub fn new(attack: f32, attack_amplitude: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            attack_amplitude,
            decay,
            sustain,
            release,
            time: 0.,
            key_on: false,
        }
    }

    pub fn envelope_signal(&self, signal: f32) -> f32 {
        // coefficient to scale the signal by
        // should be between 0.0 - 1.0
        let mut alpha: f32;
        alpha = if self.key_on {
            if self.time < self.attack {
                // Attack
                (self.time * self.attack_amplitude) / self.attack
            } else if self.time < (self.decay + self.attack) {
                // Decay
                (self.sustain - self.attack_amplitude) * (self.time - self.decay) / self.decay + 1.0
            } else {
                // Sustain
                self.sustain
            }
        } else if self.time < self.release {
            // Release
            // Assume time = time since release of key
            self.sustain - (self.sustain / self.release) * self.time
        } else {
            // No sound should be played
            0.0f32
        };
        return signal * alpha;
    }

    pub fn increment_time(&mut self, time: f32) {
        self.time += time;
    }

    pub fn key_press(&mut self) {
        self.time = 0.0;
        self.key_on = true;
    }

    pub fn key_release(&mut self) {
        self.time = 0.0;
        self.key_on = false;
    }

    // pub fn set_key_on(&mut self, key_on: bool) {
    //     self.key_on = key_on;
    // }
}

#[test]
fn envelope_signal_less_than_or_equal_one() {
    let mut adsr = ADSR::default();

    let samples: u32 = 41400;
    let time_per_sample = 1.0 / (samples as f32);
    adsr.key_press();
    //let mut string_buf = String::with_capacity(1000);
    for sample in 0..samples {
        let signal = (random::<f32>() - 0.5f32) * 2f32;
        let processed = adsr.envelope_signal(signal);
        //string_buf.push_str(&signal.to_string());
        //string_buf.push('\n');
        if processed.abs() > 1.0 {
            panic!()
        }
        adsr.increment_time(time_per_sample);
    }
    adsr.key_release();
    for sample in 0..samples {
        let signal = (random::<f32>() - 0.5f32) * 2f32;
        let processed = adsr.envelope_signal(signal);
        //string_buf.push_str(&signal.to_string());
        //string_buf.push('\n');
        if processed.abs() > 1.0 {
            println!(
                "Signal {} had a value of {}, which became {} after processing",
                sample, signal, processed
            );
            panic!()
        }
        adsr.increment_time(time_per_sample);
    }
    //print!("{}", string_buf);
}

#[test]
fn release_smaller_than_sustain() {
    let mut adsr = ADSR::default();
    let samples: u32 = 41400;
    let time_per_sample = 1.0 / (samples as f32);
    let signal_1 = (random::<f32>() - 0.5f32) * 2f32;
    let processed_1 = adsr.envelope_signal(signal_1);
    adsr.increment_time(time_per_sample);
    adsr.increment_time(time_per_sample);
    adsr.increment_time(time_per_sample);
    println!("{}", time_per_sample);
    let processed_2 = adsr.envelope_signal(signal_1);
    if processed_2.abs() > processed_1.abs() {
        panic!()
    }
}
