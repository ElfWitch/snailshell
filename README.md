# 🐌 snailshell
A tiny library and [application ](#%EF%B8%8F-snailshell-standalone)for animating text in the terminal. 


![demo](demos/demo.gif)

☝️ From [Fun Stuff example](examples/fun_stuff.rs).
(obviously looks smoother in an actual terminal)

Easily add some flair to your text RPGs or console programs.

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
