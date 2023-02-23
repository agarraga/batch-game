extern crate portaudio;

use std::env::Args;
use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS:     i32 = 2;
const SAMPLE_RATE:  f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 44_100;

pub fn main(mut args: Args) {

    // unwrap().parse().unwrap() not sure about that
    let seconds             = args.next().unwrap().parse::<i32>().unwrap();
    let left_phase_offset   = args.next().unwrap().parse::<i32>().unwrap();
    let right_phase_offset  = args.next().unwrap().parse::<i32>().unwrap();
    // let fundamental         = args[4].parse::<i32>().unwrap();

    match run(seconds, left_phase_offset, right_phase_offset) {
        Ok(_) => {}
        error => { eprintln!("oh noes! this happened: {:?}", error) }
    }
}

fn run(seconds: i32,
       left_phase_offset: i32,
       right_phase_offset: i32,
       // fundamental: i32
       ) -> Result<(), pa::Error> {

    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..TABLE_SIZE {
        sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
    }

    let mut left_phase  = 0;
    let mut right_phase = 0;

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    settings.flags = pa::stream_flags::CLIP_OFF;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut i = 0;
        for _ in 0..frames {
            buffer[i]       = sine[left_phase];
            buffer[i + 1]   = sine[right_phase];

            left_phase += left_phase_offset as usize;
            if left_phase >= TABLE_SIZE {
                left_phase -= TABLE_SIZE;
            }

            right_phase += right_phase_offset as usize;
            if right_phase >= TABLE_SIZE {
                right_phase -= TABLE_SIZE;
            }
            i += 2;
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    pa.sleep(seconds * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Check      box");

    Ok(())
}
