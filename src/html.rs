use std::fs;

pub struct Link {
    rel: String,
    attr: Option<Vec(String, String)>,
}

pub struct Header {
    title: String,
    links: Vec<Link>,
    meta: Vec<(String, String)>,
}

impl Header {
    pub fn new(title: String) -> Init {
        Init {
            title: title,
            links: Vec::<String>::new(),
            meta: Vec::<(String, String)>::new(),
        }
    }
    
    pub fn add_link(&mut self, rel: String) {
        self.links.push(Link { rel: rel, attr: None });
    }

    pub fn add_link_attr(&mut self, rel: String, attr: Vec(String, String)) {
        self.links.push(Link { rel: rel, attr: Some(attr) });
    }

    pub fn add_meta(&mut self, key: String, value: String) {
        self.meta.push((key, value));
    }
}

pub struct Page {
    header: Header,
    body: VDom,
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
    AppendChild { parent_id: String, element: VDom },
    InsertBefore { sibling_id: String, element: VDom },
    Update { id: String, element: VDom },
    Delete { id: String },
}

pub trait App {
    fn init() -> Page;

    fn update(message: Message) -> Command;
}

fn make_links(links: Vec<String>) -> String {
    links.iter().map(|link| -> {
        if let Some(attr) = link.attr {
            let attr_str = attr.map(|a| -> format!("{}=\"{}\"")).join(" ");
            format!("<link rel=\"{}\" {}>", link.rel, attr_str)
        } else {
            format!("<link rel=\"{}\">", link.rel)
        }
    }).join("\n")
}

fn make_meta(meta: Vec(String, String)) -> String {
    meta.iter().map(|m| -> format!("<meta name=\"{}\" content=\"{}\">")).join("\n")
}

fn make_page(page: Page) -> String {
    let mut template = include_str("../index.html.template").to_string();
    let links = make_links(page.header.links);
    let meta = make_meta(page.header.meta);
    let body = page.body.to_html();

    format!("
    <html>
      <head>
        {meta}
        {link}
        <title>{title}</title>
        <script src=\index.js\"></script>
      </head>
      <body>
        {body}
      </body>
    </html>
    ", meta=meta, link=link, title=page.header.title, body=body)
}

