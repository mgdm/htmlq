use assert_cmd::prelude::*;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};

#[test]
fn happy_path2() -> Result<(), Box<dyn std::error::Error>> {
    let input = "<html><head></head><body><div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div></body></html>".to_string();

    let mut process = Command::cargo_bin("htmlq")?
        .arg(".hi")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("htmlq process gets spawned");

    let mut stdin = process.stdin.take().expect("take stdin from process");
    let mut writer = BufWriter::new(&mut stdin);

    let mut stdout = process.stdout.take().expect("take stdout from process");
    let mut out = BufReader::new(&mut stdout);

    writer
        .write_all(input.as_bytes())
        .expect("writer to the stdin of process");
    writer.flush().expect("writer flush");
    drop(writer);
    drop(stdin);

    let exit_code = process.wait().expect("failed wait for process");
    assert!(exit_code.success());
    let mut buf = String::new();
    out.read_to_string(&mut buf).expect("read stdout to string");

    assert_eq!(
        buf,
        "<div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div>\n"
    );

    Ok(())
}
