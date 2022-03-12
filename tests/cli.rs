use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn happy_path3() {
    let input = "<html><head></head><body><div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div></body></html>";
    let expected_out = "<div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div>\n";

    Command::cargo_bin("htmlq")
        .unwrap()
        .arg(".hi")
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::diff(expected_out));
}
