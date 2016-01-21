extern crate cairo;

use cairo::Context;

use elements::element::ElementRef;
use elements::element_ctx::ElementContext;


pub type Pres = Box<TPres>;


pub struct PresBuildCtx<'a> {
    pub elem_ctx: &'a ElementContext,
    pub cairo_ctx: &'a Context,
}

impl <'a> PresBuildCtx<'a> {
    pub fn new(elem_ctx: &'a ElementContext, cairo_ctx: &'a Context) -> PresBuildCtx<'a> {
        return PresBuildCtx{elem_ctx: elem_ctx, cairo_ctx: cairo_ctx};
    }
}


pub trait TPres {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef;
}
