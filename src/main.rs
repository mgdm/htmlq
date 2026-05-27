extern crate html5ever;
extern crate kuchikiki;

mod link;
mod pretty_print;

use clap::Parser;
use kuchikiki::traits::*;
use kuchikiki::NodeRef;
use std::error::Error;
use std::fs::File;
use std::io;
use url::Url;

#[derive(Debug, Clone, Parser)]
#[command(version, author, about)]
struct Config {
    /// What CSS selector to filter with.
    #[arg(default_value = "html")]
    selector: String,

    /// Where to read HTML input from.
    #[arg(short = 'f', long = "filename", default_value = "-")]
    input_path: String,

    /// Where to write the filtered HTML to.
    #[arg(short = 'o', long = "output", default_value = "-")]
    output_path: String,

    /// What URL to prepend to links without an origin, i.e. starting with a slash (/).
    #[arg(short, long)]
    base: Option<String>,

    /// Look for the `<base>` tag in input for the base.
    #[arg(short = 'B', long)]
    detect_base: bool,

    /// Output only the contained text of the filtered nodes, not the entire HTML.
    #[arg(short, long = "text")]
    text_only: bool,

    /// Skip over text nodes whose text that is solely whitespace.
    #[arg(short, long)]
    ignore_whitespace: bool,

    /// If to reformat the HTML to be more nicely user-readable.
    #[arg(short, long = "pretty")]
    pretty_print: bool,

    /// Do not output the nodes matching any of these selectors.
    #[arg(short, long)]
    remove_nodes: Vec<String>,

    /// Output only the contents of the given attributes.
    #[arg(short, long)]
    attributes: Vec<String>,
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

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    let mut input: Box<dyn io::Read> = match config.input_path.as_ref() {
        "-" => Box::new(std::io::stdin()),
        f => Box::new(File::open(f).expect("should have opened input file")),
    };

    let stdout = std::io::stdout();
    let mut output: Box<dyn io::Write> = match config.output_path.as_ref() {
        "-" => Box::new(stdout.lock()),
        f => Box::new(File::create(f).expect("should have created output file")),
    };

    let document = kuchikiki::parse_html().from_utf8().read_from(&mut input)?;

    let base: Option<Url> = match (&config.base, &config.detect_base) {
        (Some(base), true) => link::detect_base(&document).or(Url::parse(&base).ok()),
        (Some(base), false) => Url::parse(&base).ok(),
        (None, true) => link::detect_base(&document),
        _ => None,
    };

    let remove_node_selector = config.remove_nodes.join(",");

    document
        .select(&config.selector)
        .expect("Failed to parse CSS selector")
        .inspect(|noderef| {
            let Ok(remove) = noderef.as_node().select_first(&remove_node_selector) else {
                return;
            };

            remove.as_node().detach();
        })
        .map(|node| {
            if let Some(base) = &base {
                link::rewrite_relative_url(node.as_node(), &base)
            }
            node
        })
        .for_each(|matched_noderef| {
            let node = matched_noderef.as_node();

            if !config.attributes.is_empty() {
                select_attributes(node, &config.attributes, &mut output);
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
