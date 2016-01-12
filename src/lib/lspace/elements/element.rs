use cairo::Context;

use std::rc::Rc;
use std::cell::{RefCell, Ref};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::container::{TContainerElement};
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use elements::root_element::{TRootElement};


pub type ElementRef = Rc<TElement>;

pub fn elem_as_ref<T: TElement + 'static>(x: T) -> ElementRef {
    return Rc::new(x);
}


pub trait TElement {
    /// Interface acquisition
    fn as_container(&self) -> Option<&TContainerElement>;
    fn as_bin(&self) -> Option<&TBinElement>;
    fn as_container_sequence(&self) -> Option<&TContainerSequenceElement>;
    fn as_root_element(&self) -> Option<&TRootElement>;

    /// Parent get and set methods
    fn get_parent(&self) -> Option<ElementRef>;
    fn set_parent(&self, p: Option<&ElementRef>);


    /// Acquire reference to the element layout requisition
    fn element_req(&self) -> Ref<ElementReq>;
    /// Acquire reference to the element layout allocation
    fn element_alloc(&self) -> Ref<ElementAlloc>;
    /// Update element X allocation
    fn element_update_x_alloc(&self, x_alloc: &LAlloc);
    /// Update element Y allocation
    fn element_update_y_alloc(&self, y_alloc: &LAlloc);


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


pub struct ElementParentMut {
    parent: Option<ElementRef>
}

impl ElementParentMut {
    pub fn new() -> ElementParentMut {
        return ElementParentMut{parent: None};
    }

    pub fn get(&self) -> Option<ElementRef> {
        return self.parent.clone();
    }

    pub fn set(&mut self, p: Option<&ElementRef>) {
        self.parent = match p {
            None => None,
            Some(pp) => Some(pp.clone())
        };
    }
}