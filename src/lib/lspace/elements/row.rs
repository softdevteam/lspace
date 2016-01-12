use cairo::Context;

use std::cell::{RefCell, Ref};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use layout::horizontal_layout;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef, ElementParentMut};
use elements::container::TContainerElement;
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement, ContainerSequenceComponentMut};
use elements::root_element::{TRootElement};


struct RowElementMut {
    parent: ElementParentMut,
    req: ElementReq,
    alloc: ElementAlloc,
    container_seq: ContainerSequenceComponentMut,
    x_spacing: f64,
}

pub struct RowElement {
    m: RefCell<RowElementMut>,
}


impl RowElement {
    pub fn new(x_spacing: f64) -> RowElement {
        return RowElement{m: RefCell::new(RowElementMut{
                parent: ElementParentMut::new(),
                req: ElementReq::new(), alloc: ElementAlloc::new(),
                container_seq: ContainerSequenceComponentMut::new(), x_spacing: x_spacing})};
    }
}


impl TElement for RowElement {
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


    /// Parent get and set methods
    fn get_parent(&self) -> Option<ElementRef> {
        return self.m.borrow().parent.get().clone();
    }

    fn set_parent(&self, p: Option<&ElementRef>) {
        self.m.borrow_mut().parent.set(p);
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
            let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
                    |c| c.element_req()).collect();
            let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();
            horizontal_layout::requisition_x(&child_x_reqs, mm.x_spacing)
        };
        mm.req.x_req = x_req;
    }

    fn allocate_x(&self) {
        let mm = self.m.borrow();
        let x_allocs;
        {
            let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
                    |c| c.element_req()).collect();
            let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();

            x_allocs = horizontal_layout::alloc_x(&mm.req.x_req,
                  &mm.alloc.x_alloc.without_position(), &child_x_reqs, mm.x_spacing);
        }
        for c in mm.container_seq.get_children().iter().zip(x_allocs.iter()) {
            c.0.element_update_x_alloc(c.1);
        }
        self.allocate_children_x();
    }

    fn update_y_req(&self) {
        self.update_children_y_req();
        let mut mm = self.m.borrow_mut();
        let y_req = {
            let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
                    |c| c.element_req()).collect();
            let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();
            horizontal_layout::requisition_y(&child_y_reqs)
        };
        mm.req.y_req = y_req;
    }

    fn allocate_y(&self) {
        let mm = self.m.borrow();
        let y_allocs;
        {
            let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
                    |c| c.element_req()).collect();
            let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();

            y_allocs = horizontal_layout::alloc_y(&mm.req.y_req,
                  &mm.alloc.y_alloc.without_position(), &child_y_reqs);
        }
        for c in mm.container_seq.get_children().iter().zip(y_allocs.iter()) {
            c.0.element_update_y_alloc(c.1);
        }
        self.allocate_children_y();
    }
}


impl TContainerElement for RowElement {
    fn children(&self) -> Ref<[ElementRef]> {
        return Ref::map(self.m.borrow(), |m| m.container_seq.children());
    }
}


impl TContainerSequenceElement for RowElement {
    fn get_children(&self) -> Ref<Vec<ElementRef>> {
        return Ref::map(self.m.borrow(), |m| m.container_seq.get_children());
    }

    fn set_children(&self, self_ref: &ElementRef, children: &Vec<ElementRef>) {
        self.m.borrow_mut().container_seq.set_children(self_ref, children);
    }
}
