use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

const BIN: &'static str = env!("CARGO_PKG_NAME");

#[test]
fn can_run_gw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is gradlew. You made it!"));

    Ok(())
}

#[test]
fn can_pass_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests");
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
    cmd.current_dir("./tests/subproject");
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is gradlew. You made it!"));

    Ok(())
}

#[test]
fn can_fail_to_find_gradlew() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir(".");
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Did not find build.gradlew or build.gradle.kt file!",
    ));

    Ok(())
}

#[test]
fn uses_directory_of_build_file_as_working_dir() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/subproject/");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"cwd subproject\""))
        .stdout(predicate::str::contains("src").not());

    Ok(())
}

#[test]
fn uses_directory_of_build_file_as_working_dir_deep() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN)?;
    cmd.current_dir("./tests/subproject/src");
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
