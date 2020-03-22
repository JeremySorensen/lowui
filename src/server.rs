use rocket::response::content::Html;

use std::net::TcpListener;
use std::thread;
use std::thread::spawn;

use tungstenite::accept;
//use tungstenite::handshake::server::{Request, Response};

#[get("/")]
fn index() -> Html<&'static str> {
    Html(
        "
    <html>
    <head>
        <title>WebSocket Echo Client</title>
    </head>
    <body>
        <h1>WebSocket Echo Client</h1>
        <p>
            <button onClick=\"initWebSocket();\">Connect</button>
            <button onClick=\"stopWebSocket();\">Disconnect</button>
            <button onClick=\"checkSocket();\">State</button>
        </p>
        <p>
            <textarea id=\"debugTextArea\" style=\"width:400px;height:200px;\"></textarea>
        </p>
        <p>
            <input type=\text\" id=\"inputText\" onkeydown=\"if(event.keyCode==13)sendMessage();\"/>
            <button onClick=\"sendMessage();\">Send</button>
        </p>

        <script type=\"text/javascript\">
            var debugTextArea = document.getElementById(\"debugTextArea\");
            function debug(message) {
                debugTextArea.value += message + \"\\n\";
                debugTextArea.scrollTop = debugTextArea.scrollHeight;
            }

            function sendMessage() {
                var msg = document.getElementById(\"inputText\").value;
                if ( websocket != null )
                {
                    document.getElementById(\"inputText\").value = \"\";
                    websocket.send( msg );
                    console.log( \"string sent :\", '\"'+msg+'\"' );
                }
            }

            var wsUri = \"ws://localhost:1234\";
            var websocket = null;

            function initWebSocket() {
                try {
                    if (typeof MozWebSocket == 'function')
                        WebSocket = MozWebSocket;
                    if ( websocket && websocket.readyState == 1 )
                        websocket.close();
                    websocket = new WebSocket( wsUri );
                    websocket.onopen = function (evt) {
                        debug(\"CONNECTED\");
                    };
                    websocket.onclose = function (evt) {
                        debug(\"DISCONNECTED\");
                    };
                    websocket.onmessage = function (evt) {
                        console.log( \"Message received :\", evt.data );
                        debug( evt.data );
                    };
                    websocket.onerror = function (evt) {
                        debug('ERROR: ' + evt.data);
                    };
                } catch (exception) {
                    debug('ERROR: ' + exception);
                }
            }

            function stopWebSocket() {
                if (websocket)
                    websocket.close();
            }

            function checkSocket() {
                if (websocket != null) {
                    var stateStr;
                    switch (websocket.readyState) {
                        case 0: {
                            stateStr = \"CONNECTING\";
                            break;
                        }
                        case 1: {
                            stateStr = \"OPEN\";
                            break;
                        }
                        case 2: {
                            stateStr = \"CLOSING\";
                            break;
                        }
                        case 3: {
                            stateStr = \"CLOSED\";
                            break;
                        }
                        default: {
                            stateStr = \"UNKNOW\";
                            break;
                        }
                    }
                    debug(\"WebSocket state = \" + websocket.readyState + \" ( \" + stateStr + \" )\");
                } else {
                    debug(\"WebSocket is null\");
                }
            }
        </script>
    </body>
</html>
    "
  )
}

fn http_init() {
    rocket::ignite().mount("/", routes![index]).launch();
}

pub fn init() {

    thread::spawn(move || {
        http_init();
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
}