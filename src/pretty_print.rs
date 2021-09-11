use html5ever::serialize::AttrRef;
use html5ever::serialize::HtmlSerializer;
use html5ever::serialize::Serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::serialize::Serializer;
use html5ever::serialize::TraversalScope;
use html5ever::QualName;
// use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;
use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::str;

lazy_static! {
    static ref INLINE_ELEMENTS: HashSet<&'static str> = vec![
        "a", "abbr", "acronym", "audio", "b", "bdi", "bdo", "big", "button", "canvas", "cite",
        "code", "data", "datalist", "del", "dfn", "em", "embed", "i", "iframe", "img", "input",
        "ins", "kbd", "label", "map", "mark", "meter", "noscript", "object", "output", "picture",
        "progress", "q", "ruby", "s", "samp", "script", "select", "slot", "small", "span",
        "strong", "sub", "sup", "svg", "template", "textarea", "time", "u", "tt", "var", "video",
        "wbr",
    ]
    .into_iter()
    .collect();
}

fn is_inline(name: &str) -> bool {
    INLINE_ELEMENTS.contains(name)
}

struct PrettyPrint<W: Write> {
    indent: usize,
    previous_was_block: bool,
    inner: HtmlSerializer<W>,
}

impl<W: Write> Serializer for PrettyPrint<W> {
    fn start_elem<'a, AttrIter>(&mut self, name: QualName, attrs: AttrIter) -> io::Result<()>
    where
        AttrIter: Iterator<Item = AttrRef<'a>>,
    {
        let inline = is_inline(&name.local);
        if !inline || self.previous_was_block {
            self.inner.writer.write_all(b"\n")?;
            self.inner.writer.write_all(&vec![b' '; self.indent])?;
        }

        self.indent += 2;
        self.inner.start_elem(name, attrs)?;

        Ok(())
    }

    fn end_elem(&mut self, name: QualName) -> io::Result<()> {
        self.indent -= 2;

        if is_inline(&name.local) {
            self.previous_was_block = false;
        } else {
            self.inner.writer.write_all(b"\n")?;
            self.inner.writer.write_all(&vec![b' '; self.indent])?;
            self.previous_was_block = true;
        }

        self.inner.end_elem(name)
    }

    fn write_text(&mut self, text: &str) -> io::Result<()> {
        if text.trim().is_empty() {
            Ok(())
        } else {
            if self.previous_was_block {
                self.inner.writer.write_all(b"\n")?;
                self.inner.writer.write_all(&vec![b' '; self.indent])?;
            }

            self.previous_was_block = false;
            self.inner.write_text(text)
        }
    }

    fn write_comment(&mut self, text: &str) -> io::Result<()> {
        self.inner.write_comment(text)
    }

    fn write_doctype(&mut self, name: &str) -> io::Result<()> {
        self.inner.write_doctype(name)
    }

    fn write_processing_instruction(&mut self, target: &str, data: &str) -> io::Result<()> {
        self.inner.write_processing_instruction(target, data)
    }
}

pub fn pretty_print(node: &NodeRef) -> String {
    let mut content: Vec<u8> = Vec::new();
    let mut pp = PrettyPrint {
        indent: 0,
        previous_was_block: false,
        inner: HtmlSerializer::new(
            &mut content,
            SerializeOpts {
                traversal_scope: TraversalScope::IncludeNode,
                ..Default::default()
            },
        ),
    };
    Serialize::serialize(node, &mut pp, TraversalScope::IncludeNode).unwrap();
    str::from_utf8(content.as_ref()).unwrap().to_owned()
}
