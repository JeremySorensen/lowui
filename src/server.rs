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

/// Starts the server, every request from the client will spawn a new thread
/// with a new instance of the type given as a type parameter
pub fn start<T: crate::App>() {

    let mut page = T::init();
    page.add_events();
    let html = format_page(page);

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
                        let message: crate::Message = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                        let command = app.update(message.into());
                        websocket.write_message(tungstenite::Message::Text(serde_json::to_string(&command).unwrap())).unwrap();
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
    
    fn format_page(page: html::Page) -> String {
        let links = format_links(page.head.links);
        let meta = page.head.meta.into_iter().map(|m| { format_meta(m) }).collect::<Vec<_>>().join("\n");
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
                var obj = JSON.parse(json);
                console.log(\"OBJ\", obj);
                if (obj.id) {{
                    let element = document.getElementById(obj.id);
                    switch (obj.command_type) {{
                        case \"Delete\": {{
                            element.remove();
                            break;
                        }}
                        case \"AppendChild\": {{
                            let newEl = makeElement(obj.node);
                            element.appendChild(newEl);
                            break;
                        }}
                        case \"InsertBefore\": {{
                            let newEl = makeElement(obj.node);
                            document.insertBefore(element, newEl);
                            break;
                        }}
                        case \"Update\": {{
                            let newEl = makeElement(obj.node);
                            element.parentNode.replaceChild(newEl, element);
                            break;
                        }}
                    }}
                }}
            }}

            function makeElement(obj) {{
                if (obj.Text) {{
                    return document.createTextNode(obj.Text);
                }}
                
                const element = obj.Element;
                let newEl = window.document.createElement(element.tag);
                element.attr && element.attr.forEach((a) => newEl.setAttribute(a.key, a.value));
                element.children && element.children.forEach((c) => newEl.appendChild(makeElement(c)));
                return newEl;
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
                sendMessage({{ event: \"check-changed\", type: event.target.type, id: event.target.id, checked: event.target.checked, name: event.target.name }});
            }}
           
        </script>
          </head>
          <body>
            {body}
          </body>
        </html>
        ", meta=meta, link=links, title=page.head.title, body=body)
    }
}