use cairo::Context;

use std::rc::Rc;
use std::cell::{Ref, RefMut};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element_ctx::ElementLayoutContext;
use elements::text_element::TTextElement;
use elements::container::{TContainerElement};
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use elements::root_element::{TRootElement};
use pyrs::PyRcWrapper;


pub type ElementRef = Rc<TElement>;

pub fn elem_as_ref<T: TElement + 'static>(x: T) -> ElementRef {
    return Rc::new(x);
}


pub trait TElement {
    /// Interface acquisition
    fn as_text_element(&self) -> Option<&TTextElement> {
        return None;
    }

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
    fn element_alloc_mut(&self) -> RefMut<ElementAlloc>;
    /// Update element X requisition
    fn element_update_x_req(&self, x_req: &LReq) -> bool;
    /// Update element Y requisition
    fn element_update_y_req(&self, y_req: &LReq) -> bool;

    /// Paint the element content that is contributed by the element itself, as opposed to child
    /// elements.
    fn draw_self(&self, cairo_ctx: &Context, visible_region: &BBox2) {
    }

    /// Paint the element along with its children
    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2);

    /// Update layout: X requisition
    fn update_x_req(&self, layout_ctx: &ElementLayoutContext) -> bool;

    /// Update layout: X allocation
    fn allocate_x(&self, x_alloc: &LAlloc) -> bool;

    /// Update layout: Y requisition
    fn update_y_req(&self) -> bool;

    /// Update layout: Y allocation
    fn allocate_y(&self, y_alloc: &LAlloc);
}


fn queue_resize_for_elem(elem: &TElement) {
    let mut alloc_mut = elem.element_alloc_mut();
    alloc_mut.x_req_dirty();
    alloc_mut.y_req_dirty();
    alloc_mut.x_alloc_dirty();
    alloc_mut.y_alloc_dirty();
}

fn queue_redraw_for_elem_if_root(elem: &TElement, bbox: Option<&BBox2>) {
    match elem.as_root_element() {
        Some(ref r) => {
            let redraw_bbox = match bbox {
                Some (b) => *b,
                None => r.element_alloc().local_bbox()
            };

            r.root_queue_redraw(&redraw_bbox);
        },
        None => {}
    }

}

pub fn queue_resize(elem: &TElement) {
    queue_resize_for_elem(elem);
    queue_redraw_for_elem_if_root(elem, None);

    let mut x: Option<ElementRef> = elem.get_parent();

    while !x.is_none() {
        let e = x.unwrap();
        queue_resize_for_elem(&*e);
        queue_redraw_for_elem_if_root(&*e, None);

        x = e.get_parent();
    }
}

pub fn queue_redraw(elem: &TElement) {
    queue_redraw_for_elem_if_root(elem, None);

    let mut bbox = elem.element_alloc().local_bbox();
    bbox = elem.element_alloc().local_bbox_to_parent_space(&bbox);
    let mut x: Option<ElementRef> = elem.get_parent();

    while !x.is_none() {
        let e = x.unwrap();

        let alloc = e.element_alloc();
        bbox = alloc.local_bbox_to_parent_space(&bbox);

        queue_redraw_for_elem_if_root(&*e, Some(&bbox));

        x = e.get_parent();
    }


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


pub type PyElement = PyRcWrapper<TElement>;
pub type PyElementOwned = Box<PyElement>;

// Function to destroy types that implement `TElement`
#[no_mangle]
pub extern "C" fn destroy_element(wrapper: PyElementOwned) {
    PyElement::destroy(wrapper);
}
