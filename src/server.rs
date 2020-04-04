use std::fs;

use rocket::response::content::{ Html, Css };
use rocket::State;

use std::net::TcpListener;
use std::thread;
use std::thread::spawn;

use tungstenite::accept;

#[get("/")]
fn index(html: State<String>) -> Html<String> {
    Html(html.inner().to_string())
}

#[get("/html/<page>")]
fn html(page: String) -> Html<String> {
    let mut path = std::path::PathBuf::from(std::env::current_exe().unwrap().parent().unwrap());
    path.push("static");
    path.push(page);
    Html(fs::read_to_string(path)
        .expect("Something went wrong reading the file"))
}

#[get("/css/<page>")]
fn css(page: String) -> Css<String> {
    let mut path = std::path::PathBuf::from(std::env::current_exe().unwrap().parent().unwrap());
    path.push("css");
    path.push(page);
    Css(fs::read_to_string(path)
        .expect("Something went wrong reading the file"))
}

fn http_init(html: String) {
    rocket::ignite()
        .manage(html)
        .mount("/", routes![index, html, css])
        .launch();
}

/// Starts the server, every request from the client will spawn a new thread
/// with a new instance of the type given as a type parameter
pub fn start<T: crate::App>() {
    let page = T::init();

    let html = page.into_html();

    thread::spawn(move || {
        http_init(html);
    });

    let server = TcpListener::bind("localhost:1234").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut app = T::new();
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                if let Ok(msg) = websocket.read_message() {
                    println!("msg={}", msg);
                    if msg.is_text() {
                        let message: crate::Message =
                            serde_json::from_str(msg.to_text().unwrap()).unwrap();
                        let command = app.update(message);
                        websocket
                            .write_message(tungstenite::Message::Text(
                                serde_json::to_string(&command).unwrap(),
                            ))
                            .unwrap();
                    }
                }
            }
        });
    }
}
