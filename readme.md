
# GW
gw is a [gdub](https://github.com/dougborg/gdub) clone written in Rust for fun.
Looks for a gradle wrapper (gradlew or gradlew.bat) in your CWD and its parent directories and executes it with the given arguments.

## Installation

1. Download [release](https://github.com/normartin/rust-gradlew-locator/releases) binary for your platform
2. Rename it to ``gw`` or on windows ``gw.exe``
3. Put it on your $PATH
4. enjoy ``gw`` in your gradle projects

or build and install from sources:

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``git clone https://github.com/normartin/rust-gradlew-locator``
3. ``cargo install --path rust-gradlew-locator``
4. Use ``gw`` anywhere
