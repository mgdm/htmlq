use assert_cmd::prelude::*;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

#[test]
fn happy_path2() -> Result<(), Box<dyn std::error::Error>> {
    let input = "<html><head></head><body><div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div></body></html>".to_string();

    let mut process = Command::cargo_bin("htmlq")?
        .arg(".hi")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = process.stdin.take().unwrap();
    let mut writer = BufWriter::new(&mut stdin);

    let mut stdout = process.stdout.take().unwrap();
    let out = BufReader::new(&mut stdout);

    writer.write_all(input.as_bytes()).unwrap();
    writer.flush().unwrap();
    drop(writer);
    drop(stdin);

    let exit_code = process.wait().expect("failed wait for process");
    assert!(exit_code.success());

    for line in out.lines() {
        println!("out: {}", line?);
    }

    Ok(())
}
