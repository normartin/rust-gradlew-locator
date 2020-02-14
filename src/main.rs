use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::io::Error;
use std::{env, fs};
use std::path::PathBuf;

static GRADLEW_NIX: &str = "gradlew";

fn main() {
    let current_dir = env::current_dir().expect("no current dir :-9?");
    println!("You are in {}", current_dir.display());

    //let p = Path::new("./test");
    //list_dir(p.to_path_buf());
    list_dir(current_dir);
}

fn list_dir(dir: PathBuf) {
    println!("in {:?}", dir);

    let found = find_wrapper_in_dir(&dir);

    match found {
        Some(wrapper) => {
            println!("Found it! {}", wrapper.display());
            let e = execute_unix(&wrapper);
            println!("Executed {}", e);
        },
        None => match dir.parent() {
            Some(parent) => list_dir(parent.to_path_buf()),
            None => {
                println!("Did not find gradlew wrapper!");
                exit(1)
            }
        }
    }
}

fn find_wrapper_in_dir(dir: &PathBuf) -> Option<PathBuf>{
    let files = fs::read_dir(dir).expect("Failed to list contents of ");
    for file in files {
        match file {
            Ok(file) => {
                println!("file: {}", file.path().display());

                if file.path().ends_with(PathBuf::from(GRADLEW_NIX)) {
                    return Some(file.path())
                }

            },
            Err(_e) => {
                println!("Error reading dir entry {}", _e)
            }
        }
    }
    None
}

pub fn execute_unix(exe: &PathBuf) -> Error {
    Command::new(exe).args(env::args()).exec()
}

//pub fn execute_win(exe: &str, args: &[&str]) -> Result<ExitStatus> {
//    Command::new(exe).args(args).spawn()?.wait()
//}
