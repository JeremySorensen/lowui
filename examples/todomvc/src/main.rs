use lowui::builders::*;
use lowui::html;
use lowui::html::NodeBuilder;

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

// 	<head>
// 		<meta charset="utf-8">
// 		<meta name="viewport" content="width=device-width, initial-scale=1">
// 		<title>Template • TodoMVC</title>
// 		<link rel="stylesheet" href="node_modules/todomvc-common/base.css">
// 		<link rel="stylesheet" href="node_modules/todomvc-app-css/index.css">
// 		<!-- CSS overrides - remove if you don't need it -->
// 		<link rel="stylesheet" href="css/app.css">
// 	</head>
        let head = html::Head::new("TEST")
        .meta("charset", "utf-8")
        .meta("")

        

// 	<body>
// 		<section class="todoapp">
// 			<header class="header">
// 				<h1>todos</h1>
// 				<input class="new-todo" placeholder="What needs to be done?" autofocus>
// 			</header>
// 			<!-- This section should be hidden by default and shown when there are todos -->
// 			<section class="main">
// 				<input id="toggle-all" class="toggle-all" type="checkbox">
// 				<label for="toggle-all">Mark all as complete</label>
// 				<ul class="todo-list">
// 					<!-- These are here just to show the structure of the list items -->
// 					<!-- List items should get the class `editing` when editing and `completed` when marked as completed -->
// 					<li class="completed">
// 						<div class="view">
// 							<input class="toggle" type="checkbox" checked>
// 							<label>Taste JavaScript</label>
// 							<button class="destroy"></button>
// 						</div>
// 						<input class="edit" value="Create a TodoMVC template">
// 					</li>
// 					<li>
// 						<div class="view">
// 							<input class="toggle" type="checkbox">
// 							<label>Buy a unicorn</label>
// 							<button class="destroy"></button>
// 						</div>
// 						<input class="edit" value="Rule the web">
// 					</li>
// 				</ul>
// 			</section>
// 			<!-- This footer should hidden by default and shown when there are todos -->
// 			<footer class="footer">
// 				<!-- This should be `0 items left` by default -->
// 				<span class="todo-count"><strong>0</strong> item left</span>
// 				<!-- Remove this if you don't implement routing -->
// 				<ul class="filters">
// 					<li>
// 						<a class="selected" href="#/">All</a>
// 					</li>
// 					<li>
// 						<a href="#/active">Active</a>
// 					</li>
// 					<li>
// 						<a href="#/completed">Completed</a>
// 					</li>
// 				</ul>
// 				<!-- Hidden if no completed items are left ↓ -->
// 				<button class="clear-completed">Clear completed</button>
// 			</footer>
// 		</section>
// 		<footer class="info">
// 			<p>Double-click to edit a todo</p>
// 			<!-- Remove the below line ↓ -->
// 			<p>Template by <a href="http://sindresorhus.com">Sindre Sorhus</a></p>
// 			<!-- Change this out with your name and url ↓ -->
// 			<p>Created by <a href="http://todomvc.com">you</a></p>
// 			<p>Part of <a href="http://todomvc.com">TodoMVC</a></p>
// 		</footer>
// 		<!-- Scripts here. Don't remove ↓ -->
// 		<script src="node_modules/todomvc-common/base.js"></script>
// 		<script src="js/app.js"></script>
// 	</body>
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
