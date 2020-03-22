# lowui
Web UI for devices on local networks, code like it is running local, page updates using websockets.

For IoT or network connected embedded devices, it is handy to be able to serve a web page that users can use for configuration
or other interaction. But often these devices are on a local network or can be expected to have a solid connection. In these
situations, rather than putting a bunch of complicated app logic in javascript and serving it up. It could be easier to just
act like the browser is part of the app running on the server. You generate some HTML and it ends up on the client browser,
the user clicks a button or enters text, and your code is run, with the ID of the widget and information about the event. You
make changes to the HTML, and the client is automatically updated. You never have to write a line of JavaScript.

The magic is a simplified RPC (remote procedure call) based on JSON and websockets. The initial page is loaded with a stock
JavaScript file that sets up the communication, as well as server side rendered HTML. API functions dispatch JSON commands
that cause the elements to be added, updated, or removed as required.

The server code is part of the library and doen't need to be dealt with. Eventually parts of this might be exposed to allow
things like routing.

lowui-rs is written in Rust. Initially it will be required to use Rust to consume the API. Eventually it would be nice to put
a C interface on it as well as bindings for other languages.

It would also be nice to switch from JSON to some smaller faster binary format for communication between the server and
browser.

The best would be to develop a small virtual-DOM like framework to enable a declarative API for the HTML.


