use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub struct HtmlPage {
    pub title: String,
    pub metas: Vec<Meta>,
    pub links: Vec<Link>,
    pub nodes: Vec<Node>,
}

impl HtmlPage {
    pub fn new<T: Into<String>>(title: T) -> HtmlPage {
        HtmlPage {
            title: title.into(),
            metas: Vec::<Meta>::new(),
            links: Vec::<Link>::new(),
            nodes: Vec::<Node>::new(),
        }
    }

    pub fn meta(mut self, meta: Meta) -> Self {
        self.metas.push(meta);
        self
    }

    pub fn meta_content<T: Into<String>, U: Into<String>>(mut self, name: T, content: U) -> Self {
        self.metas.push(Meta::new().name(name).content(content));
        self
    }

    pub fn link(mut self, link: Link) -> Self {
        self.links.push(link);
        self
    }

    pub fn node(mut self, node: impl crate::builders::NodeBuilder) -> Self {
        self.nodes.push(node.node());
        self
    }

    pub(crate) fn into_html(self) -> String {
        let links = join_as_strings(self.links, "\n", |link| {
            let attrs = join_as_strings(link.attrs, " ", |attr| attr.to_string());
            format!("<link {}>", attrs)
        });

        let metas = join_as_strings(self.metas, "\n", |meta| -> String {
            let attrs = join_as_strings(meta.attrs, " ", |attr| attr.to_string());
            format!("<meta {}>", attrs)
        });

        let nodes = join_as_strings(self.nodes, "\n", |node| -> String { node.into_html() });

        format!("
        <html>
          <head>
            {metas}
            {links}
            <title>{title}</title>
            <script type=\"text/javascript\">
            
            function sendMessage(obj) {{
                if ( websocket != null )
                {{
                    var json = JSON.stringify(obj);
                    websocket.send( json );
                }}
            }}

            function receiveMessage(json) {{
                var objs = JSON.parse(json);
                objs.forEach((obj)=> {{
                    if (obj.id) {{
                        let command = obj.command_type;
                        let element = document.getElementById(obj.id);
                        if (command === 'RemoveElement') {{
                            element.remove();
                        }}
                        else if (command.hasOwnProperty('AppendChildElement')) {{
                            let newEl = makeElement(command.AppendChildElement);
                            element.appendChild(newEl);
                        }}
                        else if (command.hasOwnProperty('InsertElementBefore')) {{
                            let newEl = makeElement(command.InsertElementBefore);
                            document.insertBefore(element, newEl);
                        }}
                        else if (command.hasOwnProperty('ReplaceElement')) {{
                            let newEl = makeElement(command.ReplaceElement);
                            element.parentNode.replaceChild(newEl, element);
                        }}
                        else if (command.hasOwnProperty('SetAttribute')) {{
                            element.setAttribute(command.SetAttribute.name, command.SetAttribute.value);
                        }}
                        else if (command.hasOwnProperty('SetAttribute')) {{
                            if (command.SetAttribute.hasOwnProperty('value')) {{
                                element.setAttribute(command.SetAttribute.name, command.SetAttribute.value);
                            }} else {{
                                element.setAttribute(command.SetAttribute.name);
                            }}
                        }}
                        else if (command.hasOwnProperty('RemoveAttribute')) {{
                            element.removeAttribute(command.RemoveAttribute.name);
                        }}
                    }}
                }});
            }}

            function makeElement(obj) {{
                if (obj.Text) {{
                    return document.createTextNode(obj.Text);
                }}
                
                const element = obj.Element;
                let newEl = window.document.createElement(element.tag);
                element.attrs && element.attrs.forEach((a) => newEl.setAttribute(a.name, a.value));
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
                    }};
                    websocket.onclose = function (evt) {{
                    }};
                    websocket.onmessage = function (evt) {{
                        receiveMessage(evt.data);
                    }};
                    websocket.onerror = function (evt) {{
                    }};
                }} catch (exception) {{
                }}
            }}

            function stopWebSocket() {{
                if (websocket) {{
                    websocket.close();
                }}
            }}

            window.onload = initWebSocket;
            window.onclose = stopWebSocket;

            function onevent() {{
                if (event.target.type === 'radio' || event.target.type === 'checkbox') {{
                    sendMessage({{
                        type: event.target.type,
                        id: event.target.id,
                        name: event.target.nodeName,
                        checked: event.target.checked 
                    }});
                }} else if (event.target.value !== '') {{
                    sendMessage({{
                        type: event.target.type,
                        id: event.target.id,
                        name: event.target.nodeName,
                        value: event.target.value 
                    }});
                }} else {{
                    sendMessage({{
                        type: event.target.type,
                        id: event.target.id,
                        name: event.target.nodeName
                    }});
                }}
            }}
        </script>
          </head>
          <body>
            {nodes}
          </body>
        </html>
        ", metas=metas, links=links, title=self.title, nodes=nodes)
    }
}

