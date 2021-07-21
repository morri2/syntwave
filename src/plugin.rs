use crate::{effect::ADSR, mono::MonoWave, synth::CompoundWave, wave::Wave};
use rand::random;
use std::thread::current;
use vst::{
    api::{Events, Supported},
    event::Event,
    plugin::{CanDo, Category, Info, Plugin},
};

/// Convert the midi note's pitch into the equivalent frequency.
///
/// This function assumes A4 is 440hz.
fn midi_pitch_to_freq(pitch: u8) -> f32 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f32 = 440.0;

    // Midi notes can be 0-127
    ((f32::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

struct SyntWave {
    current_note: Option<u8>,
    note_on: bool,
    synth: SynthType,
    sample_rate: f32,
    envelope: ADSR,
}

impl SyntWave {
    fn note_on(&mut self, note: u8) {
        self.note_on = true;
        self.current_note = Some(note);
        self.envelope.key_press();
    }

    fn note_off(&mut self) {
        self.note_on = false;
        self.envelope.key_release();
    }
}

impl Default for SyntWave {
    fn default() -> Self {
        Self {
            current_note: None,
            note_on: false,
            synth: {
                let synth = MonoWave::new(Wave::sine(440.0, 0.1)).with_sample_frequency(44100);
                SynthType::Mono(synth)
            },
            sample_rate: 44100.,
            envelope: ADSR::default(),
        }
    }
}
impl Plugin for SyntWave {
    fn get_info(&self) -> Info {
        Info {
            name: "SyntWave".to_string(),

            // Used by hosts to differentiate between plugins.
            unique_id: 4445,

            // We don't need inputs
            inputs: 0,

            // We do need two outputs though.  This is default, but let's be
            // explicit anyways.
            outputs: 2,

            // Set our category
            category: Category::Synth,

            // We don't care about other stuff, and it can stay default.
            ..Default::default()
        }
    }
    // Here's the function that allows us to receive events
    fn process_events(&mut self, events: &Events) {
        // Some events aren't MIDI events - so let's do a match
        // to make sure we only get MIDI, since that's all we care about.
        for event in events.events() {
            match event {
                Event::Midi(ev) => {
                    // Check if it's a noteon or noteoff event.
                    // This is difficult to explain without knowing how the MIDI standard works.
                    // Basically, the first byte of data tells us if this signal is a note on event
                    // or a note off event.  You can read more about that here:
                    // https://www.midi.org/specifications/item/table-1-summary-of-midi-message
                    match ev.data[0] {
                        // if note on, increment our counter
                        144 => self.note_on(ev.data[1]),

                        // if note off, decrement our counter
                        128 => self.note_off(),
                        _ => self.note_off(),
                    }
                    // if we cared about the pitch of the note, it's stored in `ev.data[1]`.
                }
                // We don't care if we get any other type of event
                _ => (),
            }
        }
    }
    /// Where the audio is proccesed, i.e the "make sound" function
    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();
        let mut output_sample = 0.0; // Default of 0.0 (no sound)

        for sample_index in 0..samples {
            if let Some(pitch) = self.current_note {
                self.synth.set_frequency(midi_pitch_to_freq(pitch));
                if let Some(sample) = self.synth.next_sample() {
                    // Apply envelope
                    output_sample = self.envelope.envelope_signal(sample);
                    self.envelope.increment_time(1.0 / self.sample_rate);
                }
            } else {
                // No note played
                output_sample = 0.0;
            }

            for buffer_index in 0..output_count {
                let buff = outputs.get_mut(buffer_index);
                buff[sample_index] = output_sample;
            }
        }
    }
    // Tells the host what the plugin supports
    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
        self.synth = self.synth.set_sample_rate(rate);
    }
}

enum SynthType {
    Mono(MonoWave),
    Multi(CompoundWave),
}
impl SynthType {
    pub fn next_sample(&mut self) -> Option<f32> {
        match self {
            Self::Mono(mono) => mono.next(),
            Self::Multi(multi) => multi.next(),
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        match self {
            Self::Mono(mono) => mono.set_frequency(frequency),
            Self::Multi(multi) => todo!(),
        }
    }

    pub fn set_sample_rate(&mut self, rate: f32) -> SynthType {
        match self {
            Self::Mono(mono) => Self::Mono(mono.clone().with_sample_frequency(rate as u32)),
            Self::Multi(multi) => todo!(),
        }
    }
}

plugin_main!(SyntWave);
