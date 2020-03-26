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
pub enum EventType {
    Click,
    NumberChanged(f64),
    TextChanged(String),
    RadioChecked(String),
    CheckChanged(bool),
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub id: String,
    pub r#type: String,
}

impl From<Message> for Event {
    fn from(message: Message) -> Event {
        if message.event == "button-click" {
            Event { event_type: EventType::Click, id: message.id, r#type: message.r#type }
        } else if message.event == "check-changed" {
            if message.r#type == "radio" {
                Event { event_type: EventType::RadioChecked(message.name.unwrap()), id: message.id, r#type: message.r#type }
            } else {
                Event { event_type: EventType::CheckChanged(message.checked.unwrap()), id: message.id, r#type: message.r#type }
            }
        } else {
            if message.r#type == "number" {
                Event { event_type: EventType::NumberChanged(message.value.unwrap().parse().unwrap()), id: message.id, r#type: message.r#type }
            } else {
                Event { event_type: EventType::TextChanged(message.value.unwrap()), id: message.id, r#type: message.r#type }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum CommandType {
    None,
    AppendChild,
    InsertBefore,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub command_type: CommandType,
    pub id: Option<String>,
    pub node: Option<html::Node>
}

impl Command {
    pub fn none() -> Command {
        Command { command_type: CommandType::None, id: None, node: None }
    }

    pub fn append_child<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::AppendChild,
            id: Some(id.into()),
            node: Some(node),
        }
    }

    pub fn insert_before<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::InsertBefore,
            id: Some(id.into()),
            node: Some(node),
        }
    }

    pub fn update<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::Update,
            id: Some(id.into()),
            node: Some(node),
        }
    }

    pub fn delete<T: Into<String>>(id: T) -> Command {
        Command {
            command_type: CommandType::Delete,
            id: Some(id.into()),
            node: None,
        }
    }
}



