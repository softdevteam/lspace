use cairo::Context;

use std::cell::{RefCell, Ref};

use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use layout::flow_layout;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef};
use elements::container::TContainerElement;


pub struct FlowElementMut {
    req: ElementReq,
    alloc: ElementAlloc,
    children: Vec<ElementRef>,
    x_spacing: f64,
    y_spacing: f64,
    indentation: flow_layout::FlowIndent,
}


pub struct FlowElement {
    m: RefCell<FlowElementMut>,
    m_lines: RefCell<Vec<flow_layout::FlowLine>>
}

impl FlowElement {
    pub fn new(children: Vec<ElementRef>, x_spacing: f64, y_spacing: f64,
               indentation: flow_layout::FlowIndent) -> FlowElement {
        return FlowElement{m: RefCell::new(FlowElementMut{
                                req: ElementReq::new(), alloc: ElementAlloc::new(),
                                children: children, x_spacing: x_spacing, y_spacing: y_spacing,
                                indentation: indentation}), m_lines: RefCell::new(Vec::new())};
    }
}

impl TElement for FlowElement {
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
            flow_layout::requisition_x(&child_x_reqs, mm.x_spacing, mm.indentation)
        };
        mm.req.x_req = x_req;
    }

    fn allocate_x(&self) {
        let mm = self.m.borrow();
        let allocs_and_lines;
        {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_x_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.x_req).collect();

            allocs_and_lines = flow_layout::alloc_x(&mm.req.x_req,
                    &mm.alloc.x_alloc.without_position(), &child_x_reqs, mm.x_spacing,
                    mm.indentation);
        }
        (*self.m_lines.borrow_mut()).clone_from(&allocs_and_lines.1);
        for c in mm.children.iter().zip(allocs_and_lines.0.iter()) {
            c.0.element_update_x_alloc(c.1);
        }
        self.allocate_children_x();
    }

    fn update_y_req(&self) {
        self.update_children_y_req();
        let mut mm = self.m.borrow_mut();
        let mut b_lines = self.m_lines.borrow_mut();
        let mut lines: &mut Vec<flow_layout::FlowLine> = &mut (*b_lines);
        let y_req = {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();
            flow_layout::requisition_y(&child_y_reqs, mm.y_spacing, lines)
        };
        mm.req.y_req = y_req;
    }

    fn allocate_y(&self) {
        let mm = self.m.borrow();
        let y_allocs;
        {
            let child_reqs: Vec<Ref<ElementReq>> = mm.children.iter().map(|c| c.element_req()).collect();
            let child_y_reqs: Vec<&LReq> = child_reqs.iter().map(|c| &c.y_req).collect();

            let mut b_lines = self.m_lines.borrow_mut();
            let mut lines: &mut Vec<flow_layout::FlowLine> = &mut (*b_lines);
            y_allocs = flow_layout::alloc_y(&mm.req.y_req,
                    &mm.alloc.y_alloc.without_position(),
                    &child_y_reqs, mm.y_spacing, lines);
        }
        for c in mm.children.iter().zip(y_allocs.iter()) {
            c.0.element_update_y_alloc(c.1);
        }
        self.allocate_children_y();
    }
}


impl TContainerElement for FlowElement {
    fn children(&self) -> Ref<Vec<ElementRef>> {
        return Ref::map(self.m.borrow(), |m| &m.children);
    }
}
