#[macro_use]
extern crate vst;
// extern crate rand;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
// use rand::random;

#[derive(Default)]
struct TestPlugin {
    note: u8,
    velocity: u8
}

impl Plugin for TestPlugin {
  fn get_info(&self) -> Info {
      Info {
          name: "Test Plugin".to_string(),
          unique_id: 1337, // Used by hosts to differentiate between plugins.
          version: 0002,

          // For audio inputs
          inputs: 0,
          // Two outputs for stereo. Default
          outputs: 2,

          // Ableton still doesn't recognize correctly
          category: Category::Synth,

          ..Default::default()
      }
  }

  fn process_events(&mut self, events: &Events) {
    // Filters out SysEx events
    for event in events.events() {
        match event {
            Event::Midi(ev) => {
                // println!("{}", ev.data);
                match ev.data[0] {

                    // note on
                    144 => { 
                      self.note = ev.data[1];
                      self.velocity = ev.data[2]
                    },

                    // note off, currently doesn't seem to be working
                    128 => { 
                      self.note = 0;
                      self.velocity = 0
                    },
                    _ => (),
                }
            },
            // Do nothing for any other type of event (read: SysEx)
            _ => ()
        }
    }
  }

  fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    if self.note == 0 { return } // should be stopping audio when there's no note but not getting it now

    // `buffer.split()` gives us a tuple containing the 
    // input and output buffers.  We only care about the
    // output, so we can ignore the input by using `_`.
    let (_, mut output_buffer) = buffer.split();

    let mut x : f32 = 0.0;

    let two : f32 = 2.0;

    let freq : f32 = 440.0*two.powf((f32::from(self.note)-69.0)/12.0); // coming out 3 octaves too high
    let p : f32 = 1.0/freq;
    let pi = std::f32::consts::PI;

    for output_channel in output_buffer.into_iter() {
        for output_sample in output_channel {
            // supposed to be a sawwave
            *output_sample = -two*(f32::from(self.velocity)/pi) * (x*pi/p).cot().atan(); //excessive high end
            x = x + 1.0;
        }
    }
  }
}

// Cotangent is Cos/Sin
trait Cotangent {
    fn cot(&self) -> f32;
}

impl Cotangent for f32 {
    fn cot(&self) -> f32 {
      self.cos()/self.sin() as f32
    }
}



plugin_main!(TestPlugin); // Important!

