use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::{env, fs};
use std::path::PathBuf;
use itertools::join;

#[cfg(unix)]
static GRADLEW: &str = "gradlew";

#[cfg(windows)]
static GRADLEW: &str = "gradlew.bat";

fn main() {
    let current_dir = env::current_dir().expect("no current dir :-9?");
    list_dir(current_dir);
}

fn list_dir(dir: PathBuf) {
    let found = find_wrapper_in_dir(&dir);

    match found {
        Some(wrapper) =>  execute(&wrapper),
        None => match dir.parent() {
            Some(parent) => list_dir(parent.to_path_buf()),
            None => {
                eprint!("Did not find gradlew wrapper!");
                exit(1)
            }
        }
    }
}

fn find_wrapper_in_dir(dir: &PathBuf) -> Option<PathBuf> {
    let files = fs::read_dir(dir).expect("Failed to list contents of ");
    for file in files {
        match file {
            Ok(file) => {
                if file.path().ends_with(PathBuf::from(GRADLEW)) {
                    return Some(file.path());
                }
            }
            Err(_e) => {
                println!("Error reading dir entry {}", _e)
            }
        }
    }
    None
}

#[cfg(unix)]
pub fn execute(exe: &PathBuf) {
    let args = env::args();
    println!("Executing {} {}", exe.display(), join(env::args().skip(1), " "));
    Command::new(exe).args(args).exec();
}
#[cfg(windows)]
pub fn execute(exe: &PathBuf) {
    let args = env::args();
    println!("Executing {} {}", exe.display(), join(env::args().skip(1), " "));
    Command::new(exe).args(args).spawn()?.wait();
}
