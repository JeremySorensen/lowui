use serde::Serialize;
use std::fmt;

/// An HTML attribute
#[derive(Debug, Serialize)]
pub struct Attr {
    pub name: &'static str,
    pub value: Option<String>,
}

impl Attr {
    /// Returns a new Attr with key and value
    pub fn new<T: Into<String>>(name: &'static str, value: T) -> Self {
        Self {
            name: name,
            value: Some(value.into()),
        }
    }

    /// Returns a new Attr with no value
    pub fn name_only(name: &'static str) -> Self {
        Self {
            name: name,
            value: None,
        }
    }
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, r#"{}="{}""#, self.name, value)
        } else {
            write!(f, r#"{}"#, self.name)
        }
    }
}

/// A &lt;link rel="..."&gt; element
#[derive(Debug)]
pub struct Link {
    /// The rel attribute
    pub rel: String,
    /// Other option attributes
    pub attr: Option<Vec<Attr>>,
}

impl Link {
    /// Returns a new Link with no attributes except rel
    pub fn new<T: Into<String>>(rel: T) -> Link {
        Link {
            rel: rel.into(),
            attr: None,
        }
    }

    /// Returns a new Link with addtional attributes
    pub fn with_attr<T: Into<String>>(rel: T, attr: Vec<Attr>) -> Link {
        Link {
            rel: rel.into(),
            attr: Some(attr),
        }
    }
}

/// An HTML element or text node
#[derive(Debug, Serialize)]
pub enum Node {
    Element {
        /// The element type, or tag
        tag: &'static str,
        /// Any attributes of the element
        attr: Vec<Attr>,
        /// Any children of the element
        children: Vec<Node>,
        /// True if element is empty like &lt;br&gt;
        is_empty: bool,
    },
    Text(String),
}

impl Node {
    pub(crate) fn new_el(
        tag: &'static str,
        attr: Vec<Attr>,
        children: Vec<Node>,
        is_empty: bool,
    ) -> Self {
        Self::Element {
            tag: tag,
            attr: attr,
            children: children,
            is_empty: is_empty,
        }
    }

    pub(crate) fn new_text<T: Into<String>>(text: T) -> Self {
        Node::Text(text.into())
    }

    pub(crate) fn to_html(&self) -> String {
        match self {
            Self::Element {
                tag,
                attr,
                children,
                is_empty,
            } => {
                let space;
                let attr_txt;
                if attr.is_empty() {
                    space = "".to_string();
                    attr_txt = "".to_string();
                } else {
                    space = " ".to_string();
                    attr_txt = attr_to_html(&attr);
                }

                if *is_empty {
                    format!("<{t}{s}{a}>", t = tag, s = space, a = attr_txt,)
                } else {
                    let children = if children.is_empty() {
                        "".to_string()
                    } else {
                        children_to_html(children)
                    };

                    format!(
                        "<{t}{s}{a}>{c}</{t}>",
                        t = tag,
                        s = space,
                        a = attr_txt,
                        c = children
                    )
                }
            }

            Self::Text(t) => t.to_string(),
        }
    }

    fn add_events(&mut self) {
        match self {
            Self::Text(_) => return,
            Self::Element {
                tag,
                attr,
                children,
                is_empty: _,
            } => {
                for child in children {
                    child.add_events();
                }

                if attr.iter().find(|a| a.name == "id").is_none() {
                    return;
                }

                if *tag == "button" {
                    attr.push(Attr::new("onclick", "buttonClick()"));
                } else if *tag == "select" {
                    attr.push(Attr::new("onchange", "valueChanged()"));
                } else if *tag == "input" {
                    if let Some(type_attr) = attr.iter().find(|a| a.name == "type") {
                        let kind = type_attr.value.as_ref().unwrap();
                        if kind == "button" {
                            attr.push(Attr::new("onclick", "buttonClick()"));
                        } else if kind == "text" || kind == "number" {
                            attr.push(Attr::new("onchange", "valueChanged()"));
                        } else if kind == "checkbox" || kind == "radio" {
                            attr.push(Attr::new("onchange", "checkChanged()"));
                        }
                    }
                }
            }
        }
    }
}

/// The &lt;head&gt; section of an html page
pub struct Head {
    pub title: String,
    pub links: Vec<Link>,
    pub meta: Vec<Attr>,
}

impl Head {
    /// Returns a new Head with a title links or meta tags can be added later
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
            links: Vec::<Link>::new(),
            meta: Vec::<Attr>::new(),
        }
    }

    /// Adds a link (&lt;link&gt; element) with no attributes
    pub fn link<T: Into<String>>(&mut self, rel: T) {
        self.links.push(Link::new(rel));
    }

    /// Adds a link (&lt;link&gt; element) with one or more attributes
    pub fn link_attr<T: Into<String>>(&mut self, rel: T, attr: Vec<Attr>) {
        self.links.push(Link::with_attr(rel, attr));
    }

    /// Adds a &lt;meta&gt; element
    pub fn meta<T: Into<String>>(&mut self, name: &'static str, content: T) {
        self.meta.push(Attr::new(name, content));
    }
}

/// An HTML page
pub struct Page {
    pub head: Head,
    pub body: Node,
}

impl Page {
    /// Returns a new Page with a Head and body (Node)
    pub fn new(head: Head, body: Node) -> Self {
        Page {
            head: head,
            body: body,
        }
    }

    pub(crate) fn add_events(&mut self) {
        self.body.add_events();
    }
}

fn children_to_html(children: &[Node]) -> String {
    children
        .iter()
        .map(|c| c.to_html())
        .collect::<Vec<String>>()
        .join("")
}

fn attr_to_html(attr: &[Attr]) -> String {
    attr.iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
