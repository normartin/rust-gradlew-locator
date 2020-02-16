use itertools::join;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

#[cfg(unix)]
static GRADLEW: &str = "gradlew";

#[cfg(windows)]
static GRADLEW: &str = "gradlew.bat";

fn main() {
    #[cfg(windows)]
    ctrlc::set_handler(move || {
        // ignore SIGINT and let the child process handle it
        // this is required for windows batch "Terminate batch job (Y/N)"
    })
    .expect("Error installing Ctrl-C handler");

    let current_dir = env::current_dir().expect("no current dir :-9?");

    let wrapper = find_wrapper(current_dir);

    match wrapper {
        None => {
            eprint!("Did not find gradlew wrapper!");
            exit(1)
        }
        Some((wrapper, dir)) => execute(&wrapper, dir),
    }
}

fn find_wrapper(dir: PathBuf) -> Option<(PathBuf, PathBuf)> {
    let found = find_wrapper_in_dir(&dir);

    match found {
        Some(wrapper) => Some((wrapper, dir)),
        None => match dir.parent() {
            Some(parent) => find_wrapper(parent.to_path_buf()),
            None => None,
        },
    }
}

fn find_wrapper_in_dir(dir: &PathBuf) -> Option<PathBuf> {
    let files = fs::read_dir(dir).expect("Failed to list contents!");

    files
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|file| file.ends_with(PathBuf::from(GRADLEW)))
}

// https://stackoverflow.com/a/53479765
pub fn execute(gradle_path: &PathBuf, working_directory: PathBuf) {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("Executing {} {}", gradle_path.display(), join(&args, " "));

    let spawn_result = Command::new(gradle_path)
        .current_dir(working_directory)
        .args(args)
        .spawn();

    let result = spawn_result.and_then(|mut child| child.wait());

    match result {
        Ok(status) => exit(status.code().unwrap_or(1)),

        Err(e) => {
            eprintln!("Failed {}", e);
            exit(1)
        }
    }
}
