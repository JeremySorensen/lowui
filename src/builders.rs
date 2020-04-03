pub mod a;
pub mod abbr;
pub mod address;
pub mod area;
pub mod article;
pub mod aside;
pub mod audio;
pub mod b;
pub mod bdi;
pub mod bdo;
pub mod blockquote;
pub mod br;
pub mod button;
pub mod canvas;
pub mod caption;
pub mod cite;
pub mod code;
pub mod col;
pub mod colgroup;
pub mod data;
pub mod datalist;
pub mod dd;
pub mod del;
pub mod details;
pub mod dfn;
pub mod dialog;
pub mod div;
pub mod dl;
pub mod dt;
pub mod em;
pub mod embed;
pub mod fieldset;
pub mod figcaption;
pub mod figure;
pub mod footer;
pub mod form;
pub mod h1;
pub mod h2;
pub mod h3;
pub mod h4;
pub mod h5;
pub mod h6;
pub mod header;
pub mod hr;
pub mod i;
pub mod iframe;
pub mod img;
pub mod input;
pub mod ins;
pub mod kbd;
pub mod label;
pub mod legend;
pub mod li;
pub mod main;
pub mod map;
pub mod mark;
pub mod meter;
pub mod nav;
pub mod object;
pub mod ol;
pub mod optgroup;
pub mod option;
pub mod output;
pub mod p;
pub mod param;
pub mod picture;
pub mod pre;
pub mod progress;
pub mod q;
pub mod rp;
pub mod rt;
pub mod ruby;
pub mod s;
pub mod samp;
pub mod section;
pub mod select;
pub mod small;
pub mod source;
pub mod span;
pub mod strong;
pub mod style;
pub mod sub;
pub mod summary;
pub mod sup;
pub mod svg;
pub mod table;
pub mod tbody;
pub mod td;
pub mod template;
pub mod textarea;
pub mod tfoot;
pub mod th;
pub mod thead;
pub mod time;
pub mod title;
pub mod tr;
pub mod track;
pub mod u;
pub mod ul;
pub mod var;
pub mod video;
pub mod wbr;

pub use a::a;
pub use abbr::abbr;
pub use address::address;
pub use area::area;
pub use article::article;
pub use aside::aside;
pub use audio::audio;
pub use b::b;
pub use bdi::bdi;
pub use bdo::bdo;
pub use blockquote::blockquote;
pub use br::br;
pub use button::button;
pub use canvas::canvas;
pub use caption::caption;
pub use cite::cite;
pub use code::code;
pub use col::col;
pub use colgroup::colgroup;
pub use data::data;
pub use datalist::datalist;
pub use dd::dd;
pub use del::del;
pub use details::details;
pub use dfn::dfn;
pub use dialog::dialog;
pub use div::div;
pub use dl::dl;
pub use dt::dt;
pub use em::em;
pub use embed::embed;
pub use fieldset::fieldset;
pub use figcaption::figcaption;
pub use figure::figure;
pub use footer::footer;
pub use form::form;
pub use h1::h1;
pub use h2::h2;
pub use h3::h3;
pub use h4::h4;
pub use h5::h5;
pub use h6::h6;
pub use header::header;
pub use hr::hr;
pub use i::i;
pub use iframe::iframe;
pub use img::img;
pub use input::input;
pub use ins::ins;
pub use kbd::kbd;
pub use label::label;
pub use legend::legend;
pub use li::li;
pub use main::main;
pub use map::map;
pub use mark::mark;
pub use meter::meter;
pub use nav::nav;
pub use object::object;
pub use ol::ol;
pub use optgroup::optgroup;
pub use option::option;
pub use output::output;
pub use p::p;
pub use param::param;
pub use picture::picture;
pub use pre::pre;
pub use progress::progress;
pub use q::q;
pub use rp::rp;
pub use rt::rt;
pub use ruby::ruby;
pub use s::s;
pub use samp::samp;
pub use section::section;
pub use select::select;
pub use small::small;
pub use source::source;
pub use span::span;
pub use strong::strong;
pub use style::style;
pub use sub::sub;
pub use summary::summary;
pub use sup::sup;
pub use svg::svg;
pub use table::table;
pub use tbody::tbody;
pub use td::td;
pub use template::template;
pub use textarea::textarea;
pub use tfoot::tfoot;
pub use th::th;
pub use thead::thead;
pub use time::time;
pub use title::title;
pub use tr::tr;
pub use track::track;
pub use u::u;
pub use ul::ul;
pub use var::var;
pub use video::video;
pub use wbr::wbr;

