use rocket::response::content::Html;
use rocket::State;

use std::net::TcpListener;
use std::thread;
use std::thread::spawn;

use tungstenite::accept;

use crate::html;

#[get("/")]
fn index(html: State<String>) -> Html<String> { Html(html.inner().to_string()) }

fn http_init(html: String) {
    rocket::ignite().manage(html).mount("/", routes![index]).launch();
}

pub fn init(mut page: html::Page) {
    page.add_events();
    let html = make_page(page);

    thread::spawn(move || {
        http_init(html);
    });

    let server = TcpListener::bind("localhost:1234").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                if let Ok(msg) = websocket.read_message() {
                    println!("meg={}", msg);
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(msg).unwrap();
                    }
                }
            }
        });
    }

    fn format_links(links: Vec<html::Link>) -> String {
        links.into_iter().map(|link| -> String {
            if let Some(attr) = link.attr {
                let attr_str = attr.iter().map(|a| { a.to_string() }).collect::<Vec<_>>().join(" ");
                format!("<link rel=\"{}\" {}>", link.rel, attr_str)
            } else {
                format!("<link rel=\"{}\">", link.rel)
            }
        }).collect::<Vec<_>>().join("\n")
    }
    
    fn format_meta(attr: html::Attr) -> String {
        format!("<meta name=\"{}\" content=\"{}\">", attr.key, attr.value.unwrap())
    }
    
    fn make_page(page: html::Page) -> String {
        let links = format_links(page.header.links);
        let meta = page.header.meta.into_iter().map(|m| { format_meta(m) }).collect::<Vec<_>>().join("\n");
        let body = page.body.to_html();
    
        format!("
        <html>
          <head>
            {meta}
            {link}
            <title>{title}</title>
            <script type=\"text/javascript\">
            
            function sendMessage(obj) {{
                if ( websocket != null )
                {{
                    var json = JSON.stringify(obj);
                    websocket.send( json );
                    console.log(\"json sent:\", json);
                }}
            }}

            function receiveMessage(json) {{
                console.log(\"received json\", json);
            }}

            var wsUri = \"ws://localhost:1234\";
            var websocket = null;

            function initWebSocket() {{
                try {{
                    if (typeof MozWebSocket == 'function')
                        WebSocket = MozWebSocket;
                    if ( websocket && websocket.readyState == 1 )
                        websocket.close();
                    websocket = new WebSocket( wsUri );
                    websocket.onopen = function (evt) {{
                        console.log(\"socket opened\");
                    }};
                    websocket.onclose = function (evt) {{
                        console.log(\"socket closed\")
                    }};
                    websocket.onmessage = function (evt) {{
                        console.log(\"json received :\", evt.data);
                        receiveMessage(evt.data);
                    }};
                    websocket.onerror = function (evt) {{
                        console.log(\"socket error:\", evt)
                    }};
                }} catch (exception) {{
                    console.log('EXCEPTION: ' + exception);
                }}
            }}

            function stopWebSocket() {{
                if (websocket) {{
                    websocket.close();
                }}
            }}

            window.onload = initWebSocket;
            window.onclose = stopWebSocket;

            function buttonClick() {{
                sendMessage({{ event: \"button-click\", type: \"button\", id: event.target.id }});
            }}

            function valueChanged() {{
                sendMessage({{ event: \"value-changed\", type: event.target.type, id: event.target.id, value: event.target.value }});
            }}

            function checkChanged() {{
                sendMessage({{ event: \"check-changed\", type: event.target.type, id: event.target.id, checked: event.target.checked }});
            }}
           
        </script>
          </head>
          <body>
            {body}
          </body>
        </html>
        ", meta=meta, link=links, title=page.header.title, body=body)
    }
}