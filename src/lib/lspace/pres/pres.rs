extern crate cairo;

use elements::element::ElementRef;
use elements::element_ctx::ElementContext;


pub struct PresBuildCtx<'a> {
    pub elem_ctx: &'a ElementContext,
}

impl <'a> PresBuildCtx<'a> {
    pub fn new(elem_ctx: &'a ElementContext) -> PresBuildCtx<'a> {
        PresBuildCtx{elem_ctx: elem_ctx}
    }
}


pub trait TPres {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef;
}

pub type Pres = Box<TPres>;