use crate::html::Node;

pub trait NodeBuilder {
    fn node(self) -> Node;
}

mod set_attr {
    pub trait SetAttr: Sized {
        fn set_attr<T: Into<String>>(self, name: &'static str, value: T) -> Self;
    }
}

pub trait EventAttr: set_attr::SetAttr {
    fn onabort<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onabort", value)
    }
    fn onauxclick<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onauxclick", value)
    }
    fn oncancel<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("oncancel", value)
    }
    fn oncanplay<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("oncanplay", value)
    }
    fn oncanplaythrough<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("oncanplaythrough", value)
    }
    fn onchange<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onchange", value)
    }
    fn onclick<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onclick", value)
    }
    fn onclose<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onclose", value)
    }
    fn oncuechange<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("oncuechange", value)
    }
    fn ondblclick<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondblclick", value)
    }
    fn ondrag<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondrag", value)
    }
    fn ondragend<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondragend", value)
    }
    fn ondragenter<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondragenter", value)
    }
    fn ondragexit<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondragexit", value)
    }
    fn ondragleave<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondragleave", value)
    }
    fn ondragover<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondragover", value)
    }
    fn ondragstart<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondragstart", value)
    }
    fn ondrop<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondrop", value)
    }
    fn ondurationchange<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ondurationchange", value)
    }
    fn onemptied<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onemptied", value)
    }
    fn onended<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onended", value)
    }
    fn oninput<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("oninput", value)
    }
    fn oninvalid<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("oninvalid", value)
    }
    fn onkeydown<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onkeydown", value)
    }
    fn onkeypress<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onkeypress", value)
    }
    fn onkeyup<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onkeyup", value)
    }
    fn onloadeddata<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onloadeddata", value)
    }
    fn onloadedmetadata<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onloadedmetadata", value)
    }
    fn onloadend<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onloadend", value)
    }
    fn onloadstart<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onloadstart", value)
    }
    fn onmousedown<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmousedown", value)
    }
    fn onmouseenter<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmouseenter", value)
    }
    fn onmouseleave<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmouseleave", value)
    }
    fn onmousemove<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmousemove", value)
    }
    fn onmouseout<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmouseout", value)
    }
    fn onmouseover<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmouseover", value)
    }
    fn onmouseup<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onmouseup", value)
    }
    fn onwheel<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onwheel", value)
    }
    fn onpause<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onpause", value)
    }
    fn onplay<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onplay", value)
    }
    fn onplaying<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onplaying", value)
    }
    fn onprogress<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onprogress", value)
    }
    fn onratechange<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onratechange", value)
    }
    fn onreset<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onreset", value)
    }
    fn onseeked<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onseeked", value)
    }
    fn onseeking<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onseeking", value)
    }
    fn onselect<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onselect", value)
    }
    fn onshow<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onshow", value)
    }
    fn onstalled<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onstalled", value)
    }
    fn onsubmit<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onsubmit", value)
    }
    fn onsuspend<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onsuspend", value)
    }
    fn ontimeupdate<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ontimeupdate", value)
    }
    fn ontoggle<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("ontoggle", value)
    }
    fn onvolumechange<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onvolumechange", value)
    }
    fn onwaiting<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onwaiting", value)
    }
    fn onblur<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onblur", value)
    }
    fn onerror<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onerror", value)
    }
    fn onfocus<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onfocus", value)
    }
    fn onload<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onload", value)
    }
    fn onresize<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onresize", value)
    }
    fn onscroll<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("onscroll", value)
    }
}

pub trait GlobalAttr: set_attr::SetAttr {
    fn accesskey<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("accesskey", value)
    }
    fn class<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("class", value)
    }
    fn contenteditable<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("contenteditable", value)
    }
    fn dir<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("dir", value)
    }
    fn draggable<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("draggable", value)
    }
    fn hidden<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("hidden", value)
    }
    fn id<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("id", value)
    }
    fn lang<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("lang", value)
    }
    fn spellcheck<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("spellcheck", value)
    }
    fn style<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("style", value)
    }
    fn tabindex<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("tabindex", value)
    }
    fn title<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("title", value)
    }
    fn translate<T: Into<String>>(self, value: T) -> Self {
        self.set_attr("translate", value)
    }

    fn data<T: Into<String>>(self, name: &'static str, value: T) -> Self {
        self.set_attr(name, value)
    }
}
