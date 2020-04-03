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
#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};

pub mod builders;
pub mod html;
mod server;

pub use self::server::start;

/// Trait representing the user application
pub trait App {
    /// Returns an instance, this will be called once for every client connection
    fn new() -> Self;

    /// Returns an HTML page, this will be called once before the server starts
    fn init() -> html::HtmlPage;

    /// Updates the application state and returns commands to update the UI
    /// this is called on every client event
    fn update(&mut self, message: Message) -> Vec<Command>;
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub r#type: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub checked: Option<bool>,
}

/// The type of command to execute in the client
/// These represent manipulations of the DOM
#[derive(Debug, Serialize)]
pub enum CommandType {
    None,
    AppendChildElement(html::Node),
    InsertElementBefore(html::Node),
    ReplaceElement(html::Node),
    RemoveElement,
    SetAttribute(html::Attr),
    RemoveAttribute(&'static str),
}

/// A command to execute on the client
#[derive(Debug, Serialize)]
pub struct Command {
    /// The type of command
    pub command_type: CommandType,
    /// The ID of the element affected by the command
    pub id: Option<String>,
}

impl Command {
    /// Returns a do-nothing command
    pub fn none() -> Command {
        Command {
            command_type: CommandType::None,
            id: None,
        }
    }

    /// Returns a command to append a child Node to an element identified by id
    pub fn append_child_element<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::AppendChildElement(node),
            id: Some(id.into()),
        }
    }

    /// Returns a command to insert a child Node before an element identified by id
    pub fn insert_element_before<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::InsertElementBefore(node),
            id: Some(id.into()),
        }
    }

    /// Returns a command to update an element given by id by replacing it with a new Node
    pub fn replace_element<T: Into<String>>(id: T, node: html::Node) -> Command {
        Command {
            command_type: CommandType::ReplaceElement(node),
            id: Some(id.into()),
        }
    }

    /// Returns a command to delete an element given by id
    pub fn remove_element<T: Into<String>>(id: T) -> Command {
        Command {
            command_type: CommandType::RemoveElement,
            id: Some(id.into()),
        }
    }

    /// Returns a command to set (add or update) an attribute on the element given by id
    pub fn set_attribute<T: Into<String>, U: Into<String>>(
        id: T,
        name: &'static str,
        value: U,
    ) -> Command {
        Command {
            command_type: CommandType::SetAttribute(html::Attr::new(name, value)),
            id: Some(id.into()),
        }
    }

    /// Returns a command to set (add or update) a name-only attribute on the element given by id
    pub fn set_name_only_attribute<T: Into<String>>(id: T, name: &'static str) -> Command {
        Command {
            command_type: CommandType::SetAttribute(html::Attr::name_only(name)),
            id: Some(id.into()),
        }
    }

    /// Returns a command to remove an attribute from the element given by id
    pub fn remove_attribute<T: Into<String>>(id: T, name: &'static str) -> Command {
        Command {
            command_type: CommandType::RemoveAttribute(name),
            id: Some(id.into()),
        }
    }
}
