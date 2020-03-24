#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod html;
pub mod server;

pub trait App {
    fn init() -> html::Page;

    fn update(message: Message) -> Command;
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
    AppendChild { parent_id: String, element: html::Node },
    InsertBefore { sibling_id: String, element: html::Node },
    Update { id: String, element: html::Node },
    Delete { id: String },
}





