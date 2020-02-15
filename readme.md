
# GW
[gdub](https://github.com/dougborg/gdub) clone written in Rust for fun.
Looks for a gradle wrapper (gradlew or gradlew.bat) in your CWD and its parent directories and executes it with the given arguments.

## Installation

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``cargo install --path .``
3. Use ``gw`` anywhere

## Notes
Use 

    strip target/release/gw
    
to decrease binary size ([see](https://github.com/johnthagen/min-sized-rust)).