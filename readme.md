
# GW
gw is a [gdub](https://github.com/dougborg/gdub) clone written in Rust for fun.
Looks for a gradle wrapper (gradlew or gradlew.bat) in your CWD and its parent directories and executes it with the given arguments.

## Installation

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``cargo install --git https://github.com/normartin/rust-gradlew-locator``
3. Use ``gw`` in one of your gradle projects

or use a released binary:

1. Download a [release](https://github.com/normartin/rust-gradlew-locator/releases) binary for your platform
2. Rename it to ``gw`` or on windows ``gw.exe``
3. Put it on your $PATH
4. Enjoy ``gw`` in your gradle projects
