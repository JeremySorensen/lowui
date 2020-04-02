use lowui::builders::*;
use lowui::html;

struct Application {
    ids: Vec<String>,
}

impl lowui::App for Application {
    fn new() -> Application {
        Application {
            ids: Vec::<String>::new(),
        }
    }

    fn init() -> html::Page {
        let head = html::Head::new("TEST");

        let body = body().el(div()
            .id("div")
            .el(button().id("button-1").text("Button 1"))
            .el(br())
            .el(button().id("button-2").text("Button 2"))
            .el(br())
            .el(input().id("number").r#type("number").value("5"))
            .el(br())
            .el(input().id("text").r#type("text").value("hello"))
            .el(br())
            .el(input().id("checkbox").r#type("checkbox"))
            .el(br())
            .el(input()
                .id("radio-1")
                .r#type("radio")
                .name("radio")
                .checked())
            .el(br())
            .el(input().id("radio-2").r#type("radio").name("radio"))
            .el(br())
            .el(select()
                .id("select")
                .el(option().id("one").text("One"))
                .el(option().id("two").text("Two"))));

        html::Page {
            head: head,
            body: body.node(),
        }
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
            let node = p().id(new_id).text(event.id);
            vec![lowui::Command::append_child("div", node.node())]
        }
    }
}

fn main() {
    lowui::start::<Application>();
}
