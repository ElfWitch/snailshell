use std::io::{Read, stdin};
use clap::Parser;
use snailshell::{set_snail_fps, snailprint_d, snailprint_s};

#[derive(Parser)]
struct Cli{
    /// Text you want to animate
    text: Option<String>,

    /// How long the text should animate for
    #[clap(short, long)]
    duration: Option<f32>,

    /// Constant speed of chars per second to render. Conflicts with duration
    #[clap(short, long, conflicts_with("duration"))]
    speed: Option<f32>,

    /// Refresh rate of animation
    #[clap(short, long)]
    fps: Option<u8>
}

fn main(){

    let args = Cli::parse();

    let text = {
        if let Some(text) = args.text{
            text
        }else{
            let mut s = String::new();

            if stdin().lock().read_to_string(&mut s).is_err(){
                println!("Input is not valid UTF-8. You may have passed contents of a binary file.")
            }

            s
        }
    };

    // fps
    if let Some(fps) = args.fps{
        set_snail_fps(fps);
    }

    // custom duration
    if let Some(duration) = args.duration{
        snailprint_d(&text, duration);
    }

    // custom speed
    else if let Some(speed) = args.speed{
        snailprint_s(&text, speed);
    }

    // default
    else{
        snailprint_s(&text, 48.0);
    }

}
