use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum VDom {
    Element {
        tag: String,
        attr: Option<Vec<(String, String)>>,
        children: Vec<VDom>,
    },
    Text(String),
}

impl VDom {
    pub fn with_child(tag: &str, child: VDom) -> VDom {
        VDom::Element {
            tag: tag.to_string(),
            attr: None,
            children: vec![child],
        }
    }

    pub fn with_children(tag: &str, children: Vec<VDom>) -> VDom {
        VDom::Element {
            tag: tag.to_string(),
            attr: None,
            children: children,
        }
    }

    pub fn with_child_attr(tag: &str, attr: Vec<(String, String)>, child: VDom) -> VDom {
        VDom::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: vec![child],
        }
    }

    pub fn with_children_attr(tag: &str, attr: Vec<(String, String)>, children: Vec<VDom>) -> VDom {
        VDom::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: children,
        }
    }

    pub fn with_text(tag: &str, text: String) -> VDom {
        VDom::Element {
            tag: tag.to_string(),
            attr: None,
            children: vec![VDom::Text(text)],
        }
    }

    pub fn with_text_attr(tag: &str, text: String, attr: Vec<(String, String)>) -> VDom {
        VDom::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: vec![VDom::Text(text)],
        }
    }

    pub fn to_html(&self) -> String {
        match self {
            VDom::Element {
                tag,
                attr,
                children,
            } => {
                let children = children_to_html(&children);

                let space;
                let attr_txt;
                if let Some(attr) = attr {
                    space = " ".to_string();
                    attr_txt = attr_to_html(&attr);
                } else {
                    space = "".to_string();
                    attr_txt = "".to_string();
                }

                format!(
                    "<{t}{s}{a}>{c}</{t}>",
                    t = tag,
                    s = space,
                    a = attr_txt,
                    c = children
                )
            }

            VDom::Text(t) => t.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn add_events(&mut self) {
        match self {
            Self::Text => return;
            Self::Element(element) => {
                for child in &children {
                    child.add_events();
                }
                
                if child.attr.iter.find(|(k, v)| -> k == "id") == None {
                    return;
                }

                match element.tag {
                    "input" => {
                        match element.attr.iter.find(|(k, v) -> k = "type") {
                            "button" => element.attr.push(("onclick", "buttonClick(this)")),
                            "checkbox" |
                            "radio" |
                            "number" |
                            "select" |
                            "text" => element.attr.push(("onchange", "valueChanged(this)")),
                        }
                    }
                }
            }
        }
    }
}

fn children_to_html(children: &[VDom]) -> String {
    children
        .iter()
        .map(|c| c.to_html())
        .collect::<Vec<String>>()
        .join("")
}

fn format_attr((key, value): &(String, String)) -> String {
    format!("{k}=\"{v}\"", k = key, v = value)
}

fn attr_to_html(attr: &[(String, String)]) -> String {
    attr.iter()
        .map(format_attr)
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn make_html() {
        let expected =
            "<html><head><title>Hello</title></head><body><h1>Hello</h1><p class=\"p-text\">This is a simple webpage.</p></body></html>"
            .to_string();

        let title = VDom::with_text("title", "Hello".to_string());

        let head = VDom::with_child("head", title);

        let h1 = VDom::with_text("h1", "Hello".to_string());

        let p = VDom::with_text_attr(
            "p",
            "This is a simple webpage.".to_string(),
            vec![(String::from("class"), String::from("p-text"))],
        );

        let body = VDom::with_children("body", vec![h1, p]);

        let html = VDom::with_children("html", vec![head, body]);

        let actual = html.to_html();

        assert_eq!(expected, actual);

        let json = html.to_json();

        println!("{}", json);
    }
}
