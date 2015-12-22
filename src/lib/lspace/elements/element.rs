use cairo::Context;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};


pub type ElemBorrow<'a> = Ref<'a, Box<TElement>>;
pub type ElemBorrowMut<'a> = RefMut<'a, Box<TElement>>;


pub struct ElementRef {
    x: Rc<RefCell<Box<TElement>>>
}

impl ElementRef {
    pub fn new<T: TElement + 'static>(x: T) -> ElementRef {
        return ElementRef{x: Rc::new(RefCell::new(Box::new(x)))};
    }

    pub fn get(&self) -> ElemBorrow {
        return self.x.borrow();
    }

    pub fn get_mut(&mut self) -> ElemBorrowMut {
        return self.x.borrow_mut();
    }
}


pub trait TElement {
    /// Acquire reference to the element layout requisition
    fn element_req(&self) -> &ElementReq;
    /// Acquire reference to the element layout allocation
    fn element_alloc(&self) -> &ElementAlloc;
    /// Acquire reference to element layout requisition and mutable allocation
    fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc);

    /// Acquire reference to element layout X requisition
    fn x_req(&self) -> &LReq {
        return &self.element_req().x_req;
    }

    /// Acquire reference to element layout X allocation
    fn x_alloc(&self) -> &LAlloc {
        return &self.element_alloc().x_alloc;
    }

    /// Acquire reference to element layout X requisition and mutable X allocation
    fn x_req_and_mut_alloc(&mut self) -> (&LReq, &mut LAlloc) {
        let ra = self.element_req_and_mut_alloc();
        return (&ra.0.x_req, &mut ra.1.x_alloc);
    }

    /// Acquire reference to element layout Y requisition
    fn y_req(&self) -> &LReq {
        return &self.element_req().y_req;
    }

    /// Acquire reference to element layout Y allocation
    fn y_alloc(&self) -> &LAlloc {
        return &self.element_alloc().y_alloc;
    }

    /// Acquire reference to element layout Y requisition and mutable Y allocation
    fn y_req_and_mut_alloc(&mut self) -> (&LReq, &mut LAlloc) {
        let ra = self.element_req_and_mut_alloc();
        return (&ra.0.y_req, &mut ra.1.y_alloc);
    }


    /// Paint the element content that is contributed by the element itself, as opposed to child
    /// elements.
    fn draw_self(&self, cairo_ctx: &Context, visible_region: &BBox2) {
    }

    /// Paint the element along with its children
    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2);

    /// Update layout: X requisition
    fn update_x_req(&mut self);

    /// Update layout: X allocation
    fn allocate_x(&mut self);

    /// Update layout: Y requisition
    fn update_y_req(&mut self);

    /// Update layout: Y allocation
    fn allocate_y(&mut self);
}
