// TODO: get NUM_SECONDS, FRAMES_PER_BUFFER from args
extern crate portaudio;

use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS:     i32 = 2;
const NUM_SECONDS:  i32 = 6;
const SAMPLE_RATE:  f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;

fn main() {
    match run() {
        Ok(_) => {}
        error => {
            eprintln!("oh noes! this happened: {:?}", error)
        }
    }
}

fn run() -> Result<(), pa::Error> {
    println!( "DOING FIRST TODO ITEM");

    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..TABLE_SIZE {
        sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
    }

    let mut left_phase = 0;
    let mut right_phase = 0;

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    settings.flags = pa::stream_flags::CLIP_OFF;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            buffer[idx] = sine[left_phase];
            buffer[idx + 1] = sine[right_phase];
            left_phase += 1;
            if left_phase >= TABLE_SIZE {
                left_phase -= TABLE_SIZE;
            }
            right_phase += 3;
            if right_phase >= TABLE_SIZE {
                right_phase -= TABLE_SIZE;
            }
            idx += 2;
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    pa.sleep(NUM_SECONDS * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Check      box");

    Ok(())
}
