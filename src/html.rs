use std::fmt;
use serde::{Deserialize, Serialize};

/// An HTML attribute
#[derive(Serialize, Deserialize)]
pub struct Attr {
    pub key: String,
    pub value: Option<String>,
}

impl Attr {
    /// Returns a new Attr with key and value
    pub fn new<T: Into<String>, U: Into<String>>(key: T, value: U) -> Self {
        Attr { key: key.into(), value: Some(value.into()) }
    }

    /// Returns a new Attr with no value
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

/// A &lt;link rel="..."&gt; element
pub struct Link {
    /// The rel attribute
    pub rel: String,
    /// Other option attributes
    pub attr: Option<Vec<Attr>>,
}

impl Link {
    /// Returns a new Link with no attributes except rel
    pub fn new<T: Into<String>>(rel: T) -> Link {
        Link { rel: rel.into(), attr: None }
    }

    /// Returns a new Link with addtional attributes
    pub fn with_attr<T: Into<String>>(rel: T, attr: Vec<Attr>) -> Link {
        Link { rel: rel.into(), attr: Some(attr) }
    }
}

/// An HTML element or text node
#[derive(Serialize, Deserialize)]
pub enum Node {
    Element {
        /// The element type, or tag
        tag: String,
        /// Any attributes of the element
        attr: Option<Vec<Attr>>,
        /// Any children of the element
        children: Option<Vec<Node>>,
    },
    Text(String),
}

impl Node {
    /// Returns an element with no children and no attributes
    pub fn empty<T: Into<String>>(tag: T) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: None,
        }
    }

    /// Returns an element with no children and one or more attributes
    pub fn with_attr<T: Into<String>>(tag: T, attr: Vec<Attr>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: None,
        }
    }

    /// Returns an element with a single child and no attributes
    pub fn with_child<T: Into<String>>(tag: T, child: Self) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: Some(vec![child]),
        }
    }

    /// Returns an element with multiple children and no attributes
    pub fn with_children<T: Into<String>>(tag: T, children: Vec<Self>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: None,
            children: Some(children),
        }
    }

    /// Returns an element with one child and one or more attributes
    pub fn with_child_attr<T: Into<String>>(tag: T, attr: Vec<Attr>, child: Self) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: Some(vec![child]),
        }
    }

    /// Returns an element with multiple children and one or more attributes
    pub fn with_children_attr<T: Into<String>>(tag: T, attr: Vec<Attr>, children: Vec<Self>) -> Self {
        Self::Element {
            tag: tag.into(),
            attr: Some(attr),
            children: Some(children),
        }
    }

    /// Returns a text node
    pub fn text<T: Into<String>>(text: T) -> Self {
        Self::Text(text.into())
    }
 
    pub (crate) fn to_html(&self) -> String {
        match self {
            Self::Element {
                tag,
                attr,
                children,
            } => {

                let children = if let Some(ch) = children {
                    children_to_html(&ch)
                } else {
                    "".to_string()
                };

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

    fn add_events(&mut self) {
        match self {
            Self::Text(_) => return,
            Self::Element{ tag, attr, children } => {
                if let Some(ch) = children {
                    for child in ch {
                        child.add_events();
                    }
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
    pub fn add_link<T: Into<String>>(&mut self, rel: T) {
        self.links.push(Link::new(rel));
    }

    /// Adds a link (&lt;link&gt; element) with one or more attributes
    pub fn add_link_attr<T: Into<String>>(&mut self, rel: T, attr: Vec<Attr>) {
        self.links.push(Link::with_attr(rel, attr));
    }

    /// Adds a &lt;meta&gt; element
    pub fn add_meta<T: Into<String>, U: Into<String>>(&mut self, name: T, content: U) {
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
        Page { head: head, body : body }
    }

    pub (crate) fn add_events(&mut self) {
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
        .map(|a| { a.to_string() })
        .collect::<Vec<_>>()
        .join(" ")
}

