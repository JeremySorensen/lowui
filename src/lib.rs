//! LOWUI - A crate for creating an manipulating HTML on the server.
//! An initial UI is represented as Node structs.
//! When the user interacts with the UI, an update function is called
//! Passing in information about the event. The update function
//! returns commands to update the UI.
//! Events and commands are shuttled between the server and client
//! automatically using websockets.
//!
//! This is useful in situations with low latency, such as on a local
//! network and completely eliminates the need to handle distributed
//! state, client/server logic separation and the need to write
//! JavaScript.

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use serde::{Deserialize, Serialize};

pub mod html;
mod server;

pub use self::server::start;

/// Trait representing the user application
pub trait App {
    
    /// Returns an instance, this will be called once for every client connection
    fn new() -> Self;
    
    /// Returns an HTML page, this will be called once before the server starts
    fn init() -> html::Page;
    
    /// Updates the application state and returns commands to update the UI
    /// this is called on every client event
    fn update(&mut self, event: Event) -> Vec<Command>;
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

/// The type of a client event
#[derive(Debug)]
pub enum EventType {
    Click,
    NumberChanged(f64),
    TextChanged(String),
    RadioChecked(String),
    CheckChanged(bool),
}

/// A client event
#[derive(Debug)]
pub struct Event {
    /// The type of the event
    pub event_type: EventType,
    /// The ID of the element that produced the event
    pub id: String,
    /// The type of control that produced the event
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

/// The type of command to execute in the client
/// These represent manipulations of the DOM
#[derive(Serialize, Deserialize)]
pub enum CommandType {
    None,
    AppendChild,
    InsertBefore,
    Update,
    Delete,
}

/// A command to execute on the client
#[derive(Serialize, Deserialize)]
pub struct Command {
    /// The type of command
    pub command_type: CommandType,
    /// The ID of the element affected by the command
    pub id: Option<String>,
    /// The new node for AppendChild, InsertBefore, and Update commands
    pub node: Option<html::Node>
}

impl Command {
    /// Returns a do-nothing command
    pub fn none() -> Command {
        Command { command_type: CommandType::None, id: None, node: None }
    }

    /// Returns a command to append a child Node to an element identified by id
    pub fn append_child<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::AppendChild,
            id: Some(id.into()),
            node: Some(node),
        }
    }

    /// Returns a command to insert a child Node before an element identified by id
    pub fn insert_before<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::InsertBefore,
            id: Some(id.into()),
            node: Some(node),
        }
    }

    /// Returns a command to update an element given by id by replacing it with a new Node
    pub fn update<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::Update,
            id: Some(id.into()),
            node: Some(node),
        }
    }

    /// Returns a command to delete an element given by id
    pub fn delete<T: Into<String>>(id: T) -> Command {
        Command {
            command_type: CommandType::Delete,
            id: Some(id.into()),
            node: None,
        }
    }
}
