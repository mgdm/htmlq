use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::{BufReader, BufWriter, Write};
use std::process::{Command, Stdio}; // Run programs

#[test]
fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
    let input = "<html><head></head><body><div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div></body></html>".to_string();
    let mut cmd = Command::cargo_bin("htmlq")?;
    cmd.arg(".hi");

    let mut process = cmd.stdin(Stdio::piped()).spawn().unwrap();
    write!(process.stdin.as_ref().unwrap(), "{}", input).unwrap();

    let ecode = process.wait().expect("failed wait for process");
    assert!(ecode.success());

    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn happy_path2() -> Result<(), Box<dyn std::error::Error>> {
    let input = "<html><head></head><body><div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div></body></html>".to_string();

    let mut process = Command::cargo_bin("htmlq")?
        .arg(".hi")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = process.stdin.as_ref().unwrap();
    let mut writer = BufWriter::new(&mut stdin);
    // let mut stdout = process.stdout.unwrap();
    // let mut out = BufReader::new(&mut stdout);

    writer.write_all(input.as_bytes()).unwrap();

    let exit_code = process.wait().expect("failed wait for process");
    assert!(exit_code.success());

    Ok(())
}
