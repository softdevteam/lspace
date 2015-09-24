use layout::lreq::{LReq};
use layout::lalloc::{LAlloc};
use layout::lbox::{LBox};


pub fn requisition_x(child_x_reqs: &[&LReq], x_spacing: f64) -> LReq {
    return LReq::linear_acc(child_x_reqs, x_spacing, None);
}

pub fn alloc_x(box_x_req: &LReq, box_x_alloc: &LAlloc,
               child_reqs_and_allocs: &mut [(&LReq, &mut LAlloc)], x_spacing: f64) {
    LAlloc::alloc_linear(child_reqs_and_allocs, box_x_req, box_x_alloc.pos_in_parent(),
                         box_x_alloc.alloc_size(), box_x_alloc.ref_point(), x_spacing, None);
}

pub fn requisition_y(child_y_reqs: &[&LReq]) -> LReq {
    return LReq::perpendicular_acc(child_y_reqs);
}

pub fn alloc_y(box_y_req: &LReq, box_y_alloc: &LAlloc,
               child_reqs_and_allocs: &mut [(&LReq, &mut LAlloc)]) {
    for i in 0..child_reqs_and_allocs.len() {
        child_reqs_and_allocs[i].1.alloc_from_region(child_reqs_and_allocs[i].0, box_y_alloc.pos_in_parent(),
                                            box_y_alloc.alloc_size(), box_y_alloc.ref_point());
    }
}



#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate test;

    use std::mem;
    use self::rand::distributions::{Range, IndependentSample};
    use super::*;

    use layout::lreq::{LNatSize, LFlex, LReq};
    use layout::lalloc::{LAlloc};
    use layout::lbox::{LBox};


    fn h_layout(x_reqs: &Vec<&LReq>, y_reqs: &Vec<&LReq>, x_spacing: f64,
                x_pos: f64, y_pos: f64) ->
            (LReq, LReq, Vec<LAlloc>, Vec<LAlloc>) {
        let n = x_reqs.len();
        assert_eq!(n, y_reqs.len());

        let mut x_allocs : Vec<LAlloc> = (0..n).map(|_| LAlloc::new_empty()).collect();
        let mut y_allocs : Vec<LAlloc> = (0..n).map(|_| LAlloc::new_empty()).collect();

        let (box_x_req, box_y_req) = {
            let mut x_pairs: Vec<(&LReq, &mut LAlloc)> = Vec::new();
            let mut y_pairs: Vec<(&LReq, &mut LAlloc)> = Vec::new();
            for pair in x_reqs.iter().zip(x_allocs.iter_mut()) {
                x_pairs.push((*pair.0, pair.1));
            }
            for pair in y_reqs.iter().zip(y_allocs.iter_mut()) {
                y_pairs.push((*pair.0, pair.1));
            }

            let box_x_req = requisition_x(x_reqs, x_spacing);
            let box_x_alloc = LAlloc::new_from_req(&box_x_req, x_pos);

            alloc_x(&box_x_req, &box_x_alloc, &mut x_pairs, x_spacing);

            let box_y_req = requisition_y(y_reqs);
            let box_y_alloc = LAlloc::new_from_req(&box_y_req, y_pos);

            alloc_y(&box_y_req, &box_y_alloc, &mut y_pairs);

            (box_x_req, box_y_req)
        };

        return (box_x_req, box_y_req, x_allocs, y_allocs);
    }

    #[test]
    fn test_horizontal_layout() {
        let ch0x = LReq::new_fixed_size(10.0);
        let ch1x = LReq::new_fixed_size(20.0);
        let ch2x = LReq::new_fixed_size(30.0);
        let ch0y = LReq::new_fixed_ref(6.0, 4.0);
        let ch1y = LReq::new_fixed_ref(8.0, 2.0);
        let ch2y = LReq::new_fixed_ref(5.0, 6.0);

        let (box_x_req, box_y_req, x_allocs, y_allocs) = h_layout(
            &vec![&ch0x, &ch1x, &ch2x], &vec![&ch0y, &ch1y, &ch2y], 0.0, 100.0, 200.0);

        assert_eq!(box_x_req, LReq::new_fixed_size(60.0));
        assert_eq!(box_y_req, LReq::new_fixed_ref(8.0, 6.0));

        assert_eq!(x_allocs[0], LAlloc::new(100.0, 10.0, 10.0));
        assert_eq!(x_allocs[1], LAlloc::new(110.0, 20.0, 20.0));
        assert_eq!(x_allocs[2], LAlloc::new(130.0, 30.0, 30.0));

        assert_eq!(y_allocs[0], LAlloc::new_ref(202.0, 10.0, 10.0, 6.0));
        assert_eq!(y_allocs[1], LAlloc::new_ref(200.0, 10.0, 10.0, 8.0));
        assert_eq!(y_allocs[2], LAlloc::new_ref(203.0, 11.0, 11.0, 5.0));
    }

    #[bench]
    fn bench_horizontal_layout(bench: &mut test::Bencher) {
        let num_children = 100;
        let num_parents = 100;
        let num_repeats = 100;

        let natsize_type_range: Range<i32> = Range::new(0, 8);
        let size_range = Range::new(5.0, 25.0);
        let flex_type_range: Range<i32> = Range::new(0, 2);
        let flex_range = Range::new(1.0, 3.0);
        let mut rng = rand::thread_rng();

        let mut children = Vec::with_capacity(num_children);
        let mut parents = Vec::with_capacity(num_parents);

        for _ in 0..num_children {
            let size_x = LNatSize::new_size(size_range.ind_sample(&mut rng));
            let size_y = LNatSize::new_ref(size_range.ind_sample(&mut rng) * 0.5,
                                     size_range.ind_sample(&mut rng) * 0.5);
            let flex_x = match flex_type_range.ind_sample(&mut rng) {
                0 => LFlex::new_fixed(),
                1 => LFlex::new_flex(flex_range.ind_sample(&mut rng),
                                     0.0),
                _ => {panic!();},
            };
            let flex_y = LFlex::new_fixed();
            children.push(LBox::new(LReq::new(size_x, flex_x), LReq::new(size_y, flex_y)))
        }

        let mut child_refs : Vec<&mut LBox> = children.iter_mut().collect();
        let (child_x_req_refs, mut child_x_alloc_and_req_refs, child_y_req_refs, mut child_y_alloc_and_req_refs) =
                LBox::reqs_and_mut_allocs(&mut child_refs);

        for _ in 0..num_parents {
            parents.push(LBox::new_empty());
        }

        bench.iter(|| {
            for _ in 0..num_repeats {
                for i in 0..num_parents {
                    parents[i].x_req = requisition_x(&child_x_req_refs, 0.0);
                    parents[i].x_alloc = LAlloc::new_from_req(&parents[i].x_req, 0.0);

                    alloc_x(&parents[i].x_req, &parents[i].x_alloc,
                            &mut child_x_alloc_and_req_refs, 0.0);

                    parents[i].y_req = requisition_y(&child_y_req_refs);
                    parents[i].y_alloc = LAlloc::new_from_req(&parents[i].y_req, 0.0);

                    alloc_y(&parents[i].y_req, &parents[i].y_alloc,
                            &mut child_y_alloc_and_req_refs);
                }
            }
        });
    }
}