/// An HTML attribute
#[derive(Debug, Serialize)]
pub struct Attr {
    pub name: &'static str,
    pub value: Option<String>,
}

impl Attr {
    /// Returns a new Attr with key and value
    pub fn new<T: Into<String>>(name: &'static str, value: T) -> Self {
        Self {
            name: name,
            value: Some(value.into()),
        }
    }

    /// Returns a new Attr with no value
    pub fn name_only(name: &'static str) -> Self {
        Self {
            name: name,
            value: None,
        }
    }
}

impl fmt::Display for Attr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = &self.value {
            write!(f, r#"{}="{}""#, self.name, value)
        } else {
            write!(f, r#"{}"#, self.name)
        }
    }
}

#[derive(Debug)]
pub struct Meta {
    pub attrs: Vec<Attr>,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            attrs: Vec::<Attr>::new(),
        }
    }

    pub fn charset<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("charset", value));
        self
    }

    pub fn content<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("content", value));
        self
    }

    pub fn http_equiv<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("http-equiv", value));
        self
    }

    pub fn name<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("name", value));
        self
    }
}

/// A &lt;link rel="..."&gt; element
#[derive(Debug)]
pub struct Link {
    /// Attributes
    pub attrs: Vec<Attr>,
}

impl Link {
    pub fn new<T: Into<String>>(rel: T) -> Self {
        Self {
            attrs: vec![Attr::new("rel", rel)],
        }
    }

    pub fn crossorigin<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("crossorigin", value));
        self
    }

    pub fn href<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("href", value));
        self
    }

    pub fn hreflang<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("hreflang", value));
        self
    }

    pub fn media<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("media", value));
        self
    }

    pub fn nonce<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("nonce", value));
        self
    }

    pub fn referrerpolicy<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("referrerpolicy", value));
        self
    }

    pub fn sizes<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("sizes", value));
        self
    }

    pub fn r#type<T: Into<String>>(mut self, value: T) -> Self {
        self.attrs.push(Attr::new("type", value));
        self
    }
}

/// An HTML element or text node
#[derive(Debug, Serialize)]
pub enum Node {
    Element {
        /// The element type, or tag
        tag: &'static str,
        /// Any attributes of the element
        attrs: Vec<Attr>,
        /// Any children of the element
        children: Vec<Node>,
        /// True if element is empty like &lt;br&gt;
        is_empty: bool,
    },
    Text(String),
}

impl Node {
    pub(crate) fn new_el(
        tag: &'static str,
        events: Vec<EventKind>,
        mut attrs: Vec<Attr>,
        children: Vec<Node>,
        is_empty: bool,
    ) -> Self {
        for event_kind in events {
            attrs.push(Attr::new(event_kind.to_str(), "onevent()"));
        }

        Self::Element {
            tag: tag,
            attrs: attrs,
            children: children,
            is_empty: is_empty,
        }
    }

    pub(crate) fn new_text<T: Into<String>>(text: T) -> Self {
        Node::Text(text.into())
    }

