use lowui::server;
use lowui::html;

fn id(value: &str) -> html::Attr {
    html::Attr::new("id".to_string(), value.to_string())
}

fn vid(value: &str) -> Vec<html::Attr> {
    vec![id(value)]
}

fn input(kind: &str) -> html::Attr {
    html::Attr::new("type".to_string(), kind.to_string())
}

fn value(val: &str) -> html::Attr {
    html::Attr::new("value".to_string(), val.to_string())
}

fn br() -> html::Node {
    html::Node::empty("br")
}

struct Application {
    ids: Vec<String>,
 }

impl lowui::App for Application {

    fn new() -> Application { Application {ids: Vec::<String>::new()} }

    fn init() -> html::Page {
        let header = html::Header::new("TEST".to_string());
        let body = html::Node::with_children_attr(
            "div",
            vid("div"),
            vec![
                html::Node::with_text_attr("button", "Button 1".to_string(), vid("button-1")),
                br(),
                html::Node::with_text_attr("button", "Button 2".to_string(), vid("button-2")),
                br(),
                html::Node::with_attr("input", vec![id("number"), input("number"), value("5")]),
                br(),
                html::Node::with_attr("input", vec![id("text"), input("text"), value("hello")]),
                br(),
                html::Node::with_attr("input", vec![id("checkbox"), input("checkbox")]),
                br(),
                html::Node::with_attr("input", vec![id("radio-1"), input("radio"), html::Attr::new("name".to_string(), "radio".to_string()), html::Attr::with_key_only("checked".to_string())]),
                br(),
                html::Node::with_attr("input", vec![id("radio-2"), input("radio"), html::Attr::new("name".to_string(), "radio".to_string())]),
                br(),
                html::Node::with_children_attr("select", vid("select"), vec![
                    html::Node::with_text_attr("option", "One".to_string(), vid("one")),
                    html::Node::with_text_attr("option", "Two".to_string(), vid("two")),
                ]),
            ]
        );
        html::Page { header: header, body: body }
    }

    fn update(&mut self, event: lowui::Event) -> lowui::Command {
        println!("{:?}", event);

        let id = match event {
            lowui::Event::Click { id } => id,
            lowui::Event::NumberChanged { id, .. } => id,
            lowui::Event::TextChanged { id, .. } => id,
            lowui::Event::RadioChecked { id, .. } => id,
            lowui::Event::CheckChanged { id, .. } => id,
        };


        if id == "button-2" {
            if let Some(remove_id) = self.ids.pop() {
                lowui::Command::Delete { id: remove_id }
            } else {
                lowui::Command::None
            }
        } else {
            let new_id = self.ids.len().to_string();
            self.ids.push(new_id.clone());
            lowui::Command::AppendChild { id: "div".to_string(), element: html::Node::with_text_attr("p", id, vid(&new_id)) }
        }
    }
}

fn main() {
    server::start::<Application>();
}
