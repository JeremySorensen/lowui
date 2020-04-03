use lowui::builders::*;
use lowui::html;
use lowui::html::EventKind;

struct Application {
    ids: Vec<String>,
}

impl lowui::App for Application {
    fn new() -> Application {
        Application {
            ids: Vec::<String>::new(),
        }
    }

    fn init() -> html::HtmlPage {
        html::HtmlPage::new("Hello - lowui").node(
            div()
                .id("div")
                .el(button()
                    .event(EventKind::Onclick)
                    .id("button-1")
                    .text("Button 1"))
                .el(br())
                .el(button()
                    .event(EventKind::Onclick)
                    .id("button-2")
                    .text("Button 2"))
                .el(br())
                .el(input()
                    .event(EventKind::Onchange)
                    .id("number")
                    .r#type("number")
                    .value("5"))
                .el(br())
                .el(input()
                    .event(EventKind::Onchange)
                    .id("text")
                    .r#type("text")
                    .value("hello"))
                .el(br())
                .el(input()
                    .event(EventKind::Onchange)
                    .id("checkbox")
                    .r#type("checkbox"))
                .el(br())
                .el(input()
                    .event(EventKind::Onchange)
                    .id("radio-1")
                    .r#type("radio")
                    .name("radio")
                    .checked())
                .el(br())
                .el(input()
                    .event(EventKind::Onchange)
                    .id("radio-2")
                    .r#type("radio")
                    .name("radio"))
                .el(br())
                .el(select()
                    .event(EventKind::Onchange)
                    .id("select")
                    .el(option().id("one").text("One"))
                    .el(option().id("two").text("Two"))),
        )
    }

    fn update(&mut self, message: lowui::Message) -> Vec<lowui::Command> {
        if message.id == "button-2" {
            if let Some(remove_id) = self.ids.pop() {
                vec![lowui::Command::remove_element(remove_id)]
            } else {
                vec![lowui::Command::none()]
            }
        } else {
            let new_id = self.ids.len().to_string();
            self.ids.push(new_id.clone());
            let node = p().id(new_id).text(message.id);
            vec![lowui::Command::append_child_element("div", node.node())]
        }
    }
}

fn main() {
    lowui::start::<Application>();
}
