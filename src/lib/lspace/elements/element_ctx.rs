use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;
use std::cell::RefCell;

use cairo::Context;

use layout::lreq::LReq;

use elements::element_layout::ElementReq;
use elements::text_element::{TextReqKey, TextStyleParams};



struct ElementContextMut {
    req_table: HashMap<TextReqKey, Rc<ElementReq>>,
}

impl ElementContextMut {
    fn text_shared_req(&mut self, style: Rc<TextStyleParams>, text: String,
                       cairo_ctx: &Context) -> Rc<ElementReq> {
        let key = style.text_req_key(text.clone());
        let req_entry = self.req_table.entry(key);
        return match req_entry {
            Entry::Vacant(v) => {
                style.apply(cairo_ctx);
                let font_extents = cairo_ctx.font_extents();
                let text_extents = cairo_ctx.text_extents(text.clone().as_str());
                let x_req = LReq::new_fixed_size(text_extents.x_advance);
                let y_req = LReq::new_fixed_ref(font_extents.ascent, font_extents.descent);
                let shreq = Rc::new(ElementReq::new_from_reqs(x_req, y_req));
                v.insert(shreq.clone());
                shreq

            },
            Entry::Occupied(o) => o.get().clone()
        };
    }
}


pub struct ElementContext {
    m: RefCell<ElementContextMut>
}

impl ElementContext {
    pub fn new() -> ElementContext {
        ElementContext{m: RefCell::new(ElementContextMut{req_table: HashMap::new()})}
    }

    pub fn text_shared_req(&self, style: Rc<TextStyleParams>, text: String,
                           cairo_ctx: &Context) -> Rc<ElementReq> {
        self.m.borrow_mut().text_shared_req(style, text, cairo_ctx)
    }
}

