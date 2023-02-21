// TODO: get NUM_SECONDS, FRAMES_PER_BUFFER from args
extern crate portaudio;

use std::env;
use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS:     i32 = 2;
// const NUM_SECONDS:  i32 = 3;
const SAMPLE_RATE:  f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = env::args().collect();
    let seconds:    i32     = args[1].parse::<i32>().unwrap();
    let table_size: usize   = args[2].parse::<usize>().unwrap();
    let left_phase: i32     = args[3].parse::<i32>().unwrap();
    let right_phase: i32    = args[4].parse::<i32>().unwrap();
    match run(seconds, table_size, left_phase, right_phase) {
        Ok(_) => {}
        error => {
            eprintln!("oh noes! this happened: {:?}", error)
        }
    }
}

fn run(seconds: i32,
       table_size: usize,
       left_phase_offset: i32,
       right_phase_offset: i32
       ) -> Result<(), pa::Error> {
    println!( "DOING FIRST TODO ITEM");

    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..table_size {
        sine[i] = (i as f64 / table_size as f64 * PI * 2.0).sin() as f32;
    }

    let mut left_phase = 0;
    let mut right_phase = 0;

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    settings.flags = pa::stream_flags::CLIP_OFF;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _frame in 0..frames {
            buffer[idx] = sine[left_phase];
            buffer[idx + 1] = sine[right_phase];
            left_phase += left_phase_offset as usize;
            if left_phase >= table_size {
                left_phase -= table_size;
            }
            right_phase += right_phase_offset as usize;
            if right_phase >= table_size {
                right_phase -= table_size;
            }
            idx += 1;
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
