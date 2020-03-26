use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Attr {
    pub key: String,
    pub value: Option<String>,
}

impl Attr {
    pub fn new<T: Into<String>, U: Into<String>>(key: T, value: U) -> Self {
        Attr { key: key.into(), value: Some(value.into()) }
    }

    pub fn with_key_only<T: Into<String>>(key: T) -> Self {
        Attr { key: key.into(), value: None }
    }
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, r#"{}="{}""#, self.key, value)
        } else {
            write!(f, r#"{}"#, self.key)
        }
    }
}

pub struct Link {
    pub rel: String,
    pub attr: Option<Vec<Attr>>,
}

impl Link {
    pub fn new<T: Into<String>>(rel: T) -> Link {
        Link { rel: rel.into(), attr: None }
    }

    pub fn with_attr<T: Into<String>>(rel: T, attr: Vec<Attr>) -> Link {
        Link { rel: rel.into(), attr: Some(attr) }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Node {
    Element {
        tag: String,
        attr: Option<Vec<Attr>>,
        children: Vec<Node>,
    },
    Text(String),
}

impl Node {
    pub fn empty<T: Into<String>>(tag: T) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: Vec::<Node>::new(),
        }
    }

    pub fn with_attr<T: Into<String>>(tag: T, attr: Vec<Attr>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: Vec::<Node>::new(),
        }
    }

    pub fn with_child<T: Into<String>>(tag: T, child: Self) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: vec![child],
        }
    }

    pub fn with_children<T: Into<String>>(tag: T, children: Vec<Self>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: children,
        }
    }

    pub fn with_child_attr<T: Into<String>>(tag: T, attr: Vec<Attr>, child: Self) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: vec![child],
        }
    }

    pub fn with_children_attr<T: Into<String>>(tag: T, attr: Vec<Attr>, children: Vec<Self>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: children,
        }
    }

    pub fn with_text<T: Into<String>, U: Into<String>>(tag: T, text: T) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: vec![Self::Text(text.into())],
        }
    }

    pub fn with_text_attr<T: Into<String>, U: Into<String>>(tag: T, text: U, attr: Vec<Attr>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: vec![Self::Text(text.into())],
        }
    }

    pub fn to_html(&self) -> String {
        match self {
            Self::Element {
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

            Self::Text(t) => t.to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn add_events(&mut self) {
        match self {
            Self::Text(_) => return,
            Self::Element{ tag, attr, children } => {
                for child in children {
                    child.add_events();
                }
                
                match attr {
                    None => return,
                    Some(attr_ref) => {
                        if attr_ref.iter().find(|a| { a.key == "id" }).is_none() {
                            return;
                        }

                        if tag == "button" {
                            attr_ref.push(Attr::new("onclick", "buttonClick()"));
                        } else if tag == "select" {
                            attr_ref.push(Attr::new("onchange", "valueChanged()"));
                        } else if tag == "input" {
                            if let Some(type_attr) = attr_ref.iter().find(|a| { a.key == "type" }) {
                                let kind = type_attr.value.as_ref().unwrap();
                                if kind == "button" {
                                    attr_ref.push(Attr::new("onclick", "buttonClick()"));
                                } else if kind == "text" || kind == "number" {
                                    attr_ref.push(Attr::new("onchange", "valueChanged()"));
                                } else if kind == "checkbox" || kind == "radio" {
                                    attr_ref.push(Attr::new("onchange", "checkChanged()"));
                                }
                            }
                        }
                    }
                }
            }
        }
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
        .map(|a| { a.to_string() })
        .collect::<Vec<_>>()
        .join(" ")
}

pub struct Header {
    pub title: String,
    pub links: Vec<Link>,
    pub meta: Vec<Attr>,
}

impl Header {
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
            links: Vec::<Link>::new(),
            meta: Vec::<Attr>::new(),
        }
    }
    
    pub fn add_link<T: Into<String>>(&mut self, rel: T) {
        self.links.push(Link::new(rel));
    }

    pub fn add_link_attr<T: Into<String>>(&mut self, rel: T, attr: Vec<Attr>) {
        self.links.push(Link::with_attr(rel, attr));
    }

    pub fn add_meta<T: Into<String>, U: Into<String>>(&mut self, key: T, value: U) {
        self.meta.push(Attr::new(key, value));
    }
}

pub struct Page {
    pub header: Header,
    pub body: Node,
}

impl Page {

    pub fn new(header: Header, body: Node) -> Self {
        Page { header: header, body : body }
    }

    pub fn add_events(&mut self) {
        self.body.add_events();
    }
}

