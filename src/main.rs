#![feature(impl_trait_in_bindings)]

extern crate kuchiki;

use std::fs::File;
use clap::{App, Arg};
use kuchiki::traits::*;
use std::io;


fn main() {
    let matches = App::new("htmlq")
        .version("0.0.1")
        .author("Michael Maclean <michael@mgdm.net>")
        .about("Runs CSS selectors on HTML")
        .arg(
            Arg::with_name("filename")
                .short("f")
                .long("filename")
                .value_name("FILE")
                .help("The input file. Defaults to stdin")
                .takes_value(true),
        )
        .arg(Arg::with_name("text_only").short("t").long("text"))
        .arg(Arg::with_name("selector").multiple(true).required(true))
        .get_matches();

    let input_path = matches.value_of("filename").unwrap_or("-");

    let mut input: Box<io::Read> = match input_path.as_ref() {
        "-" => Box::new(std::io::stdin()),
        f => Box::new(File::open(f).unwrap()),
    };

    let selector_parts: Vec<&str> = matches.values_of("selector").unwrap().collect();
    let selector = selector_parts.join(" ");
    let text_only = matches.is_present("text_only");

    let document = kuchiki::parse_html().from_utf8().read_from(&mut input).unwrap();
    let stdout = &mut std::io::stdout();

    for css_match in document.select(&selector).unwrap() {
         let as_node = css_match.as_node();

         if text_only {
             println!("{}", as_node.text_contents());
         } else {
            as_node.serialize(&mut stdout.lock()).unwrap();
         }
    }
}
