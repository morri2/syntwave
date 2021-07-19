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
}

plugin_main!(SyntWave); 