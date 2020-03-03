
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

## Manual installation

1. Download a [release](https://github.com/normartin/rust-gradlew-locator/releases) binary for your platform
2. Rename it to ``gw`` or on windows ``gw.exe``
3. Put it on your $PATH
4. Enjoy ``gw`` in your gradle projects

## Installation on windows

1. Run "Powershell" from the start menu.
2. Paste following script block and hit _Enter_. It will always install the latest version.
    ```powershell
    $BinPath = $HOME + "\.bin\"
    New-Item -ItemType Directory -Force -Path $BinPath
    Invoke-WebRequest https://github.com/normartin/rust-gradlew-locator/releases/latest/download/gw-windows-latest.exe -OutFile ($BinPath + "gw.exe")
    IF (($Env:PATH) -notcontains $BinPath) { [Environment]::SetEnvironmentVariable("Path", $env:Path + ";" + $BinPath, "User") }
    ```
3. Restart your applications to make them pick up the new $PATH.

### Installation via cargo

1. Install Rust's [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. ``cargo install gw``
3. Use ``gw`` in one of your gradle projects
