use cairo::Context;

use std::cell::{RefCell, Ref, RefMut};

use layout::lalloc::LAlloc;
use layout::lreq::LReq;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef, ElementParentMut};
use elements::container::TContainerElement;
use elements::bin::{TBinElement, BinComponentMut};
use elements::container_sequence::{TContainerSequenceElement};


pub trait TRootElement : TBinElement {
    fn root_requisition_x(&self) -> f64;
    fn root_allocate_x(&self, width: f64);
    fn root_requisition_y(&self) -> f64;

    fn root_allocate_y(&self, height: f64);
}


struct RootElementMut {
    req: ElementReq,
    alloc: ElementAlloc,
    bin: BinComponentMut,
}

pub struct RootElement {
    m: RefCell<RootElementMut>,
}

impl RootElement {
    pub fn new() -> RootElement {
        return RootElement{m: RefCell::new(RootElementMut{
            req: ElementReq::new(), alloc: ElementAlloc::new(),
            bin: BinComponentMut::new()})};
    }
}

impl TElement for RootElement {
    /// Interface acquisition
    fn as_container(&self) -> Option<&TContainerElement> {
        return Some(self);
    }

    fn as_bin(&self) -> Option<&TBinElement> {
        return Some(self);
    }

    fn as_container_sequence(&self) -> Option<&TContainerSequenceElement> {
        return None
    }

    fn as_root_element(&self) -> Option<&TRootElement> {
        return Some(self);
    }

    /// Parent get and set methods
    fn get_parent(&self) -> Option<ElementRef> {
        return None;
    }

    fn set_parent(&self, p: Option<&ElementRef>) {
        panic!("Cannot set parent of root element");
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

const NO_CHILDREN: [ElementRef; 0] = [];

impl TContainerElement for RootElement {
    fn children(&self) -> Ref<[ElementRef]> {
        let m = self.m.borrow();
        return Ref::map(self.m.borrow(), |m| m.bin.children());
    }

    fn compute_x_req(&self) -> LReq {
        let mut mm = self.m.borrow_mut();
        return match mm.bin.get_child() {
            None => LReq::new_empty(),
            Some(ref ch) => ch.element_req().x_req.clone()
        };
    }

    fn compute_child_x_allocs(&self) -> Vec<LAlloc> {
        let mm = self.m.borrow();
        return match mm.bin.get_child() {
            None => vec![],
            Some(ref ch) => vec![mm.alloc.x_alloc]
        };
    }

    fn compute_y_req(&self) -> LReq {
        let mut mm = self.m.borrow_mut();
        return match mm.bin.get_child() {
            None => LReq::new_empty(),
            Some(ref ch) => ch.element_req().y_req.clone()
        };
    }

    fn compute_child_y_allocs(&self) -> Vec<LAlloc> {
        let mm = self.m.borrow();
        return match mm.bin.get_child() {
            None => vec![],
            Some(ref ch) => vec![mm.alloc.y_alloc]
        };
    }
}

impl TBinElement for RootElement {
    fn get_child(&self) -> Option<ElementRef> {
        let mm = self.m.borrow();
        return mm.bin.get_child();
    }

    fn set_child(&self, self_ref: &ElementRef, child: ElementRef) {
        let mut mm = self.m.borrow_mut();
        mm.bin.set_child(self_ref, child);
    }

    fn clear_child(&self) {
        let mut mm = self.m.borrow_mut();
        mm.bin.clear_child();
    }
}

impl TRootElement for RootElement {
    fn root_requisition_x(&self) -> f64 {
        self.update_x_req();
        return self.m.borrow().req.x_req.size().size();
    }

    fn root_allocate_x(&self, width: f64) {
        // Need to assign to local variable first, then mutate value to ensure that the dynamic
        // borrows don't overlap
        let x_alloc = LAlloc::new_from_req_in_avail_size(&self.m.borrow().req.x_req, 0.0, width);

        self.allocate_x(&x_alloc);
    }

    fn root_requisition_y(&self) -> f64 {
        self.update_y_req();
        return self.m.borrow().req.y_req.size().size();
    }

    fn root_allocate_y(&self, height: f64) {
        let y_alloc = LAlloc::new_from_req_in_avail_size(&self.m.borrow().req.y_req, 0.0, height);

        self.allocate_y(&y_alloc);
    }
}


