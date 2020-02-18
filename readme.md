
# GW
gw is a [gdub](https://github.com/dougborg/gdub) clone written in Rust for fun.
It looks for a gradle build file (build.gradle or build.gradle.kts) and wrapper (gradlew or gradlew.bat) in your CWD and 
its parent directories and calls the wrapper in the directory of the build file.
Any arguments passed to gw are passed to the wrapper.

Call ``gw build`` anywhere in your gradle project and it will call `./gradlew build` in the directory of the nearest build file.

    gw build # instead of # ./gradlew build
    
    gw build # instead of # ../gradlew build

    gw build # instead of # ../../gradlew build
    ...
 


## Installation

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``cargo install --git https://github.com/normartin/rust-gradlew-locator``
3. Use ``gw`` in one of your gradle projects

or use a released binary:

1. Download a [release](https://github.com/normartin/rust-gradlew-locator/releases) binary for your platform
2. Rename it to ``gw`` or on windows ``gw.exe``
3. Put it on your $PATH
4. Enjoy ``gw`` in your gradle projects
