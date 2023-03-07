extern crate portaudio;

use portaudio as pa;
use std::f64::consts::PI;


const CHANNELS: i32     = 2;
const SAMPLE_RATE: f64  = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 512;
const TABLE_SIZE: usize = SAMPLE_RATE as usize;

fn get_sine() -> [f32; TABLE_SIZE] {
    let mut sine = [0.0; TABLE_SIZE];
    let mut i = 0;
    while i < TABLE_SIZE {
        sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
        i += 1;
    }
    sine
}

pub struct StereoPhase {
    pub left: usize,
    pub right: usize,
}

// pub fn create_stream() -> portaudio::Stream<portaudio::NonBlocking, portaudio::Output<>> {}

pub fn run_stereo_sine_stream(phase_offset: StereoPhase) -> Result<(), pa::Error> {

    let sine    = get_sine();
    let pa      = pa::PortAudio::new()?;
    let mut phase = StereoPhase {left: 0, right: 0};

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    settings.flags = pa::stream_flags::CLIP_OFF;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut i = 0;
        for _ in 0..frames {
            buffer[i]       = sine[phase.left];
            buffer[i + 1]   = sine[phase.right];

            phase.left += phase_offset.left;
            if phase.left >= TABLE_SIZE {
                phase.left -= TABLE_SIZE;
            }

            phase.right += phase_offset.right;
            if phase.right >= TABLE_SIZE {
                phase.right -= TABLE_SIZE;
            }

            i += 2;
        }

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;
    stream.stop()?;
    stream.close()?;

    Ok(())
}
