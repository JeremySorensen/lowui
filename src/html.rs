use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Attr {
    pub key: String,
    pub value: Option<String>,
}

impl Attr {
    pub fn new(key: String, value: String) -> Self {
        Attr { key: key, value: Some(value) }
    }

    pub fn with_key_only(key: String) -> Self {
        Attr { key: key, value: None }
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
    pub fn empty(tag: &str) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: None,
            children: Vec::<Node>::new(),
        }
    }

    pub fn with_attr(tag: &str, attr: Vec<Attr>) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: Vec::<Node>::new(),
        }
    }

    pub fn with_child(tag: &str, child: Self) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: None,
            children: vec![child],
        }
    }

    pub fn with_children(tag: &str, children: Vec<Self>) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: None,
            children: children,
        }
    }

    pub fn with_child_attr(tag: &str, attr: Vec<Attr>, child: Self) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: vec![child],
        }
    }

    pub fn with_children_attr(tag: &str, attr: Vec<Attr>, children: Vec<Self>) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: children,
        }
    }

    pub fn with_text(tag: &str, text: String) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: None,
            children: vec![Self::Text(text)],
        }
    }

    pub fn with_text_attr(tag: &str, text: String, attr: Vec<Attr>) -> Self {
        Self::Element {
            tag: tag.to_string(),
            attr: Some(attr),
            children: vec![Self::Text(text)],
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
                            attr_ref.push(Attr::new("onclick".to_string(), "buttonClick()".to_string()));
                        } else if tag == "select" {
                            attr_ref.push(Attr::new("onchange".to_string(), "valueChanged()".to_string()));
                        } else if tag == "input" {
                            if let Some(type_attr) = attr_ref.iter().find(|a| { a.key == "type" }) {
                                let kind = type_attr.value.as_ref().unwrap();
                                if kind == "button" {
                                    attr_ref.push(Attr::new("onclick".to_string(), "buttonClick()".to_string()));
                                } else if kind == "text" || kind == "number" {
                                    attr_ref.push(Attr::new("onchange".to_string(), "valueChanged()".to_string()));
                                } else if kind == "checkbox" || kind == "radio" {
                                    attr_ref.push(Attr::new("onchange".to_string(), "checkChanged()".to_string()));
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
    pub fn new(title: String) -> Self {
        Self {
            title: title,
            links: Vec::<Link>::new(),
            meta: Vec::<Attr>::new(),
        }
    }
    
    pub fn add_link(&mut self, rel: String) {
        self.links.push(Link { rel: rel, attr: None });
    }

    pub fn add_link_attr(&mut self, rel: String, attr: Vec<Attr>) {
        self.links.push(Link { rel: rel, attr: Some(attr) });
    }

    pub fn add_meta(&mut self, key: String, value: String) {
        self.meta.push(Attr::new(key, value));
    }
}

pub struct Page {
    pub header: Header,
    pub body: Node,
}

impl Page {
    pub fn add_events(&mut self) {
        self.body.add_events();
    }
}

