use cairo::Context;

use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;

use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element::{TElement, ElementRef};
use elements::container::TContainerElement;


pub struct RootElement {
    req: ElementReq,
    alloc: ElementAlloc,
    children: Vec<ElementRef>,
}

impl RootElement {
    pub fn new(child: ElementRef) -> RootElement {
        return RootElement{req: ElementReq::new(), alloc: ElementAlloc::new(),
                           children: vec![child]};
    }

    pub fn root_requisition_x(&mut self) -> f64 {
        self.update_x_req();
        return self.req.x_req.size().size();
    }

    pub fn root_allocate_x(&mut self, width: f64) {
        self.alloc.x_alloc = LAlloc::new_from_req_in_avail_size(&self.req.x_req, 0.0, width);
        self.allocate_x();
    }

    pub fn root_requisition_y(&mut self) -> f64 {
        self.update_y_req();
        return self.req.y_req.size().size();
    }

    pub fn root_allocate_y(&mut self, height: f64) {
        self.alloc.y_alloc = LAlloc::new_from_req_in_avail_size(&self.req.y_req, 0.0, height);
        self.allocate_y();
    }
}

impl TElement for RootElement {
    fn element_req(&self) -> &ElementReq {
        return &self.req;
    }

    fn element_alloc(&self) -> &ElementAlloc {
        return &self.alloc;
    }

    /// Update element X allocation
    fn element_update_x_alloc(&mut self, x_alloc: &LAlloc) {
        self.alloc.x_alloc.clone_from(x_alloc);
    }
    /// Update element Y allocation
    fn element_update_y_alloc(&mut self, y_alloc: &LAlloc) {
        self.alloc.y_alloc.clone_from(y_alloc);
    }

    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        self.draw_self(cairo_ctx, visible_region);
        self.draw_children(cairo_ctx, visible_region);
    }


    fn update_x_req(&mut self) {
        self.update_children_x_req();
        self.req.x_req = self.children[0].get().x_req().clone();
    }

    fn allocate_x(&mut self) {
        self.children[0].get_mut().element_update_x_alloc(&self.alloc.x_alloc);
        self.allocate_children_x();
    }

    fn update_y_req(&mut self) {
        self.update_children_y_req();
        self.req.y_req = self.children[0].get().y_req().clone();
    }

    fn allocate_y(&mut self) {
        self.children[0].get_mut().element_update_y_alloc(&self.alloc.y_alloc);
        self.allocate_children_y();
    }
}

impl TContainerElement for RootElement {
    fn children<'a>(&'a self) -> &'a Vec<ElementRef> {
        return &self.children;
    }

    fn children_mut<'a>(&'a mut self) -> &'a mut Vec<ElementRef> {
        return &mut self.children;
    }
}

