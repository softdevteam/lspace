use cairo::Context;

use std::cell::{RefCell, Ref};

use layout::lalloc::LAlloc;
use layout::lreq::LReq;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef};
use elements::container::TContainerElement;


struct RootElementMut {
    req: ElementReq,
    alloc: ElementAlloc,
    children: Vec<ElementRef>,
}

pub struct RootElement {
    m: RefCell<RootElementMut>,
}

impl RootElement {
    pub fn new(child: ElementRef) -> RootElement {
        return RootElement{m: RefCell::new(RootElementMut{
                                req: ElementReq::new(), alloc: ElementAlloc::new(),
                                children: vec![child]})};
    }

    pub fn root_requisition_x(&self) -> f64 {
        self.update_x_req();
        return self.m.borrow().req.x_req.size().size();
    }

    pub fn root_allocate_x(&self, width: f64) {
        // The following line of code would make sense in other languages:
        //
        //    self.m.borrow_mut().alloc.x_alloc =
        //          LAlloc::new_from_req_in_avail_size(&self.m.borrow().req.x_req, 0.0, width);
        //
        // The mutable borrow for the assignment could only be taken out after the value has been
        // computed; since the immutable borrow is only required during the computation, surely
        // it could expire before the assignment starts, but alas, we have to do the following in
        // order to hand-hold rust to make sure things go in and out of scope at the right time...
        let x_alloc = LAlloc::new_from_req_in_avail_size(&self.m.borrow().req.x_req, 0.0, width);
        self.m.borrow_mut().alloc.x_alloc = x_alloc;

        self.allocate_x();
    }

    pub fn root_requisition_y(&self) -> f64 {
        self.update_y_req();
        return self.m.borrow().req.y_req.size().size();
    }

    pub fn root_allocate_y(&self, height: f64) {
        let y_alloc = LAlloc::new_from_req_in_avail_size(&self.m.borrow().req.y_req, 0.0, height);
        self.m.borrow_mut().alloc.y_alloc = y_alloc;

        self.allocate_y();
    }
}

impl TElement for RootElement {
    fn element_req(&self) -> Ref<ElementReq> {
        return Ref::map(self.m.borrow(), |m| &m.req);
    }

    fn element_alloc(&self) -> Ref<ElementAlloc> {
        return Ref::map(self.m.borrow(), |m| &m.alloc);
    }

    /// Update element X allocation
    fn element_update_x_alloc(&self, x_alloc: &LAlloc) {
        self.m.borrow_mut().alloc.x_alloc.clone_from(x_alloc);
    }
    /// Update element Y allocation
    fn element_update_y_alloc(&self, y_alloc: &LAlloc) {
        self.m.borrow_mut().alloc.y_alloc.clone_from(y_alloc);
    }


    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        self.draw_self(cairo_ctx, visible_region);
        self.draw_children(cairo_ctx, visible_region);
    }


    fn update_x_req(&self) {
        self.update_children_x_req();
        let mut mm = self.m.borrow_mut();
        let x_req: LReq = {mm.children[0].get().element_req().x_req.clone()};
        mm.req.x_req = x_req;
    }

    fn allocate_x(&self) {
        {
            let mm = self.m.borrow();
            mm.children[0].get().element_update_x_alloc(&mm.alloc.x_alloc);
        }
        self.allocate_children_x();
    }

    fn update_y_req(&self) {
        self.update_children_y_req();
        let mut mm = self.m.borrow_mut();
        let y_req: LReq = {mm.children[0].get().element_req().y_req.clone()};
        mm.req.y_req = y_req;
    }

    fn allocate_y(&self) {
        {
            let mm = self.m.borrow();
            mm.children[0].get().element_update_y_alloc(&mm.alloc.y_alloc);
        }
        self.allocate_children_y();
    }
}

impl TContainerElement for RootElement {
    fn children(&self) -> Ref<Vec<ElementRef>> {
        return Ref::map(self.m.borrow(), |m| &m.children);
    }
}

