extern crate portaudio;

use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::env::Args;
use portaudio as pa;

const CHANNELS:     i32 = 2;
const SAMPLE_RATE:  f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

pub fn main(mut args: Args) {
    let path: String = args.next().unwrap();
    let wave_file = File::open(path).unwrap();
    let mut reader = BufReader::new(wave_file);
    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer);

    play_wave(buffer);

}

fn print_bytes(buffer: Vec<u8>) {
    for i in 0..24 {
        println!("{:#04x}", buffer[i]);
    }
}

fn play_wave(wave: Vec<u8>) -> Result<(), pa::Error> {

    let pa = pa::PortAudio::new()?;

    let settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    // settings.flags = pa::stream_flags::CLIP_OFF;

    let mut wave_i = 40;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {

        let mut buff_i = 0;

        for _ in 0..frames {

            let mut wave_left = wave[wave_i + 1] as i16;
            wave_left <<= 8;
            wave_left |= wave[wave_i] as i16;

            let mut wave_right = wave[wave_i + 3] as i16;
            wave_right <<= 8;
            wave_right |= wave[wave_i + 2] as i16;


            buffer[buff_i]      = wave_left;
            buffer[buff_i + 1]  = wave_right;

            buff_i  += 2;
            wave_i  += 4;
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    pa.sleep(3_000);

    stream.stop()?;
    stream.close()?;

    println!("Check      box");

    Ok(())

}
