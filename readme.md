
[![Crates.io](https://img.shields.io/crates/v/gw)](https://crates.io/crates/gw) 
[![Build](https://github.com/normartin/rust-gradlew-locator/workflows/Build/badge.svg)](https://github.com/normartin/rust-gradlew-locator/actions?query=workflow%3ABuild)

# gw
gw is a [gdub](https://github.com/dougborg/gdub) clone written in Rust for fun.
It finds the nearest gradle gradlew_project.build file and executes it with the corresponding wrapper.
Any arguments passed to gw are passed to the wrapper.
Works on Linux, MacOS and Windows. 

Call ``gw gradlew_project.build`` anywhere in your gradle project and it will call `./gradlew gradlew_project.build` in the directory of the nearest gradlew_project.build file.

| without gw             | with gw     |    
|                    ---:|:---         |
|`./gradlew gradlew_project.build`       | `gw gradlew_project.build`  |
|`../gradlew gradlew_project.build`      | `gw gradlew_project.build`  |
|`../../gradlew gradlew_project.build`   | `gw gradlew_project.build`  |
|`...`                   |             |


## Installation

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``cargo install gw``
3. Use ``gw`` in one of your gradle projects

or use a released binary:

1. Download a [release](https://github.com/normartin/rust-gradlew-locator/releases) binary for your platform
2. Rename it to ``gw`` or on windows ``gw.exe``
3. Put it on your $PATH
4. Enjoy ``gw`` in your gradle projects