    pub(crate) fn into_html(self) -> String {
        match self {
            Self::Element {
                tag,
                attrs,
                children,
                is_empty,
            } => {
                let space;
                let attr_txt;
                if attrs.is_empty() {
                    space = "".to_string();
                    attr_txt = "".to_string();
                } else {
                    space = " ".to_string();
                    attr_txt = join_as_strings(attrs, " ", |attr| attr.to_string())
                }

                if is_empty {
                    format!("<{t}{s}{a}>", t = tag, s = space, a = attr_txt,)
                } else {
                    let children = if children.is_empty() {
                        "".to_string()
                    } else {
                        join_as_strings(children, "", |child| child.into_html())
                    };

                    format!(
                        "<{t}{s}{a}>{c}</{t}>",
                        t = tag,
                        s = space,
                        a = attr_txt,
                        c = children
                    )
                }
            }

            Self::Text(t) => t.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum EventKind {
    Onabort,
    Onauxclick,
    Oncancel,
    Oncanplay,
    Oncanplaythrough,
    Onchange,
    Onclick,
    Onclose,
    Oncuechange,
    Ondblclick,
    Ondrag,
    Ondragend,
    Ondragenter,
    Ondragexit,
    Ondragleave,
    Ondragover,
    Ondragstart,
    Ondrop,
    Ondurationchange,
    Onemptied,
    Onended,
    Oninput,
    Oninvalid,
    Onkeydown,
    Onkeypress,
    Onkeyup,
    Onloadeddata,
    Onloadedmetadata,
    Onloadend,
    Onloadstart,
    Onmousedown,
    Onmouseenter,
    Onmouseleave,
    Onmousemove,
    Onmouseout,
    Onmouseover,
    Onmouseup,
    Onwheel,
    Onpause,
    Onplay,
    Onplaying,
    Onprogress,
    Onratechange,
    Onreset,
    Onseeked,
    Onseeking,
    Onselect,
    Onshow,
    Onstalled,
    Onsubmit,
    Onsuspend,
    Ontimeupdate,
    Ontoggle,
    Onvolumechange,
    Onwaiting,
    Onblur,
    Onerror,
    Onfocus,
    Onload,
    Onresize,
    Onscroll,
}

impl EventKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            EventKind::Onabort => "onabort",
            EventKind::Onauxclick => "onauxclick",
            EventKind::Oncancel => "oncancel",
            EventKind::Oncanplay => "oncanplay",
            EventKind::Oncanplaythrough => "oncanplaythrough",
            EventKind::Onchange => "onchange",
            EventKind::Onclick => "onclick",
            EventKind::Onclose => "onclose",
            EventKind::Oncuechange => "oncuechange",
            EventKind::Ondblclick => "ondblclick",
            EventKind::Ondrag => "ondrag",
            EventKind::Ondragend => "ondragend",
            EventKind::Ondragenter => "ondragenter",
            EventKind::Ondragexit => "ondragexit",
            EventKind::Ondragleave => "ondragleave",
            EventKind::Ondragover => "ondragover",
            EventKind::Ondragstart => "ondragstart",
            EventKind::Ondrop => "ondrop",
            EventKind::Ondurationchange => "ondurationchange",
            EventKind::Onemptied => "onemptied",
            EventKind::Onended => "onended",
            EventKind::Oninput => "oninput",
            EventKind::Oninvalid => "oninvalid",
            EventKind::Onkeydown => "onkeydown",
            EventKind::Onkeypress => "onkeypress",
            EventKind::Onkeyup => "onkeyup",
            EventKind::Onloadeddata => "onloadeddata",
            EventKind::Onloadedmetadata => "onloadedmetadata",
            EventKind::Onloadend => "onloadend",
            EventKind::Onloadstart => "onloadstart",
            EventKind::Onmousedown => "onmousedown",
            EventKind::Onmouseenter => "onmouseenter",
            EventKind::Onmouseleave => "onmouseleave",
            EventKind::Onmousemove => "onmousemove",
            EventKind::Onmouseout => "onmouseout",
            EventKind::Onmouseover => "onmouseover",
            EventKind::Onmouseup => "onmouseup",
            EventKind::Onwheel => "onwheel",
            EventKind::Onpause => "onpause",
            EventKind::Onplay => "onplay",
            EventKind::Onplaying => "onplaying",
            EventKind::Onprogress => "onprogress",
            EventKind::Onratechange => "onratechange",
            EventKind::Onreset => "onreset",
            EventKind::Onseeked => "onseeked",
            EventKind::Onseeking => "onseeking",
            EventKind::Onselect => "onselect",
            EventKind::Onshow => "onshow",
            EventKind::Onstalled => "onstalled",
            EventKind::Onsubmit => "onsubmit",
            EventKind::Onsuspend => "onsuspend",
            EventKind::Ontimeupdate => "ontimeupdate",
            EventKind::Ontoggle => "ontoggle",
            EventKind::Onvolumechange => "onvolumechange",
            EventKind::Onwaiting => "onwaiting",
            EventKind::Onblur => "onblur",
            EventKind::Onerror => "onerror",
            EventKind::Onfocus => "onfocus",
            EventKind::Onload => "onload",
            EventKind::Onresize => "onresize",
            EventKind::Onscroll => "onscroll",
        }
    }
}

fn join_as_strings<T, F>(items: Vec<T>, delim: &'static str, fun: F) -> String
where
    F: FnMut(T) -> String,
{
    items
        .into_iter()
        .map(fun)
        .collect::<Vec<String>>()
        .join(delim)
}
