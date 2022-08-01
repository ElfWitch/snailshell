//! # snailshell
//! Tiny library for making terminal text display with pleasant RPG-style animations.
//!
//! ## Examples
//! ```
//! # use snailshell::*;
//! // basic
//! snailprint("MUDKIP used WATER GUN!");
//!
//! // custom speeds
//! snailprint_d("This fully prints in exactly one second.", 1.0);
//! snailprint_s("This prints six characters per second", 6.0);
//! ```
//!
//! ## Colored Text
//! ```
//! use snailshell::*;
//!
//! // use any library you like.
//! // Snailshell works on any type that implements display.
//! // That means any type which you can use print!(), println!(), or format!() with!
//! use crossterm::style::Stylize;
//!
//! snailprint("flamingo, oh oh ou-oh".magenta());
//!
//! ```
//!
//! ### Refresh Rate
//! You can change the refresh rate with [set_snail_fps].
//! Call it once at the beginning of your program.
//! This is entirely optional.
//!
//! Default fps is 60.

use std::fmt::Display;
use std::io::{stdout, Write};
use std::sync::RwLock;
use std::thread::sleep;
use once_cell::sync::Lazy;


/// refresh rate of animated text
///
/// Text will only flush stdout buffer when characters should be updated.
/// Think of this as maximum FPS.
///
static FPS: Lazy<RwLock<u8>> = Lazy::new(||{
    RwLock::new(60)
});

/// Sets the global fps of animated text.
///
/// # Safety
/// This is only meant to be used once at the start of your program as internally it mutates a static variable.
pub fn set_snail_fps(fps: u8){
    if let Ok(mut f) = FPS.write(){
        *f = fps;
    }
}

/// Animate text with a fixed duration of two seconds.
///
/// ### Example
/// ```
/// # use snailshell::snailprint;
/// snailprint("The simplest way to use snailshell");
/// ```
pub fn snailprint<T: Display>(text: T){
    snailprint_d(text, 2.0);
}

/// Animate text at a constant speed of chars per second. This will animate so that each character
/// printed takes a predictable speed, unlike [snailprint_d](snailprint_d()).
/// ### Example
/// ```
/// # use snailshell::snailprint_s;
/// snailprint_s("this will print one character per second", 1.0);
/// snailprint_s("this will print 50 characters per second", 50.0);
/// ```
pub fn snailprint_s<T: Display>(text: T, speed: f32){
    let s = format!("{}", text);

    // duration is calculated from amount of ' ' whitespace in formatted text
    snailprint_d(text, s.split(' ').count() as f32 / speed);
}

/// Animate text with custom fixed duration. If you are printing a message with 10 characters and
/// a one with 200, they will both take the same amount of time if passed the same duration.
///### Example
/// ```
/// # use snailshell::snailprint_d;
/// snailprint_d("This message will take five seconds to print", 5.0);
/// snailprint_d("And so will this one", 5.0);
/// ```
///
///
pub fn snailprint_d<T: Display>(text: T, duration: f32){
    let s = format!("{}", text);

    let mut chars = s.chars().rev().collect::<Vec<char>>();

    let char_len = chars.len();

    let fps = match FPS.read() {
        Ok(f) => {
            *f as f32
        }
        Err(_) => {
            60.0
        }
    };

    let delta = 1.0 / fps;

    let mut percentage = 0.0;
    while !chars.is_empty(){
        let char_targ =  (char_len as f32 * percentage) as usize;

        while char_targ > char_len - chars.len() && !chars.is_empty(){
            print!("{}", chars.pop().unwrap());
            stdout().flush().unwrap();
        }
        sleep(std::time::Duration::from_secs_f32(delta));
        percentage += delta / duration;
    }
    println!()
}
