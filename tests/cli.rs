use assert_cmd::Command;
use predicates::prelude::*;

macro_rules! cmd_success_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name(){
            let (stdin, args, expected) = $value;
            Command::cargo_bin("htmlq")
                .unwrap()
                .args(args)
                .write_stdin(stdin)
                .assert()
                .success()
                .stdout(predicate::str::diff(expected));
        }
    )*
    }
}

cmd_success_tests!(
    find_by_class: (
        "<html><head></head><body><div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div></body></html>",
        [".hi"],
        "<div class=\"hi\"><a href=\"/foo/bar\">Hello</a></div>\n"
    ),
    find_by_id: (
        "<html><head></head><body><div id=\"my-id\"><a href=\"/foo/bar\">Hello</a></div></body></html>",
        ["#my-id"],
        "<div id=\"my-id\"><a href=\"/foo/bar\">Hello</a></div>\n"
    ),
    remove_links: (
        "<html><head></head><body><div id=\"my-id\"><a href=\"/foo/bar\">Hello</a></div></body></html>",
        ["#my-id", "--remove-nodes", "a"],
        "<div id=\"my-id\"></div>\n",
    ),
);
