use cairo::Context;

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

use layout::lalloc::LAlloc;
use layout::lreq::LReq;
use geom::bbox2::BBox2;
use graphics::border::Border;
use elements::element_ctx::ElementLayoutContext;
use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef, ElementParentMut};
use elements::container::TContainerElement;
use elements::bin::{TBinElement, BinComponentMut};
use elements::container_sequence::{TContainerSequenceElement};
use elements::root_element::{TRootElement};


struct BorderElementMut {
    parent: ElementParentMut,
    req: ElementReq,
    alloc: ElementAlloc,
    bin: BinComponentMut,
}

pub struct BorderElement {
    m: RefCell<BorderElementMut>,
    border: Rc<Border>,
}

impl BorderElement {
    pub fn new(border: &Rc<Border>) -> BorderElement {
        return BorderElement{m: RefCell::new(BorderElementMut{
            parent: ElementParentMut::new(),
            req: ElementReq::new(), alloc: ElementAlloc::new(),
            bin: BinComponentMut::new()}),
            border: border.clone()};
    }
}

impl TElement for BorderElement {
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

    /// Paint the element content that is contributed by the element itself
    fn draw_self(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        let mm = self.m.borrow();
        let alloc = &mm.alloc;
        self.border.draw_background(cairo_ctx,
                                    alloc.x_alloc.pos_in_parent(), alloc.y_alloc.pos_in_parent(),
                                    alloc.x_alloc.actual_size(), alloc.y_alloc.actual_size());
        self.border.draw(cairo_ctx,
                         alloc.x_alloc.pos_in_parent(), alloc.y_alloc.pos_in_parent(),
                         alloc.x_alloc.actual_size(), alloc.y_alloc.actual_size());
    }

    /// Draw
    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        self.draw_self(cairo_ctx, visible_region);
        self.draw_children(cairo_ctx, visible_region);
    }

    // Update layout
    fn update_x_req(&self, layout_ctx: &ElementLayoutContext) -> bool {
        self.container_update_x_req(layout_ctx)
    }

    fn allocate_x(&self, x_alloc: &LAlloc) -> bool {
        self.container_allocate_x(x_alloc)
    }

    fn update_y_req(&self) -> bool {
        self.container_update_y_req()
    }

    fn allocate_y(&self, y_alloc: &LAlloc) {
        self.container_allocate_y(y_alloc);
    }
}


impl TContainerElement for BorderElement {
    fn children(&self) -> Ref<[ElementRef]> {
        Ref::map(self.m.borrow(), |m| m.bin.children())
    }

    fn compute_x_req(&self) -> LReq {
        let mm = self.m.borrow();
        let child_x_req = match mm.bin.get_child() {
            None => LReq::new_empty(),
            Some(ref ch) => ch.element_req().x_req.clone()
        };
        child_x_req.apply_border(self.border.left_margin(), self.border.right_margin())
    }

    fn compute_child_x_allocs(&self) -> Vec<LAlloc> {
        let mm = self.m.borrow();
        match mm.bin.get_child() {
            None => vec![],
            Some(_) => vec![mm.alloc.x_alloc.apply_border(self.border.left_margin(),
                                                          self.border.right_margin())]
        }
    }

    fn compute_y_req(&self) -> LReq {
        let mm = self.m.borrow();
        let child_y_req = match mm.bin.get_child() {
            None => LReq::new_empty(),
            Some(ref ch) => ch.element_req().y_req.clone()
        };
        child_y_req.apply_border(self.border.top_margin(), self.border.bottom_margin())
    }

    fn compute_child_y_allocs(&self) -> Vec<LAlloc> {
        let mm = self.m.borrow();
        match mm.bin.get_child() {
            None => vec![],
            Some(_) => vec![mm.alloc.y_alloc.apply_border(self.border.top_margin(),
                                                          self.border.bottom_margin())]
        }
    }
}

impl TBinElement for BorderElement {
    fn get_child(&self) -> Option<ElementRef> {
        let mm = self.m.borrow();
        mm.bin.get_child()
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
