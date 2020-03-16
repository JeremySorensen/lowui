window.onload = function(e) {
    let app = this.document.getElementById('app')

    const json = '{ "Element": { "tag": "div", "attr": null, "children": [ { "Element": { "tag": "h1", "attr": null, "children": [ { "Text": "Hello Again!" } ] } }, { "Element": { "tag": "p", "attr": [ [ "class", "p-text" ] ], "children": [ { "Text": "This is a simple webpage." } ] } } ] } }';

    const obj = JSON.parse(json);

    this.update('app', obj);
}

function appendChild(id, obj) {
    document.getElementById(id).appendChild(renderHtml(obj));
}

function insertBefore(id, obj) {
    const beforeElement = document.getElementById(id);
    const parent = beforeElement.parentNode;
    parent.insertBefore(renderHtml(obj), beforeElement);
}

function update(id, obj) {
    const element = document.getElementById(id);
    element.replaceWith(renderHtml(obj));
}

function remove(id) {
    const element = document.getElementById(id);
    element.remove();
}

function renderHtml(obj) {
    if (obj.hasOwnProperty('Text')) {
        return document.createTextNode(obj.Text);
    }

    let element = document.createElement(obj.Element.tag);
    if (obj.Element.attr) {
        obj.Element.attr.forEach(a => {
            element.setAttribute(a[0], a[1]);
        });
    }

    obj.Element.children.forEach(c => {
        element.appendChild(renderHtml(c));
    });

    return element;
}