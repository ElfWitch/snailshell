use clap::Parser;
use snailshell::{set_snail_fps, snailprint_d, snailprint_s};

#[derive(Parser)]
struct CLI{
    /// Text you want to animate
    text: String,

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

fn main() {

    let args = CLI::parse();

    // fps
    if let Some(fps) = args.fps{
        set_snail_fps(fps);
    }

    // custom duration
    if let Some(duration) = args.duration{
        snailprint_d(&args.text, duration);
    }

    // custom speed
    else if let Some(speed) = args.speed{
        snailprint_s(&args.text, speed);
    }

    // default
    else{
        snailprint_s(&args.text, 48.0);
    }

}
