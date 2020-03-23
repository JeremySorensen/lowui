use std::fs;
use crate::vdom;

pub struct Link {
    rel: String,
    attr: Option<Vec<vdom::Attr>>,
}

pub struct Header {
    title: String,
    links: Vec<Link>,
    meta: Vec<vdom::Attr>,
}

impl Header {
    pub fn new(title: String) -> Self {
        Self {
            title: title,
            links: Vec::<Link>::new(),
            meta: Vec::<vdom::Attr>::new(),
        }
    }
    
    pub fn add_link(&mut self, rel: String) {
        self.links.push(Link { rel: rel, attr: None });
    }

    pub fn add_link_attr(&mut self, rel: String, attr: Vec<vdom::Attr>) {
        self.links.push(Link { rel: rel, attr: Some(attr) });
    }

    pub fn add_meta(&mut self, key: String, value: String) {
        self.meta.push(vdom::Attr::new(key, value));
    }
}

pub struct Page {
    header: Header,
    body: vdom::VDom,
}

pub enum Message {
    Click,
    SelectChanged(String),
    CheckChanged(bool),
    TextChanged(String),
    TextChangeFinished(String),
}

pub enum Command {
    None,
    AppendChild { parent_id: String, element: vdom::VDom },
    InsertBefore { sibling_id: String, element: vdom::VDom },
    Update { id: String, element: vdom::VDom },
    Delete { id: String },
}

pub trait App {
    fn init() -> Page;

    fn update(message: Message) -> Command;
}

fn make_links(links: Vec<Link>) -> String {
    links.into_iter().map(|link| -> String {
        if let Some(attr) = link.attr {
            let attr_str = attr.iter().map(|a| { a.to_string() }).collect::<Vec<_>>().join(" ");
            format!("<link rel=\"{}\" {}>", link.rel, attr_str)
        } else {
            format!("<link rel=\"{}\">", link.rel)
        }
    }).collect::<Vec<_>>().join("\n")
}

fn make_meta(attr: vdom::Attr) -> String {
    format!("<meta name=\"{}\" content=\"{}\">", attr.key, attr.value.unwrap())
}

fn make_page(page: Page) -> String {
    let links = make_links(page.header.links);
    let meta = page.header.meta.into_iter().map(|m| { make_meta(m) }).collect::<Vec<_>>().join("\n");
    let body = page.body.to_html();

    format!("
    <html>
      <head>
        {meta}
        {link}
        <title>{title}</title>
        <script src=\"index.js\"></script>
      </head>
      <body>
        {body}
      </body>
    </html>
    ", meta=meta, link=links, title=page.header.title, body=body)
}

