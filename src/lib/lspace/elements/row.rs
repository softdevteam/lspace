use cairo::Context;

use std::cell::{RefCell, Ref, RefMut};

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

    // Layout structure acquisition
    fn element_req(&self) -> Ref<ElementReq> {
        return Ref::map(self.m.borrow(), |m| &m.req);
    }

    fn element_alloc(&self) -> Ref<ElementAlloc> {
        return Ref::map(self.m.borrow(), |m| &m.alloc);
    }

    fn element_alloc_mut(&self) -> RefMut<ElementAlloc> {
        return RefMut::map(self.m.borrow_mut(), |m| &mut m.alloc);
    }

    /// Update element X requisition
    fn element_update_x_req(&self, x_req: &LReq) -> bool {
        return self.m.borrow_mut().req.update_x_req(x_req);
    }

    /// Update element Y requisition
    fn element_update_y_req(&self, y_req: &LReq) -> bool {
        return self.m.borrow_mut().req.update_y_req(y_req);
    }

    // Draw
    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        self.draw_self(cairo_ctx, visible_region);
        self.draw_children(cairo_ctx, visible_region);
    }

    // Update layout
    fn update_x_req(&self) -> bool {
        return self.container_update_x_req();
    }

    fn allocate_x(&self, x_alloc: &LAlloc) -> bool {
        return self.container_allocate_x(x_alloc);
    }

    fn update_y_req(&self) -> bool {
        return self.container_update_y_req();
    }

    fn allocate_y(&self, y_alloc: &LAlloc) {
        self.container_allocate_y(y_alloc);
    }
}


impl TContainerElement for RowElement {
    fn children(&self) -> Ref<[ElementRef]> {
        return Ref::map(self.m.borrow(), |m| m.container_seq.children());
    }

    fn compute_x_req(&self) -> LReq {
        let mut mm = self.m.borrow_mut();
        let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();
        return horizontal_layout::requisition_x(&child_x_reqs, mm.x_spacing);
    }

    fn compute_child_x_allocs(&self) -> Vec<LAlloc> {
        let mut mm = self.m.borrow_mut();
        let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();

        return horizontal_layout::alloc_x(&mm.req.x_req,
                                          &mm.alloc.x_alloc.without_position(), &child_x_reqs,

                                          mm.x_spacing);
    }

    fn compute_y_req(&self) -> LReq {
        let mut mm = self.m.borrow_mut();
        let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();
        return horizontal_layout::requisition_y(&child_y_reqs);
    }

    fn compute_child_y_allocs(&self) -> Vec<LAlloc> {
        let mut mm = self.m.borrow_mut();
        let child_reqs: Vec<Ref<ElementReq>> = mm.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();

        return horizontal_layout::alloc_y(&mm.req.y_req,
                                          &mm.alloc.y_alloc.without_position(), &child_y_reqs);
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
