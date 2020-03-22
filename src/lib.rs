#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod vdom;
pub mod server;

use vdom::VDom;

pub struct Init {
    title: String,
    links: Vec<String>,
    meta: Vec<(String, String)>,
}

impl Init {
    pub fn new(title: String) -> Init {
        Init {
            title: title,
            links: Vec::<String>::new(),
            meta: Vec::<(String, String)>::new(),
        }
    }
    
    pub fn add_link(&mut self, link: String) {
        self.links.push(link);
    }

    pub fn add_meta(&mut self, key: String, value: String) {
        self.meta.push((key, value));
    }
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
    fn init() -> Init;

    fn update(message: Message) -> Command;
}
