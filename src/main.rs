extern crate html5ever;
extern crate kuchiki;

use clap::{App, Arg};
use kuchiki::traits::*;
use std::fs::File;
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
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("The output file. Defaults to stdout")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("text_only")
                .short("t")
                .long("text")
                .help("Output the contents of text elements"),
        )
        .arg(
            Arg::with_name("attributes")
                .short("a")
                .long("attribute")
                .takes_value(true)
                .help("Attributes to return from selected elements"),
        )
        .arg(
            Arg::with_name("selector")
                .multiple(true)
                .required(true)
                .help("The CSS expression to select"),
        )
        .get_matches();

    let input_path = matches.value_of("filename").unwrap_or("-");
    let output_path = matches.value_of("filename").unwrap_or("-");
    let text_only = matches.is_present("text_only");
    let attributes: Option<Vec<&str>> = match matches.values_of("attributes") {
        Some(values) => Some(values.collect()),
        None => None,
    };

    let selector: String = match matches.values_of("selector") {
        Some(values) => values.collect::<Vec<&str>>().join(" "),
        None => String::from(""),
    };

    let mut input: Box<io::Read> = match input_path.as_ref() {
        "-" => Box::new(std::io::stdin()),
        f => Box::new(File::open(f).unwrap()),
    };

    let stdout = std::io::stdout();
    let mut output: Box<io::Write> = match output_path.as_ref() {
        "-" => Box::new(stdout.lock()),
        f => Box::new(File::create(f).unwrap()),
    };

    let document = kuchiki::parse_html()
        .from_utf8()
        .read_from(&mut input)
        .unwrap();

    for css_match in document.select(&selector).unwrap() {
        let as_node = css_match.as_node();

        if let Some(attrs) = &attributes {
            if let Some(as_element) = as_node.as_element() {
                for attr in attrs {
                    if let Ok(elem_atts) = as_element.attributes.try_borrow() {
                        if let Some(val) = elem_atts.get(*attr) {
                            output.write_all(format!("{}\n", val).as_ref()).unwrap();
                        }
                    }
                }
            }
        } else {
            if text_only {
                output
                    .write_all(format!("{}\n", as_node.text_contents()).as_ref())
                    .unwrap();
            } else {
                as_node.serialize(&mut output).unwrap();
            }
        }
    }
}
