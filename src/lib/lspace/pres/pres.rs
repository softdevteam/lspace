extern crate cairo;

use std::cell::RefCell;

use cairo::Context;

use elements::element::ElementChildRef;
use elements::element_ctx::ElementContext;


pub type Pres = Box<TPres>;


pub struct PresBuildCtx<'a> {
    pub elem_ctx: &'a RefCell<ElementContext>,
    pub cairo_ctx: &'a Context,
}

impl <'a> PresBuildCtx<'a> {
    pub fn new(elem_ctx: &'a RefCell<ElementContext>, cairo_ctx: &'a Context) -> PresBuildCtx<'a> {
        return PresBuildCtx{elem_ctx: elem_ctx, cairo_ctx: cairo_ctx};
    }
}


pub trait TPres {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementChildRef;
}
