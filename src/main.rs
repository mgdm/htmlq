extern crate html5ever;
extern crate kuchiki;

#[macro_use]
extern crate lazy_static;

mod link;
mod pretty_print;

use clap::{App, Arg, ArgMatches};
use kuchiki::traits::*;
use kuchiki::NodeRef;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;
use std::str;
use url::Url;

#[derive(Debug, Clone)]
struct Config {
    input_path: String,
    output_path: String,
    selector: String,
    base: Option<String>,
    detect_base: bool,
    text_only: bool,
    ignore_whitespace: bool,
    pretty_print: bool,
    attributes: Option<Vec<String>>,
}

impl Config {
    fn from_args(matches: ArgMatches) -> Option<Config> {
        let attributes: Option<Vec<String>> = match matches.values_of("attribute") {
            Some(values) => Some(values.map(String::from).collect()),
            None => None,
        };

        let selector: String = match matches.values_of("selector") {
            Some(values) => values.collect::<Vec<&str>>().join(" "),
            None => String::from("html"),
        };

        let base: Option<String> = match matches.value_of("base") {
            Some(val) => Some(val.to_owned()),
            _ => None,
        };

        Some(Config {
            input_path: String::from(matches.value_of("filename").unwrap_or("-")),
            output_path: String::from(matches.value_of("output").unwrap_or("-")),
            base,
            detect_base: matches.is_present("detect_base"),
            text_only: matches.is_present("text_only"),
            ignore_whitespace: matches.is_present("ignore_whitespace"),
            pretty_print: matches.is_present("pretty_print"),
            attributes,
            selector,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_path: "-".to_string(),
            output_path: "-".to_string(),
            selector: "html".to_string(),
            base: None,
            detect_base: false,
            ignore_whitespace: true,
            pretty_print: true,
            text_only: false,
            attributes: Some(vec![]),
        }
    }
}

fn select_attributes(node: &NodeRef, attributes: &[String], output: &mut dyn io::Write) {
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
        if ignore_whitespace && text_node.borrow().trim().is_empty() {
            continue;
        }

        result.push_str(&text_node.borrow());

        if ignore_whitespace {
            result.push_str("\n");
        }
    }

    result
}

fn get_config<'a, 'b>() -> App<'a, 'b> {
    App::new("htmlq")
        .version("0.3.0")
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
            Arg::with_name("pretty_print")
                .short("p")
                .long("pretty")
                .help("Pretty-print the serialised output"),
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
            Arg::with_name("base")
                .short("b")
                .long("base")
                .takes_value(true)
                .help("Use this URL as the base for links"),
        )
        .arg(
            Arg::with_name("detect_base")
                .short("B")
                .long("detect-base")
                .help("Try to detect the base URL from the <base> tag in the document. If not found, default to the value of --base, if supplied"),
        )
        .arg(
            Arg::with_name("selector")
                .default_value("html")
                .multiple(true)
                .help("The CSS expression to select"),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config();
    let matches = config.get_matches();
    let config = Config::from_args(matches).unwrap_or_default();

    let mut input: Box<dyn io::Read> = match config.input_path.as_ref() {
        "-" => Box::new(std::io::stdin()),
        f => Box::new(File::open(f).unwrap()),
    };

    let stdout = std::io::stdout();
    let mut output: Box<dyn io::Write> = match config.output_path.as_ref() {
        "-" => Box::new(stdout.lock()),
        f => Box::new(File::create(f).unwrap()),
    };

    let mut base: Option<Url> = None;
    if let Some(b) = config.base {
        let u = Url::parse(&b);

        if let Err(e) = u {
            eprintln!("Failed to parse the provided base URL: {}", e);
            process::exit(1);
        }

        base = Some(u.unwrap());
    }

    let document = kuchiki::parse_html().from_utf8().read_from(&mut input)?;

    if config.detect_base {
        if let Some(b) = link::detect_base(&document) {
            base = Some(b)
        }
    }

    if let Some(base) = base {
        link::rewrite_relative_urls(&document, &base);
    }

    for css_match in document
        .select(&config.selector)
        .expect("Failed to parse CSS selector")
    {
        let node = css_match.as_node();

        if let Some(attributes) = &config.attributes {
            select_attributes(node, attributes, &mut output);
            continue;
        }

        if config.text_only {
            let content = serialize_text(node, config.ignore_whitespace);
            output.write_all(format!("{}\n", content).as_ref())?;
            continue;
        }

        if config.pretty_print {
            let content = pretty_print::pretty_print(node);
            output.write_all(content.as_ref())?;
            continue;
        }

        let mut content: Vec<u8> = Vec::new();
        node.serialize(&mut content)?;
        output.write_all(format!("{}\n", str::from_utf8(&content)?).as_ref())?;
    }

    Ok(())
}
