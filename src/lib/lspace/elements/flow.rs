use cairo::Context;

use std::cell::{RefCell, Ref, RefMut};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use layout::flow_layout;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef, ElementParentMut};
use elements::container::TContainerElement;
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement, ContainerSequenceComponentMut};
use elements::root_element::{TRootElement};


struct FlowElementMut {
    parent: ElementParentMut,
    req: ElementReq,
    alloc: ElementAlloc,
    container_seq: ContainerSequenceComponentMut,
    x_spacing: f64,
    y_spacing: f64,
    indentation: flow_layout::FlowIndent,
    lines: Vec<flow_layout::FlowLine>,
}

impl FlowElementMut {
    fn compute_x_req(&mut self) -> LReq {
        let child_reqs: Vec<Ref<ElementReq>> = self.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();
        return flow_layout::requisition_x(&child_x_reqs, self.x_spacing, self.indentation);
    }

    fn compute_child_x_allocs(&mut self) -> Vec<LAlloc> {
        let allocs_and_lines;
        {
            let child_reqs: Vec<Ref<ElementReq>> = self.container_seq.get_children().iter().map(
                |c| c.element_req()).collect();
            let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();

            allocs_and_lines = flow_layout::alloc_x(&self.req.x_req,
                                                    &self.alloc.x_alloc.without_position(),
                                                    &child_x_reqs, self.x_spacing,
                                                    self.indentation);
        }
        self.lines.clone_from(&allocs_and_lines.1);
        return allocs_and_lines.0;
    }

    fn compute_y_req(&mut self) -> LReq {
        let child_reqs: Vec<Ref<ElementReq>> = self.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();

        return flow_layout::requisition_y(&child_y_reqs, self.y_spacing, &mut self.lines);
    }

    fn compute_child_y_allocs(&mut self) -> Vec<LAlloc> {
        let child_reqs: Vec<Ref<ElementReq>> = self.container_seq.get_children().iter().map(
            |c| c.element_req()).collect();
        let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();

        return flow_layout::alloc_y(&self.req.y_req,
                                    &self.alloc.y_alloc.without_position(),
                                    &child_y_reqs, self.y_spacing, &mut self.lines);
    }
}


pub struct FlowElement {
    m: RefCell<FlowElementMut>,
}

impl FlowElement {
    pub fn new(x_spacing: f64, y_spacing: f64,
               indentation: flow_layout::FlowIndent) -> FlowElement {
        return FlowElement{m: RefCell::new(FlowElementMut{
                parent: ElementParentMut::new(),
                req: ElementReq::new(), alloc: ElementAlloc::new(),
                container_seq: ContainerSequenceComponentMut::new(),
                x_spacing: x_spacing, y_spacing: y_spacing,
                indentation: indentation,
                lines: Vec::new()})};
    }
}

impl TElement for FlowElement {
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
        let update_needed;
        {
            let mut elem_alloc = self.element_alloc_mut();
            let changed = elem_alloc.update_x_alloc(x_alloc);
            update_needed = changed | elem_alloc.is_x_alloc_update_required();
            elem_alloc.x_alloc_updated();
            if update_needed {
                elem_alloc.y_req_dirty();
                elem_alloc.y_alloc_dirty();
            }
        }
        if update_needed {
            let x_allocs = self.compute_child_x_allocs();
            self.allocate_children_x(&x_allocs);
        }
        return update_needed;
    }

    fn update_y_req(&self) -> bool {
        return self.container_update_y_req();
    }

    fn allocate_y(&self, y_alloc: &LAlloc) {
        self.container_allocate_y(y_alloc);
    }
}


impl TContainerElement for FlowElement {
    fn children(&self) -> Ref<[ElementRef]> {
        return Ref::map(self.m.borrow(), |m| m.container_seq.children());
    }

    fn compute_x_req(&self) -> LReq {
        return self.m.borrow_mut().compute_x_req();
    }

    fn compute_child_x_allocs(&self) -> Vec<LAlloc> {
        return self.m.borrow_mut().compute_child_x_allocs();
    }

    fn compute_y_req(&self) -> LReq {
        return self.m.borrow_mut().compute_y_req();
    }

    fn compute_child_y_allocs(&self) -> Vec<LAlloc> {
        return self.m.borrow_mut().compute_child_y_allocs();
    }
}


impl TContainerSequenceElement for FlowElement {
    fn get_children(&self) -> Ref<Vec<ElementRef>> {
        return Ref::map(self.m.borrow(), |m| m.container_seq.get_children());
    }

    fn set_children(&self, self_ref: &ElementRef, children: &Vec<ElementRef>) {
        self.m.borrow_mut().container_seq.set_children(self_ref, children);
    }
}
