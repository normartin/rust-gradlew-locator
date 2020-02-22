use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const BIN: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn can_run_gw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/gradlew_project");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is gradlew. You made it!"));

    Ok(())
}

#[test]
fn can_pass_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/gradlew_project");
    cmd.arg("foo").arg("bar");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("arg 1 foo"))
        .stdout(predicate::str::contains("arg 2 bar"));

    Ok(())
}

#[test]
fn can_run_deep_gw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/gradlew_project/subproject");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is gradlew. You made it!"));

    Ok(())
}

#[test]
fn can_fail_to_find_gradlew() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir(".");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Did not find build.gradle or build.gradle.kts file!",
    ));

    Ok(())
}

// fails on ci windows. TODO: enable
#[cfg(unix)]
#[test]
fn uses_gradle_from_path() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    let mut cmd = Command::new("sh");

    #[cfg(windows)]
    let mut cmd = Command::new("cmd");

    let dir_with_gradle_executable = env::current_dir().unwrap().join(PathBuf::from("tests"));

    #[cfg(unix)]
    let path = format!(
        "{}:{}",
        dir_with_gradle_executable.to_str().unwrap(),
        std::env::var("PATH").unwrap()
    );

    #[cfg(windows)]
    let path = format!(
        "{};{}",
        std::env::var("PATH").unwrap(),
        dir_with_gradle_executable.to_str().unwrap()
    );

    cmd.env("PATH", path);
    cmd.current_dir("./tests/gradle_project");

    #[cfg(windows)]
    cmd.arg("/C");

    #[cfg(unix)]
    cmd.arg("-c");

    cmd.arg(executable_path(BIN));

    cmd.assert()
        .success()
        .stderr(predicate::str::contains(
            "Did not find gradlew wrapper! Trying gradle from $PATH",
        ))
        .stdout(
            predicate::str::contains("This is global gradle. You made it!")
                // in case you have installed gradle on your system
                .or(predicate::str::contains("Welcome to Gradle")),
        );

    Ok(())
}

#[test]
fn uses_directory_of_build_file_as_working_dir() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/gradlew_project/subproject/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"cwd subproject\""))
        .stdout(predicate::str::contains("src").not());

    Ok(())
}

#[test]
fn uses_directory_of_build_file_as_working_dir_deep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/gradlew_project/subproject/src");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"cwd subproject\""))
        .stdout(predicate::str::contains("src").not());

    Ok(())
}

#[test]
fn returns_failure_if_gradlew_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests");
    cmd.arg("fail");
    cmd.assert().failure();

    Ok(())
}

fn executable_path(name: &str) -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    let exe = String::from(name) + std::env::consts::EXE_SUFFIX;
    path.push(exe);
    path
}
