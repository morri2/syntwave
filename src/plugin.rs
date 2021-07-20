use crate::{synth::SynthWave, wave::Wave};
use rand::random;
use std::thread::current;
use vst::{
    api::{Events, Supported},
    event::Event,
    plugin::{CanDo, Category, Info, Plugin},
};
//#[derive(Default)]
struct SyntWave {
    current_note: Option<u8>,
    synth: SynthWave,
    sample_rate: f32,
}

impl Default for SyntWave {
    fn default() -> Self {
        Self {
            current_note: None,
            synth: {
                let mut synth = SynthWave::new();
                synth.push_addative_wave(Wave::sine(440.0, 0.1));
                synth
            },
            sample_rate: 44100.,
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
                        144 => self.current_note = Some(ev.data[1]),

                        // if note off, decrement our counter
                        128 => self.current_note = None,
                        _ => self.current_note = None,
                    }
                    // if we cared about the pitch of the note, it's stored in `ev.data[1]`.
                }
                // We don't care if we get any other type of event
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        // We only want to process *anything* if a note is
        // being held.  Else, we can return early and skip
        // processing anything!
        if let Some(pitch) = self.current_note {
            // `buffer.split()` gives us a tuple containing the
            // input and output buffers.  We only care about the
            // output, so we can ignore the input by using `_`.
            let (_, mut output_buffer) = buffer.split();

            // Now, we want to loop over our output channels.  This
            // includes our left and right channels (or more, if you
            // are working with surround sound).
            for output_channel in output_buffer.into_iter() {
                // Let's iterate over every sample in our channel.
                for output_sample in output_channel {
                    if let Some(sample) = self.synth.next() {
                        *output_sample = sample;
                    }
                }
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
}

plugin_main!(SyntWave);
