use layout::lreq::LReq;
use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;

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
}


pub struct ElementAlloc {
    pub x_alloc: LAlloc,
    pub y_alloc: LAlloc,
    pub layout_flags: u8,
}

impl ElementAlloc {
    pub fn new() -> ElementAlloc {
        return ElementAlloc{x_alloc: LAlloc::new_empty(), y_alloc: LAlloc::new_empty(),
                            layout_flags: LAYOUT_FLAGS_ALL_CLEAN};
    }
}
