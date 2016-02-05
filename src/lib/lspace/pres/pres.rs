extern crate cairo;

use cairo::Context;

use elements::element::ElementRef;
use elements::element_ctx::ElementContext;

use pyrs::PyWrapper;


pub type Pres = Box<TPres>;


pub struct PresBuildCtx<'a> {
    pub elem_ctx: &'a ElementContext,
}

impl <'a> PresBuildCtx<'a> {
    pub fn new(elem_ctx: &'a ElementContext) -> PresBuildCtx<'a> {
        return PresBuildCtx{elem_ctx: elem_ctx};
    }
}


pub trait TPres {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef;
}



pub type PyPres = PyWrapper<TPres>;
pub type PyPresOwned = Box<PyPres>;

// Function to destroy types that implement `TPres`
#[no_mangle]
pub extern "C" fn destroy_pres(wrapper: PyPresOwned) {
    PyWrapper::destroy(wrapper);
}
