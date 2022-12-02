extern crate html5ever;
extern crate kuchiki;

#[macro_use]
extern crate lazy_static;

mod link;
mod pretty_print;

use clap::{App, Arg, ArgMatches};
use kuchiki::traits::*;
use kuchiki::NodeRef;
use std::borrow::BorrowMut;
use std::error::Error;
use std::fs::File;
use std::io;
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
    remove_nodes: Option<Vec<String>>,
    attributes: Option<Vec<String>>,
}

impl Config {
    fn from_args(matches: ArgMatches) -> Option<Config> {
        let attributes = matches
            .values_of("attribute")
            .map(|values| values.map(String::from).collect());

        let remove_nodes = matches
            .values_of("remove_nodes")
            .map(|values| values.map(String::from).collect());

        let selector: String = match matches.values_of("selector") {
            Some(values) => values.collect::<Vec<&str>>().join(" "),
            None => String::from("html"),
        };

        let base = matches.value_of("base").map(|b| b.to_owned());

        Some(Config {
            input_path: String::from(matches.value_of("filename").unwrap_or("-")),
            output_path: String::from(matches.value_of("output").unwrap_or("-")),
            base,
            detect_base: matches.is_present("detect_base"),
            text_only: matches.is_present("text_only"),
            ignore_whitespace: matches.is_present("ignore_whitespace"),
            pretty_print: matches.is_present("pretty_print"),
            remove_nodes,
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
            remove_nodes: None,
            attributes: Some(vec![]),
        }
    }
}

fn select_attributes(node: &NodeRef, attributes: &[String], output: &mut dyn io::Write) {
    if let Some(as_element) = node.as_element() {
        for attr in attributes {
            if let Ok(elem_atts) = as_element.attributes.try_borrow() {
                if let Some(val) = elem_atts.get(attr.as_str()) {
                    writeln!(output, "{}", val).ok();
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
            result.push('\n');
        }
    }

    result
}

fn get_config<'a, 'b>() -> App<'a, 'b> {
    App::new("htmlq")
        .version("0.4.0")
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
            Arg::with_name("remove_nodes")
                .long("remove-nodes")
                .short("r")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true)
                .value_name("SELECTOR")
                .help("Remove nodes matching this expression before output. May be specified multiple times")
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
        f => Box::new(File::open(f).expect("should have opened input file")),
    };

    let stdout = std::io::stdout();
    let mut output: Box<dyn io::Write> = match config.output_path.as_ref() {
        "-" => Box::new(stdout.lock()),
        f => Box::new(File::create(f).expect("should have created output file")),
    };

    let document = kuchiki::parse_html().from_utf8().read_from(&mut input)?;

    let base: Option<Url> = match (&config.base, &config.detect_base) {
        (Some(base), true) => link::detect_base(&document).or(Url::parse(&base).ok()),
        (Some(base), false) => Url::parse(&base).ok(),
        (None, true) => link::detect_base(&document),
        _ => None,
    };

    let remove_node_selector = match config.remove_nodes {
        Some(ref remove_node_selectors) => remove_node_selectors.join(","),
        None => Default::default(),
    };

    document
        .select(&config.selector)
        .expect("Failed to parse CSS selector")
        .filter(|noderef| {
            if let Ok(mut node) = noderef.as_node().select_first(&remove_node_selector) {
                node.borrow_mut().as_node().detach();
                false
            } else {
                true
            }
        })
        .map(|node| {
            if let Some(base) = &base {
                link::rewrite_relative_url(node.as_node(), &base)
            }
            node
        })
        .for_each(|matched_noderef| {
            let node = matched_noderef.as_node();

            if let Some(attributes) = &config.attributes {
                select_attributes(node, attributes, &mut output);
                return;
            }

            if config.text_only {
                // let content = serialize_text(node, config.ignore_whitespace);
                // output.write_all(format!("{}\n", content).as_ref()).ok();
                writeln!(output, "{}", serialize_text(node, config.ignore_whitespace)).ok();
                return;
            }

            if config.pretty_print {
                // let content = pretty_print::pretty_print(node);
                // output.write_all(content.as_ref()).ok();
                writeln!(output, "{}", pretty_print::pretty_print(node)).ok();
                return;
            }

            writeln!(output, "{}", node.to_string()).ok();
            // let mut content: Vec<u8> = Vec::new();
            // let Ok(_) = node.serialize(&mut content) else {
            //     return
            // };
            // output.write_all(format!("{}\n", content).as_ref()).ok();
        });

    Ok(())
}
