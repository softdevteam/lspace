use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use geom::point2::Point2;
use geom::vector2::Vector2;
use geom::bbox2::BBox2;

use elements::element::{TElement, ElementRef};

const LAYOUT_FLAG_X_REQ_DIRTY: u8       = 0b0001;
const LAYOUT_FLAG_Y_REQ_DIRTY: u8       = 0b0010;
const LAYOUT_FLAG_X_ALLOC_DIRTY: u8     = 0b0100;
const LAYOUT_FLAG_Y_ALLOC_DIRTY: u8     = 0b1000;
const LAYOUT_FLAGS_ALL_DIRTY: u8        = 0b1111;
const LAYOUT_FLAGS_ALL_CLEAN: u8        = 0b0000;


pub struct ElementReq {
    pub x_req: LReq,
    pub y_req: LReq,
}

impl ElementReq {
    pub fn new() -> ElementReq {
        return ElementReq{x_req: LReq::new_empty(), y_req: LReq::new_empty()};
    }

    pub fn new_from_reqs(x_req: LReq, y_req: LReq) -> ElementReq {
        return ElementReq{x_req: x_req, y_req: y_req};
    }


    /// Update element X allocation
    pub fn update_x_req(&mut self, x_req: &LReq) -> bool {
        let changed: bool = *x_req != self.x_req;
        self.x_req.clone_from(x_req);
        return changed;
    }

    /// Update element Y allocation
    pub fn update_y_req(&mut self, y_req: &LReq) -> bool {
        let changed: bool = *y_req != self.y_req;
        self.y_req.clone_from(y_req);
        return changed;
    }
}


pub struct ElementAlloc {
    pub x_alloc: LAlloc,
    pub y_alloc: LAlloc,
    pub layout_flags: u8,
}

impl ElementAlloc {
    pub fn new() -> ElementAlloc {
        return ElementAlloc{x_alloc: LAlloc::new_empty(), y_alloc: LAlloc::new_empty(),
                            layout_flags: LAYOUT_FLAGS_ALL_DIRTY};
    }

    /// Update element X allocation
    pub fn update_x_alloc(&mut self, x_alloc: &LAlloc) -> bool {
        let changed: bool = *x_alloc != self.x_alloc;
        self.x_alloc.clone_from(x_alloc);
        return changed;
    }

    /// Update element Y allocation
    pub fn update_y_alloc(&mut self, y_alloc: &LAlloc) -> bool {
        let changed: bool = *y_alloc != self.y_alloc;
        self.y_alloc.clone_from(y_alloc);
        return changed;
    }

    // Check if updates required
    pub fn is_x_req_update_required(&self) -> bool {
        return self.layout_flags & LAYOUT_FLAG_X_REQ_DIRTY != 0;
    }

    pub fn is_y_req_update_required(&self) -> bool {
        return self.layout_flags & LAYOUT_FLAG_Y_REQ_DIRTY != 0;
    }

    pub fn is_x_alloc_update_required(&self) -> bool {
        return self.layout_flags & LAYOUT_FLAG_X_ALLOC_DIRTY != 0;
    }

    pub fn is_y_alloc_update_required(&self) -> bool {
        return self.layout_flags & LAYOUT_FLAG_Y_ALLOC_DIRTY != 0;
    }

    // Clear and set dirty flags
    pub fn x_req_updated(&mut self) {
        self.layout_flags = self.layout_flags & !LAYOUT_FLAG_X_REQ_DIRTY;
    }

    pub fn y_req_updated(&mut self) {
        self.layout_flags = self.layout_flags & !LAYOUT_FLAG_Y_REQ_DIRTY;
    }

    pub fn x_alloc_updated(&mut self) {
        self.layout_flags = self.layout_flags & !LAYOUT_FLAG_X_ALLOC_DIRTY;
    }

    pub fn y_alloc_updated(&mut self) {
        self.layout_flags = self.layout_flags & !LAYOUT_FLAG_Y_ALLOC_DIRTY;
    }

    pub fn x_req_dirty(&mut self) {
        self.layout_flags = self.layout_flags | LAYOUT_FLAG_X_REQ_DIRTY;
    }

    pub fn y_req_dirty(&mut self) {
        self.layout_flags = self.layout_flags | LAYOUT_FLAG_Y_REQ_DIRTY;
    }

    pub fn x_alloc_dirty(&mut self) {
        self.layout_flags = self.layout_flags | LAYOUT_FLAG_X_ALLOC_DIRTY;
    }

    pub fn y_alloc_dirty(&mut self) {
        self.layout_flags = self.layout_flags | LAYOUT_FLAG_Y_ALLOC_DIRTY;
    }

    // Modifications
    pub fn notify_requisition_changed(elem: &TElement) {
        ElementAlloc::set_flags_on_path_to_root(elem, LAYOUT_FLAG_X_REQ_DIRTY | LAYOUT_FLAG_Y_REQ_DIRTY);
    }

    fn set_flags_on_path_to_root(elem: &TElement, flags: u8) {
        let mut alloc = elem.element_alloc_mut();
        alloc.layout_flags = alloc.layout_flags | flags;

        let mut p_ref: Option<ElementRef> = elem.get_parent();
        while p_ref.is_some() {
            let p = p_ref.unwrap();
            let mut p_alloc = p.element_alloc_mut();
            p_alloc.layout_flags = alloc.layout_flags | flags;

            p_ref = p.get_parent();
        }
    }

    pub fn local_bbox(&self) -> BBox2 {
        BBox2::from_lower_size(Point2::origin(),
                               Vector2::new(self.x_alloc.actual_size(), self.y_alloc.actual_size()))
    }

    pub fn local_bbox_to_parent_space(&self, bbox: &BBox2) -> BBox2 {
        bbox.offset(&Vector2::new(self.x_alloc.pos_in_parent(), self.y_alloc.pos_in_parent()))
    }
}
