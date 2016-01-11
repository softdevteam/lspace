use cairo::Context;

use std::cell::{RefCell, Ref};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use layout::vertical_layout;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef};
use elements::container::TContainerElement;
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use elements::root_element::{TRootElement};


struct ColumnElementMut {
    req: ElementReq,
    alloc: ElementAlloc,
    children: Vec<ElementRef>,
    y_spacing: f64,
}

pub struct ColumnElement {
    m: RefCell<ColumnElementMut>
}


impl ColumnElement {
    pub fn new(y_spacing: f64) -> ColumnElement {
        return ColumnElement{m: RefCell::new(ColumnElementMut{
                                req: ElementReq::new(), alloc: ElementAlloc::new(),
                                children: Vec::new(), y_spacing: y_spacing})};
    }
}


impl TElement for ColumnElement {
    /// Interface acquisition
    fn as_container(&self) -> Option<&TContainerElement> {
        return Some(self);
    }

    fn as_bin(&self) -> Option<&TBinElement> {
        return None;
    }

    fn as_container_sequence(&self) -> Option<&TContainerSequenceElement> {
        return Some(self);
    }

    fn as_root_element(&self) -> Option<&TRootElement> {
        return None;
    }


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
        let x_req = {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();
            vertical_layout::requisition_x(&child_x_reqs)
        };
        mm.req.x_req = x_req;
    }

    fn allocate_x(&self) {
        let mm = self.m.borrow();
        let x_allocs;
        {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();

            x_allocs = vertical_layout::alloc_x(&mm.req.x_req,
                    &mm.alloc.x_alloc.without_position(), &child_x_reqs);
        }
        for c in mm.children.iter().zip(x_allocs.iter()) {
            c.0.element_update_x_alloc(c.1);
        }
        self.allocate_children_x();
    }

    fn update_y_req(&self) {
        self.update_children_y_req();
        let mut mm = self.m.borrow_mut();
        let y_req = {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();
            vertical_layout::requisition_y(&child_y_reqs, mm.y_spacing, None)
        };
        mm.req.y_req = y_req;
    }

    fn allocate_y(&self) {
        let mm = self.m.borrow();
        let y_allocs;
        {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();

            y_allocs = vertical_layout::alloc_y(&mm.req.y_req,
                    &mm.alloc.y_alloc.without_position(), &child_y_reqs, mm.y_spacing, None);
        }
        for c in mm.children.iter().zip(y_allocs.iter()) {
            c.0.element_update_y_alloc(c.1);
        }
        self.allocate_children_y();
    }
}


impl TContainerElement for ColumnElement {
    fn children(&self) -> Ref<Vec<ElementRef>> {
        return Ref::map(self.m.borrow(), |m| &m.children);
    }
}


impl TContainerSequenceElement for ColumnElement {
    fn get_children(&self) -> Ref<Vec<ElementRef>> {
        return Ref::map(self.m.borrow(), |m| &m.children);
    }

    fn set_children(&self, self_ref: &ElementRef, children: &Vec<ElementRef>) {
        let mut mm = self.m.borrow_mut();
        mm.children.clone_from(children);
    }
}
