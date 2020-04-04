use lowui::builders::*;
use lowui::html;
use lowui::html::EventKind;

struct Application {}

impl Application {
    fn stylesheet<T: Into<String>>(url: T) -> html::Link {
        html::Link::new("stylesheet").href(url)
    }
}

impl lowui::App for Application {
    fn new() -> Application {
        Application {}
    }

    fn init() -> html::HtmlPage {
        html::HtmlPage::new("TodoMVC - lowui")
            .meta(html::Meta::new().charset("utf-8"))
            .meta_content("viewport", "width=device-width, initial-scale=1")
            .link(Self::stylesheet("css/index.css"))
            .node(
                section()
                    .class("todoapp")
                    .el(header().class("header").el(h1().text("todos")))
                    .el(input()
                        .class("new-todo")
                        .placeholder("What needs to be done?")
                        .autofocus()),
            )
            .node(
                footer()
                    .class("info")
                    .el(p().text("Double-click to edit a todo"))
                    .el(p().text("Created by ").el(a()
                        .href("http://discovercoding.com")
                        .text("Jeremy Sorensen")))
                    .el(p()
                        .text("NOT an offical part of ")
                        .el(a().href("http://todomvc.com").text("TodoMVC"))),
            )
    }

    fn update(&mut self, event: lowui::Message) -> Vec<lowui::Command> {
        // println!("{:?}", event);

        // if event.id == "button-2" {
        //     if let Some(remove_id) = self.ids.pop() {
        //         vec![lowui::Command::delete(remove_id)]
        //     } else {
        //         vec![lowui::Command::none()]
        //     }
        // } else {
        //     let new_id = self.ids.len().to_string();
        //     self.ids.push(new_id.clone());
        //     let node = p().id(new_id).text(event.id);
        //     vec![lowui::Command::append_child("div", node.node())]
        // }

        vec![lowui::Command::none()]
    }
}

fn main() {
    lowui::start::<Application>();
}
