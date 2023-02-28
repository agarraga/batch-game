use std::env;
use std::env::Args;

mod sine;
mod wave;
mod pixel;

fn main() {
    let mut args: Args = env::args();
    unsafe {
        match args.nth(1).unwrap().as_str() {
            "sine" => sine::main(args),
            "wave" => wave::main(args),
            "pixel" => pixel::main(),
            _ => ()
        }
    }
}

