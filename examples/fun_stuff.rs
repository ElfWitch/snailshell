use snailshell::*;

fn main() {
    // you can optionally set the refresh rate of all subsequent print functions:
    set_snail_fps(60);

    clear_console();
    rest(0.5);

    // basic
    snailprint_d("🐌 Welcome, traveller. This is snailshell. :) 🫰\n", 1.5);
    snailprint_d("It is a tiny library for animating text output from \nterminal applications.", 2.0);
    snailprint("It is super simple to use!");
    dots();
    rest(1.0);

    // speed
    clear_console();
    snailprint("Custom Speed:");
    snailprint_d("🔥🔥🔥 VERY 🔥 FAST 🔥🔥🔥", 0.5);
    snailprint_d("🐌Slooow🐌", 4.0);
    dots();
    rest(1.0);

    // nostromo
    clear_console();
    snailprint("Works with mulitline strings!\n");
    snailprint_d("PRIORITY ONE\nINSURE RETURN OF ORGANISM\nFOR ANALYSIS.\nALL OTHER CONSIDERATIONS SECONDARY.\nCREW EXPENDABLE.\n", 7.0);
    dots();
    rest(1.0);

    // constant rate
    clear_console();
    snailprint("Constant Rate:\n");
    snailprint_s("- this message right here", 9.0);
    rest(0.5);
    snailprint_s("- will take as long per character as this one", 9.0);
    dots();
    rest(1.0);

    clear_console();
    snailprint_d("MUDKIP used WATER GUN! 🌊", 0.5);
    dots();
    rest(0.5);
    println!()
}


pub fn clear_console(){
    print!("\x1B[2J\x1B[1;1H");
}

/// print waiting dots
fn dots(){
    use std::io::{stdout, Write};

    for _ in 0..3{
        print!(".");
        rest(0.5);
        stdout().flush().unwrap();
    }
}

fn rest(secs: f32){
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs_f32(secs));
}