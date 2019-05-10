extern crate html5ever;
extern crate kuchiki;

use clap::{App, Arg, ArgMatches};
use kuchiki::traits::*;
use kuchiki::NodeRef;
use std::fs::File;
use std::io;
use std::str;

#[derive(Debug, Clone)]
struct Config {
    input_path: String,
    output_path: String,
    selector: String,
    text_only: bool,
    ignore_whitespace: bool,
    attributes: Option<Vec<String>>,
}

impl Config {
    fn from_args(matches: ArgMatches) -> Option<Config> {
        let attributes: Option<Vec<String>> = match matches.values_of("attribute") {
            Some(values) => Some(values.map(|s| String::from(s)).collect()),
            None => None,
        };

        let selector: String = match matches.values_of("selector") {
            Some(values) => values.collect::<Vec<&str>>().join(" "),
            None => String::from(""),
        };

        Some(Config {
            input_path: String::from(matches.value_of("filename").unwrap_or("-")),
            output_path: String::from(matches.value_of("output").unwrap_or("-")),
            text_only: matches.is_present("text_only"),
            ignore_whitespace: matches.is_present("ignore_whitespace"),
            attributes,
            selector,
        })
    }
}

fn select_attributes(node: &NodeRef, attributes: &Vec<String>, output: &mut io::Write) {
    if let Some(as_element) = node.as_element() {
        for attr in attributes {
            if let Ok(elem_atts) = as_element.attributes.try_borrow() {
                if let Some(val) = elem_atts.get(attr.as_str()) {
                    output.write_all(format!("{}\n", val).as_ref()).unwrap();
                }
            }
        }
    }
}

fn serialize_text(node: &NodeRef, ignore_whitespace: bool) -> String {
    let mut result = String::new();
    for text_node in node.inclusive_descendants().text_nodes() {
        if ignore_whitespace && text_node.borrow().trim().len() == 0 {
            continue;
        }

        result.push_str(&text_node.borrow());

        if ignore_whitespace {
            result.push_str("\n");
        }
    }

    result
}

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
                .help("Output only the contents of text nodes inside selected elements"),
        )
        .arg(
            Arg::with_name("ignore_whitespace")
                .short("w")
                .long("ignore-whitespace")
                .help("When printing text nodes, ignore those that consist entirely of whitespace"),
        )
        .arg(
            Arg::with_name("attribute")
                .short("a")
                .long("attribute")
                .takes_value(true)
                .help("Only return this attribute (if present) from selected elements"),
        )
        .arg(
            Arg::with_name("selector")
                .multiple(true)
                .required(true)
                .help("The CSS expression to select"),
        )
        .get_matches();

    let config = Config::from_args(matches).unwrap();

    let mut input: Box<io::Read> = match config.input_path.as_ref() {
        "-" => Box::new(std::io::stdin()),
        f => Box::new(File::open(f).unwrap()),
    };

    let stdout = std::io::stdout();
    let mut output: Box<io::Write> = match config.output_path.as_ref() {
        "-" => Box::new(stdout.lock()),
        f => Box::new(File::create(f).unwrap()),
    };

    let document = kuchiki::parse_html()
        .from_utf8()
        .read_from(&mut input)
        .unwrap();

    for css_match in document.select(&config.selector).unwrap() {
        let node = css_match.as_node();

        if let Some(attributes) = &config.attributes {
            select_attributes(node, attributes, &mut output);
        } else {
            if config.text_only {
                let content = serialize_text(node, config.ignore_whitespace);
                output.write_all(format!("{}\n", content).as_ref()).unwrap();
            } else {
                let mut content: Vec<u8> = Vec::new();
                node.serialize(&mut content).unwrap();
                output
                    .write_all(format!("{}\n", str::from_utf8(&content).unwrap()).as_ref())
                    .unwrap();
            }
        }
    }
}
