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
    m: RefCell<ElementContextMut>,
    empty_shared_req: Rc<ElementReq>
}

impl ElementContext {
    pub fn new() -> ElementContext {
        ElementContext{m: RefCell::new(ElementContextMut{req_table: HashMap::new()}),
                       empty_shared_req: Rc::new(ElementReq::new())}
    }

    pub fn text_shared_req(&self, style: Rc<TextStyleParams>, text: String,
                           cairo_ctx: &Context) -> Rc<ElementReq> {
        self.m.borrow_mut().text_shared_req(style, text, cairo_ctx)
    }

    pub fn empty_shared_req(&self) -> Rc<ElementReq> {
        return self.empty_shared_req.clone();
    }
}


pub struct ElementLayoutContext <'a> {
    ctx: &'a ElementContext,
    cairo_ctx: &'a Context
}

impl <'a> ElementLayoutContext<'a> {
    pub fn new<'b>(ctx: &'b ElementContext, cairo_ctx: &'b Context) -> ElementLayoutContext<'b> {
        ElementLayoutContext{ctx: ctx, cairo_ctx: cairo_ctx}
    }

    pub fn elem_ctx(&'a self) -> &'a ElementContext {
        self.ctx
    }

    pub fn cairo_ctx(&'a self) -> &'a Context {
        self.cairo_ctx
    }
}
