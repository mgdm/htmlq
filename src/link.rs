use kuchiki::NodeRef;
use std::borrow::BorrowMut;
use url::Url;

pub fn rewrite_relative_urls(document: &NodeRef, base: &Url) {
    for mut css_match in document
        .select("a, area, link")
        .expect("Failed to parse CSS selector while doing link rewriting")
    {
        let node = css_match.borrow_mut();
        let mut attrs = node.attributes.borrow_mut();

        if attrs.contains("href") {
            let url = attrs.get("href").unwrap();
            let new_url = base.join(url).unwrap().to_string();
            attrs.insert("href", new_url);
        }
    }
}

#[cfg(test)]
mod tests {
    use html5ever::tendril::TendrilSink;

    use super::*;

    fn make_doc(html: &mut String) -> NodeRef {
        kuchiki::parse_html()
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap()
    }

    #[test]
    fn it_works() {
        let mut html =
            "<html><head></head><body><a href=\"/foo/bar\">Hello</a></body></html>".to_string();
        let expected_html =
            "<html><head></head><body><a href=\"https://mgdm.net/foo/bar\">Hello</a></body></html>";
        let base = Url::parse("https://mgdm.net").unwrap();

        let doc = make_doc(&mut html);
        rewrite_relative_urls(&doc, &base);

        let mut content: Vec<u8> = Vec::new();
        doc.serialize(&mut content).unwrap();
        let result = std::str::from_utf8(&content).unwrap();

        assert_eq!(expected_html, result);
    }
}
