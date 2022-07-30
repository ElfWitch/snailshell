# 🐌 snailshell
[![MIT LICENSE](https://img.shields.io/crates/l/snailshell)](https://github.com/ElfWitch/snailshell/blob/main/LICENSE)
[![VERSION](https://img.shields.io/crates/v/snailshell)](https://crates.io/crates/snailshell)
[![DOCS](https://img.shields.io/docsrs/snailshell)](https://docs.rs/snailshell/latest/snailshell/)
[![DOWNLOADS](https://img.shields.io/crates/d/snailshell)](https://crates.io/crates/snailshell)

A tiny library and [application ](#%EF%B8%8F-snailshell-standalone)for animating text in the terminal.

🪄 It's so easy add some flair to your text RPGs or console programs. 🪄

![demo](demos/fun_stuff.gif)

☝️ From [Fun Stuff example](examples/fun_stuff.rs).
(obviously looks smoother in an actual terminal)

### Compatibility
Compatible with EVERYTHING! 🥳

Snailshell works with any type which implements `Display`, so literally everything that you would normally use with `print!()`, `println!()`, or `format!()` just works!
This includes colored text from other libraries such as [Crossterm](https://github.com/crossterm-rs/crossterm).


### 👩‍🏫 Examples
___
#### Basic
```rust
use snailshell::*;

// It's dead simple.
snailprint("hello, friend :)");
```

#### Custom Fixed Duration
```rust
snailprint_d("This whole message will print in half a second regardless of the size.", 0.5);
```

#### Refresh Rate
```rust
// This is optional. Default fps is 60.
// All subsequent snailprint functions will use this fps.
set_snail_fps(30);
```

## 🐌🖥️ Snailshell Standalone:
You can also use snailshell from the command line.

![cli demo](demos/cli_demo.gif)

Useful for making shell scripts and videos!
### Installation:
`cargo install snailshell`

### Usage:
___
```USAGE:
snailshell [OPTIONS] <TEXT>

ARGS:
<TEXT>    Text you want to animate

OPTIONS:
-d, --duration <DURATION>    How long the text should animate for
-f, --fps <FPS>              Refresh rate of animation
-h, --help                   Print help information
-s, --speed <SPEED>          Constant speed of chars per second to render. Conflicts with duration
```

### Example
`snailshell "text you want to animate"`

### 📖 License
___
This crate is licensed under the [MIT license](LICENSE). 
