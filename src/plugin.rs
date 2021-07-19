use vst::{plugin::{Info, Plugin, Category}};

#[derive(Default)]
struct SyntWave;

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

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        // `buffer.split()` gives us a tuple containing the 
        // input and output buffers.  We only care about the
        // output, so we can ignore the input by using `_`.
        let (_, output_buffer) = buffer.split();
        
        // Now, we want to loop over our output channels.  This
        // includes our left and right channels (or more, if you
        // are working with surround sound).
        for output_channel in output_buffer.into_iter() {
            // Let's iterate over every sample in our channel.
            for output_sample in output_channel {
                // For every sample, we want to add a random value from
                // -1.0 to 1.0.
                *output_sample = 0f32;
            }
        }
    }
}

plugin_main!(SyntWave); 