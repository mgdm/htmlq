use html5ever::local_name;
use kuchiki::NodeRef;
use url::Url;

pub fn rewrite_relative_url(node: &NodeRef, base: &Url) {
    let Some(elem) = node.as_element() else {
        return
    };
    if !(local_name!("a") == elem.name.local
        || local_name!("link") == elem.name.local
        || local_name!("area") == elem.name.local)
    {
        return;
    };
    let mut attrs = elem.attributes.borrow_mut();

    if attrs.contains("href") {
        let Some(url) = attrs.get_mut("href") else {
            return
        };
        if url.starts_with("////") {
            *url = url.trim_start_matches('/').to_string();
            return;
        }
        let new_url = base.join(url).ok().unwrap_or_else(|| base.to_owned());
        attrs.insert("href", new_url.to_string());
    }
}

pub fn detect_base(document: &NodeRef) -> Option<Url> {
    let Ok(node) = document.select_first("base") else {
        return None
    };

    let attrs = node.attributes.borrow();

    if attrs.contains("href") {
        let href = attrs
            .get("href")
            .expect("should have retrieved href from node attributes");
        return match Url::parse(href) {
            Ok(url) => Some(url),
            _ => None,
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use html5ever::tendril::TendrilSink;

    use super::*;

    macro_rules! rewrite_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (mut input, expected) = $value;
                let base = Url::parse("https://mgdm.net").unwrap();
                let doc = make_doc(&mut input);
                for css_match in doc
                    .select("a, area, link")
                    .expect("Failed to parse CSS selector while doing link rewriting")
                {
                    let node = css_match.as_node();
                    rewrite_relative_url(&node, &base);
                }

                let result = serialize_doc(&doc);
                assert_eq!(expected, result);
            }
        )*
        }
    }

    macro_rules! detect_base_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (mut input, expected) = $value;
                let doc = make_doc(&mut input);
                let result = detect_base(&doc);
                assert_eq!(expected, result);
            }
        )*
        }
    }

    fn make_doc(html: &mut String) -> NodeRef {
        kuchiki::parse_html()
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap()
    }

    fn serialize_doc(doc: &NodeRef) -> String {
        let mut content: Vec<u8> = Vec::new();
        doc.serialize(&mut content).unwrap();
        std::str::from_utf8(&content).unwrap().to_string()
    }

    rewrite_tests! {
        rewrite_a_href: (
            "<html><head></head><body><a href=\"/foo/bar\">Hello</a></body></html>".to_string(),
            "<html><head></head><body><a href=\"https://mgdm.net/foo/bar\">Hello</a></body></html>".to_string(),
        ),
        rewrite_link_href: (
            "<html><head><link  href=\"/style.css\" rel=\"stylesheet\"/></head><body>Hello</body></html>".to_string(),
            "<html><head><link href=\"https://mgdm.net/style.css\" rel=\"stylesheet\"></head><body>Hello</body></html>".to_string(),
        ),
        rewrite_map_area_href: (
            "<html><head></head><body><map name=\"primary\"><area coords=\"75,75,75\" href=\"left.html\" shape=\"circle\"></map></body></html>".to_string(),
            "<html><head></head><body><map name=\"primary\"><area coords=\"75,75,75\" href=\"https://mgdm.net/left.html\" shape=\"circle\"></map></body></html>".to_string()
        ),
        do_not_rewrite_absolute_url: (
            "<html><head></head><body><a href=\"https://example.org/foo/bar\">Hello</a></body></html>".to_string(),
            "<html><head></head><body><a href=\"https://example.org/foo/bar\">Hello</a></body></html>".to_string(),
        ),
    }

    detect_base_tests! {
        base_ok: (
            "<html><head><base href=\"https://example.org\"></head><body><a href=\"https://example.org/foo/bar\">Hello</a></body></html>".to_string(),
            Some(Url::parse("https://example.org").unwrap())
        ),
        base_not_found: (
            "<html><head></head><body><a href=\"https://example.org/foo/bar\">Hello</a></body></html>".to_string(),
            None
        ),
    }
}
