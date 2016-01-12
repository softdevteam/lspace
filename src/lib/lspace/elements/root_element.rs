use cairo::Context;

use std::cell::{RefCell, Ref};

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

        let x_req: LReq = match mm.bin.get_child() {
            None => LReq::new_empty(),
            Some(ref ch) => ch.element_req().x_req.clone()
        };
        mm.req.x_req = x_req;
    }

    fn allocate_x(&self) {
        {
            let mm = self.m.borrow();
            match mm.bin.get_child() {
                None => {},
                Some(ref ch) => ch.element_update_x_alloc(&mm.alloc.x_alloc)
            };
        }
        self.allocate_children_x();
    }

    fn update_y_req(&self) {
        self.update_children_y_req();
        let mut mm = self.m.borrow_mut();
        let y_req: LReq = match mm.bin.get_child() {
            None => LReq::new_empty(),
            Some(ref ch) => ch.element_req().y_req.clone()
        };
        mm.req.y_req = y_req;
    }

    fn allocate_y(&self) {
        {
            let mm = self.m.borrow();
            match mm.bin.get_child() {
                None => {},
                Some(ref ch) => ch.element_update_y_alloc(&mm.alloc.y_alloc)
            };
        }
        self.allocate_children_y();
    }
}

const NO_CHILDREN: [ElementRef; 0] = [];

impl TContainerElement for RootElement {
    fn children(&self) -> Ref<[ElementRef]> {
        let m = self.m.borrow();
        return Ref::map(self.m.borrow(), |m| m.bin.children());
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
        self.m.borrow_mut().alloc.x_alloc = x_alloc;

        self.allocate_x();
    }

    fn root_requisition_y(&self) -> f64 {
        self.update_y_req();
        return self.m.borrow().req.y_req.size().size();
    }

    fn root_allocate_y(&self, height: f64) {
        let y_alloc = LAlloc::new_from_req_in_avail_size(&self.m.borrow().req.y_req, 0.0, height);
        self.m.borrow_mut().alloc.y_alloc = y_alloc;

        self.allocate_y();
    }
}


