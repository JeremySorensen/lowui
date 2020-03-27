use lowui::html;

fn id(value: &str) -> html::Attr {
    html::Attr::new("id", value)
}

fn vid(value: &str) -> Vec<html::Attr> {
    vec![id(value)]
}

fn input(kind: &str) -> html::Attr {
    html::Attr::new("type", kind)
}

fn value(val: &str) -> html::Attr {
    html::Attr::new("value", val)
}

fn br() -> html::Node {
    html::Node::empty("br")
}

fn with_text_attr(tag: &str, text: &str, attr: Vec<html::Attr>) -> html::Node {
    html::Node::with_child_attr(tag, attr, html::Node::text(text))
}

struct Application {
    ids: Vec<String>,
}

impl lowui::App for Application {

    fn new() -> Application { Application {ids: Vec::<String>::new()} }

    fn init() -> html::Page {
        let head = html::Head::new("TEST");
        let body = html::Node::with_children_attr(
            "div",
            vid("div"),
            vec![
                with_text_attr("button", "Button 1", vid("button-1")),
                br(),
                with_text_attr("button", "Button 2", vid("button-2")),
                br(),
                html::Node::with_attr("input", vec![id("number"), input("number"), value("5")]),
                br(),
                html::Node::with_attr("input", vec![id("text"), input("text"), value("hello")]),
                br(),
                html::Node::with_attr("input", vec![id("checkbox"), input("checkbox")]),
                br(),
                html::Node::with_attr("input", vec![id("radio-1"), input("radio"), html::Attr::new("name", "radio"), html::Attr::with_key_only("checked")]),
                br(),
                html::Node::with_attr("input", vec![id("radio-2"), input("radio"), html::Attr::new("name", "radio")]),
                br(),
                html::Node::with_children_attr("select", vid("select"), vec![
                    with_text_attr("option", "One", vid("one")),
                    with_text_attr("option", "Two", vid("two")),
                ]),
            ]
        );
        html::Page { head: head, body: body }
    }

    fn update(&mut self, event: lowui::Event) -> Vec<lowui::Command> {
        println!("{:?}", event);

        if event.id == "button-2" {
            if let Some(remove_id) = self.ids.pop() {
                vec![lowui::Command::delete(remove_id)]
            } else {
                vec![lowui::Command::none()]
            }
        } else {
            let new_id = self.ids.len().to_string();
            self.ids.push(new_id.clone());
            let node = with_text_attr("p", &event.id, vid(&new_id));
            vec![lowui::Command::append_child(
                "div",
                node
            )]
        }
    }
}

fn main() {
    lowui::start::<Application>();
}
