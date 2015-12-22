use layout::lreq::{LReq};
use layout::lalloc::{LAlloc};


pub struct LBox {
    pub x_req: LReq,
    pub y_req: LReq,
    pub x_alloc: LAlloc,
    pub y_alloc: LAlloc
}



impl LBox {
    pub fn new_empty() -> LBox {
        return LBox{x_req: LReq::new_empty(), y_req: LReq::new_empty(),
                    x_alloc: LAlloc::new_empty(), y_alloc: LAlloc::new_empty()};
    }

    pub fn new(x_req: LReq, y_req: LReq) -> LBox {
        return LBox{x_req: x_req, y_req: y_req,
                    x_alloc: LAlloc::new_empty(), y_alloc: LAlloc::new_empty()};
    }


    pub fn x_reqs<'a>(boxes: &'a [LBox]) -> Vec<&'a LReq> {
        return boxes.iter().map(|b| &b.x_req).collect();
    }

    pub fn y_reqs<'a>(boxes: &'a [LBox]) -> Vec<&'a LReq> {
        return boxes.iter().map(|b| &b.y_req).collect();
    }

    pub fn update_x_allocs(boxes: &mut [LBox], x_allocs: &[LAlloc]) {
        let n = boxes.len();
        assert!(n == x_allocs.len());
        for i in 0..n {
            boxes[i].x_alloc = x_allocs[i];
        }
    }

    pub fn update_y_allocs(boxes: &mut [LBox], y_allocs: &[LAlloc]) {
        let n = boxes.len();
        assert!(n == y_allocs.len());
        for i in 0..n {
            boxes[i].y_alloc = y_allocs[i];
        }
    }
}
