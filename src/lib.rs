#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};

pub mod html;
pub mod server;

pub trait App {
    
    fn new() -> Self;
    
    fn init() -> html::Page;
    
    fn update(&mut self, event: Event) -> Command;
}

#[derive(Serialize, Deserialize)]
struct Message {
    pub event: String,
    pub id: String,
    pub r#type: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub checked: Option<bool>,
}

#[derive(Debug)]
pub enum Event {
    Click { id: String },
    NumberChanged { id: String, value: f64 },
    TextChanged { id: String, value: String },
    RadioChecked { id: String, name: String },
    CheckChanged { id: String, checked: bool },
}

impl From<Message> for Event {
    fn from(message: Message) -> Event {
        if message.event == "button-click" {
            Event::Click { id: message.id }
        } else if message.event == "check-changed" {
            if message.r#type == "radio" {
                Event::RadioChecked { id: message.id, name: message.name.unwrap() }
            } else {
                Event::CheckChanged { id: message.id, checked: message.checked.unwrap() }
            }
        } else {
            if message.r#type == "number" {
                Event::NumberChanged { id: message.id, value: message.value.unwrap().parse().unwrap() }
            } else {
                Event::TextChanged { id: message.id, value: message.value.unwrap() }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    None,
    AppendChild { id: String, element: html::Node },
    InsertBefore { id: String, element: html::Node },
    Update { id: String, element: html::Node },
    Delete { id: String },
}





