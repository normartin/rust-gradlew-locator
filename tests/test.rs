use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*;
use std::process::Command; // Run programs // Used for writing assertions

#[test]
fn can_run_gw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir("./tests");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is gradlew. You made it!"));

    Ok(())
}

#[test]
fn can_pass_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
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
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir("./tests/deep");
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("This is gradlew. You made it!"));

    Ok(())
}

#[test]
fn can_fail_to_find_gradlew() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir(".");
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Did not find gradlew wrapper!"));

    Ok(())
}

#[test]
fn uses_directoy_of_wrapper_as_workdir() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir("./tests/deep");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"cwd tests\""))
        .stdout(predicate::str::contains("deep").not());

    Ok(())
}

#[test]
fn returns_failure_if_gradlew_fails() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir("./tests");
    cmd.arg("fail");
    cmd.assert().failure();

    Ok(())
}
