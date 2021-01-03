#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};

#[derive(Default)]
struct TestPlugin;

impl Plugin for TestPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Test Plugin".to_string(),
            unique_id: 1337, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
}

plugin_main!(TestPlugin); // Important!

