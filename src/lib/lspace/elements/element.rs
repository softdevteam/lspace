use cairo::Context;

use std::rc::Rc;
use std::cell::{RefCell, Ref};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};


pub type ElementRef = Rc<TElement>;


pub trait TElement {
    /// Acquire reference to the element layout requisition
    fn element_req(&self) -> Ref<ElementReq>;
    /// Acquire reference to the element layout allocation
    fn element_alloc(&self) -> Ref<ElementAlloc>;
    /// Update element X allocation
    fn element_update_x_alloc(&self, x_alloc: &LAlloc);
    /// Update element Y allocation
    fn element_update_y_alloc(&self, y_alloc: &LAlloc);

    /// Acquire reference to element layout X requisition
    fn x_req(&self) -> Ref<LReq> {
        return Ref::map(self.element_req(), |r| &r.x_req);
    }

    /// Acquire reference to element layout X allocation
    fn x_alloc(&self) -> Ref<LAlloc> {
        return Ref::map(self.element_alloc(), |a| &a.x_alloc);
    }

    /// Acquire reference to element layout Y requisition
    fn y_req(&self) -> Ref<LReq> {
        return Ref::map(self.element_req(), |r| &r.y_req);
    }

    /// Acquire reference to element layout Y allocation
    fn y_alloc(&self) -> Ref<LAlloc> {
        return Ref::map(self.element_alloc(), |a| &a.y_alloc);
    }


    /// Paint the element content that is contributed by the element itself, as opposed to child
    /// elements.
    fn draw_self(&self, cairo_ctx: &Context, visible_region: &BBox2) {
    }

    /// Paint the element along with its children
    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2);

    /// Update layout: X requisition
    fn update_x_req(&self);

    /// Update layout: X allocation
    fn allocate_x(&self);

    /// Update layout: Y requisition
    fn update_y_req(&self);

    /// Update layout: Y allocation
    fn allocate_y(&self);
}
