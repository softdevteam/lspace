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


    pub fn x_reqs<'a>(boxes: &'a mut [&'a mut LBox]) -> Vec<&'a LReq> {
        return boxes.iter().map(|b| &b.x_req).collect();
    }

    pub fn y_reqs<'a>(boxes: &'a mut [&'a mut LBox]) -> Vec<&'a LReq> {
        return boxes.iter().map(|b| &b.y_req).collect();
    }

    pub fn x_reqs_and_mut_allocs<'a>(boxes: &'a mut [&'a mut LBox]) -> (Vec<&'a LReq>, Vec<&'a mut LAlloc>) {
        let n = boxes.len();
        let mut allocs_and_reqs = boxes.iter_mut().map(|b| (&mut b.x_alloc, &b.x_req));
        let mut allocs : Vec<&mut LAlloc> = Vec::with_capacity(n);
        let mut reqs : Vec<&LReq> = Vec::with_capacity(n);

        for (mut alloc, req) in allocs_and_reqs {
            allocs.push(alloc);
            reqs.push(req);
        }

        return (reqs, allocs);
    }

    pub fn y_reqs_and_mut_allocs<'a>(boxes: &'a mut [&'a mut LBox]) -> (Vec<&'a LReq>, Vec<&'a mut LAlloc>) {
        let n = boxes.len();
        let mut allocs_and_reqs = boxes.iter_mut().map(|b| (&mut b.y_alloc, &b.y_req));
        let mut allocs : Vec<&mut LAlloc> = Vec::with_capacity(n);
        let mut reqs : Vec<&LReq> = Vec::with_capacity(n);

        for (mut alloc, req) in allocs_and_reqs {
            allocs.push(alloc);
            reqs.push(req);
        }

        return (reqs, allocs);
    }

    pub fn reqs_and_mut_allocs<'a>(boxes: &'a mut [&'a mut LBox]) ->
                (Vec<&'a LReq>, Vec<&'a mut LAlloc>, Vec<&'a LReq>, Vec<&'a mut LAlloc>) {
        let n = boxes.len();
        let mut everything = boxes.iter_mut().map(|b| (&mut b.x_alloc, &b.x_req, &mut b.y_alloc, &b.y_req));
        let mut x_allocs : Vec<&mut LAlloc> = Vec::with_capacity(n);
        let mut x_reqs : Vec<&LReq> = Vec::with_capacity(n);
        let mut y_allocs : Vec<&mut LAlloc> = Vec::with_capacity(n);
        let mut y_reqs : Vec<&LReq> = Vec::with_capacity(n);

        for (mut x_alloc, x_req, y_alloc, y_req) in everything {
            x_allocs.push(x_alloc);
            x_reqs.push(x_req);
            y_allocs.push(y_alloc);
            y_reqs.push(y_req);
        }

        return (x_reqs, x_allocs, y_reqs, y_allocs);
    }

}
