use std::path::PathBuf;
use std::process::{exit, Command};
use std::{env, fs};

#[cfg(unix)]
static GRADLEW: &str = "gradlew";

#[cfg(windows)]
static GRADLEW: &str = "gradlew.bat";

static BUILD_FILE: &str = "build.gradle";
static BUILD_FILE_KT: &str = "build.gradle.kts";

fn main() {
    #[cfg(windows)]
    ctrlc::set_handler(move || {
        // ignore SIGINT and let the child process handle it
        // this is required for windows batch "Terminate batch job (Y/N)"
    })
    .expect("Error installing Ctrl-C handler");

    let current_dir = env::current_dir().expect("no current dir :-9?");

    let found_build_file_path = find_path_containing_recursive(&current_dir, &is_build_file);

    match found_build_file_path {
        None => {
            eprintln!("Did not find {} or {} file!", BUILD_FILE, BUILD_FILE_KT);
            exit(1)
        }
        Some(build_file_path) => {
            let found_wrapper_path = find_path_containing_recursive(&current_dir, &is_gradlew);

            match found_wrapper_path {
                None => {
                    eprintln!("Did not find gradlew wrapper!");
                    exit(1)
                }
                Some(wrapper_path) => {
                    let wrapper_file = wrapper_path.join(PathBuf::from(GRADLEW));
                    execute(&wrapper_file, &build_file_path)
                }
            }
        }
    }
}

fn is_gradlew(path: &PathBuf) -> bool {
    path.ends_with(&PathBuf::from(GRADLEW))
}

fn is_build_file(path: &PathBuf) -> bool {
    path.ends_with(&PathBuf::from(BUILD_FILE)) || path.ends_with(&PathBuf::from(BUILD_FILE_KT))
}

fn find_path_containing_recursive(
    dir: &PathBuf,
    matches: &dyn Fn(&PathBuf) -> bool,
) -> Option<PathBuf> {
    let found = find_file_in_dir(&dir, matches);

    if found {
        Some(dir.clone())
    } else {
        match dir.parent() {
            Some(parent) => find_path_containing_recursive(&parent.to_path_buf(), matches),
            None => None,
        }
    }
}

fn find_file_in_dir(dir: &PathBuf, matches: &dyn Fn(&PathBuf) -> bool) -> bool {
    let files = fs::read_dir(dir).expect("Failed to list contents!");

    files
        .filter_map(Result::ok)
        .any(|entry| matches(&entry.path()))
}

// https://stackoverflow.com/a/53479765
pub fn execute(gradle_path: &PathBuf, working_directory: &PathBuf) {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("Executing {} {}", gradle_path.display(), args.join(" "));

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
