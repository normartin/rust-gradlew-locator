
[![Crates.io](https://img.shields.io/crates/v/gw)](https://crates.io/crates/gw) 
[![Build](https://github.com/normartin/rust-gradlew-locator/workflows/Build/badge.svg)](https://github.com/normartin/rust-gradlew-locator/actions?query=workflow%3ABuild)

# gw
gw is a [gdub](https://github.com/dougborg/gdub) clone written in Rust for fun.
It finds the nearest gradle build file and executes it with the corresponding wrapper.
Any arguments passed to gw are passed to the wrapper.
If there is no gradle wrapper it tries to use gradle from $PATH.
Works on Linux, MacOS and Windows. 

Call ``gw build`` anywhere in your gradle project and it will call `./gradlew build` in the directory of the nearest gradle build file.

| **without gw**                            | **with gw** |    
|                                       ---:|:---         |
|`./gradlew build`                          | `gw build`  |
|`../gradlew build`                         | `gw build`  |
|`../../gradlew build`                      | `gw build`  |
|`...`                                      |             |
|                                           |             |
|`gradle build`                             | `gw build`  |
|`gradle -b ../build.gradle build`          | `gw build`  |
|`gradle -b ../../build.gradle build`       | `gw build`  |
|`...`                                      |             |

## Installation

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``cargo install gw``
3. Use ``gw`` in one of your gradle projects

or use a released binary:

1. Download a [release](https://github.com/normartin/rust-gradlew-locator/releases) binary for your platform
2. Rename it to ``gw`` or on windows ``gw.exe``
3. Put it on your $PATH
4. Enjoy ``gw`` in your gradle projects
