import boolean_attrs
import empty_tags;
import json

bool_attr_set = frozenset(boolean_attrs.boolean_attrs)
empty_tag_set = frozenset(empty_tags.empty_tags)

with open("elements.json") as file:
    elements = json.load(file)
    

TEMPLATE = """
use crate::html::{ Attr, Node };
$BUILDER_IMPORT$
use crate::builders::set_attr::SetAttr;

#[derive(Debug)]
pub struct $BUILDER_NAME$ {
    attr: Vec<Attr>,
    $CHILD_VEC_DECL$
}

impl $BUILDER_NAME$ {
    pub fn new() -> Self {
        Self { 
            attr: Vec::<Attr>::new(),
            $CHILD_VEC_INST$
        }
    }

    $ATTR_FUNS$

    $CHILD_FUNS$
}

impl NodeBuilder for $BUILDER_NAME$ {
    fn node(self) -> Node {
        Node::new_el("$TAG_NAME$", self.attr, $CHILDREN$, $IS_EMPTY$)
    }
}

impl SetAttr for $BUILDER_NAME$ {
    fn set_attr<T: Into<String>>(mut self, name: &'static str, value: T) -> Self {
        self.attr.push(Attr::new(name, value));
        self
    }
}

"""

BUILDER_IMPORT = ["""
use crate::builders::NodeBuilder;
""",
"""
use crate::builders::{ NodeBuilder, GlobalAttr };
""",
"""
use crate::builders::{ NodeBuilder, EventAttr };
""",
"""
use crate::builders::{ NodeBuilder, EventAttr, GlobalAttr };
""",
]

NO_CHILD_VEC = "Vec::<impl NodeBuilder>::new()"

ATTR_VAL_FUN = """
    pub fn $ATTR_NAME_SAFE$<T: Into<String>>(mut self, value: T) -> Self {
        self.attr.push(Attr::new("$ATTR_NAME$", value));
        self
    }
"""

ATTR_NO_VAL_FUN = """
    pub fn $ATTR_NAME_SAFE$(mut self) -> Self {
        self.attr.push(Attr::name_only("$ATTR_NAME$"));
        self
    }
"""

CHILD_FUNS = """
    pub fn el(mut self, node: impl NodeBuilder) -> Self {
        self.children.push(node.node());
        self
    }

    pub fn els(mut self, nodes: Vec<impl NodeBuilder>) -> Self {
        for node in nodes {
            self.children.push(node.node());
        }
        self
    }

    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.children.push(Node::new_text(text));
        self
    }
"""

EVENT_ATTR_TRAIT = """
impl EventAttr for $BUILDER_NAME$ { }
"""

GLOBAL_ATTR_TRAIT = """
impl GlobalAttr for $BUILDER_NAME$ { }
"""

CREATE_FUN = """
pub fn $TAG_NAME$() -> $BUILDER_NAME$ {
    $BUILDER_NAME$::new()
}
"""

def fix_name(name):
    if name == 'type':
        return 'r#type'
    elif name == 'loop':
        return 'r#loop'
    elif name == 'for':
        return 'r#for'
    elif name == 'async':
        return 'r#async'
    else:
        return name.replace("-", "_")

def make_attr_fun(name):
    name_safe = fix_name(name)
    if name in bool_attr_set:
        return ATTR_NO_VAL_FUN.replace("$ATTR_NAME$", name).replace("$ATTR_NAME_SAFE$", name_safe)
    else:
        return ATTR_VAL_FUN.replace("$ATTR_NAME$", name, 2).replace("$ATTR_NAME_SAFE$", name_safe)

def make_attr_funs(names):
    return '\n'.join(list(map(make_attr_fun, names)))

def make_element_code(element):
    tag = element['tag']
    builder = tag.capitalize()
    attr_funs = make_attr_funs(element['attr'])
    if tag in empty_tag_set:
        child_vec_decl = ""
        child_vec_inst = ""
        child_funs = ""
        children = "Vec::<Node>::new()"
        is_empty = "true"
    else:
        child_vec_decl = "children: Vec<Node>,"
        child_vec_inst = "children: Vec::<Node>::new(),"
        child_funs = CHILD_FUNS
        children = "self.children"
        is_empty = "false"

    result = TEMPLATE[:]

    builder_import_index = 0

    if element['has_events']:
        result += EVENT_ATTR_TRAIT
        builder_import_index |= 2

    if element['has_global']:
        result += GLOBAL_ATTR_TRAIT
        builder_import_index |= 1

    builder_import = BUILDER_IMPORT[builder_import_index]

    result += CREATE_FUN

    result = result \
    .replace("$BUILDER_IMPORT$", builder_import) \
    .replace("$BUILDER_NAME$", builder) \
    .replace("$TAG_NAME$", tag) \
    .replace("$CHILD_VEC_DECL$", child_vec_decl) \
    .replace("$CHILD_VEC_INST$", child_vec_inst) \
    .replace("$ATTR_FUNS$", attr_funs) \
    .replace("$CHILD_FUNS$", child_funs) \
    .replace("$CHILDREN$", children) \
    .replace("$IS_EMPTY$", is_empty)

    with open('../src/builders/' + tag + '.rs', 'wt') as out_file:
        out_file.write(result)

def gen_code():
    for element in elements:
        make_element_code(element)

if __name__ == "__main__":
    gen_code()
