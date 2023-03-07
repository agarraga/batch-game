use std::env;
use std::env::Args;

mod sine;
use sine::StereoPhase;

fn main() {
    let phase = StereoPhase {left: 440, right: 440};

    match sine::run_stereo_sine_stream(phase) {
        Ok(_) => {},
        error => {
            eprintln!("oh noes! This happened: {:?}", error);
        }
    }
}

