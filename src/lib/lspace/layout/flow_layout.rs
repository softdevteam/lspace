use layout::lreq::{LReq};
use layout::lalloc::{LAlloc};
use layout::lbox::{LBox};


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FlowIndent {
    NoIndent,
    First{indent: f64},
    ExceptFirst{indent: f64}
}


pub struct FlowLine {
    y_req: LReq,
    pos_x_in_parent: f64,
    pos_y_in_parent: f64,
    start: usize,
    end: usize
}

impl FlowLine {
    fn new_x(pos_x_in_parent: f64, start: usize, end: usize) -> FlowLine {
        return FlowLine{y_req: LReq::new_empty(),
                        pos_x_in_parent: pos_x_in_parent, pos_y_in_parent: 0.0,
                        start: start, end: end};
    }

}


pub fn requisition_x(child_x_reqs: &[&LReq], x_spacing: f64, indentation: FlowIndent) -> LReq {
    let (one_line, sep_lines) = match indentation {
        FlowIndent::NoIndent => {
            let one_line = LReq::linear_acc(child_x_reqs, x_spacing, None);
            let sep_lines = LReq::perpendicular_acc(child_x_reqs);
            (one_line, sep_lines)
        },

        FlowIndent::First{indent} => {
            let one_line = LReq::linear_acc(child_x_reqs, x_spacing, None);
            let indented : Vec<LReq> = child_x_reqs.iter().enumerate().map(|(i,ref x)| {
                    if i == 0 {
                        x.indent(indent)
                    } else {
                        ***x
                    }
                }).collect();
            let indented_refs : Vec<&LReq> = indented.iter().collect();
            let sep_lines = LReq::perpendicular_acc(&indented_refs);
            (one_line.indent(indent), sep_lines)
        },

        FlowIndent::ExceptFirst{indent} => {
            let indent_req = LReq::new_fixed_size(indent);
            let one_line = LReq::linear_acc(child_x_reqs, x_spacing, None);
            let indented : Vec<LReq> = child_x_reqs.iter().enumerate().map(|(i,ref x)| {
                    if i != 0 {
                        x.indent(indent)
                    } else {
                        ***x
                    }
                }).collect();
            let indented_refs : Vec<&LReq> = indented.iter().collect();
            let sep_lines = LReq::perpendicular_acc(&indented_refs);
            (one_line, sep_lines)
        }
    };

    let min_size = sep_lines.min_size();
    let preferred_size = one_line.size().size();

    return LReq::new_flex_size_min(preferred_size, min_size, one_line.flex().stretch());
}

pub fn alloc_x(box_x_req: &LReq, box_x_alloc: &LAlloc,
               child_reqs: &[&LReq], x_spacing: f64,
               indentation: FlowIndent) -> (Vec<LAlloc>, Vec<FlowLine>) {
    if box_x_req.size().size() <= box_x_alloc.alloc_size() {
        let (indent, line_req, line_alloc) = match indentation {
            FlowIndent::First{indent} => (indent, box_x_req.dedent(indent),
                                          box_x_alloc.indent(indent)),
            _ => (0.0, *box_x_req, *box_x_alloc)
        };
        let child_allocs = LAlloc::alloc_linear(child_reqs, box_x_req, line_alloc.pos_in_parent(),
                             line_alloc.alloc_size(), line_alloc.ref_point(), x_spacing, None);
        let line = FlowLine::new_x(indent, 0, child_reqs.len());
        return (child_allocs, vec![line]);
    } else {
        let n = child_reqs.len();
        let mut child_allocs = Vec::with_capacity(n);
        let mut lines : Vec<FlowLine> = Vec::new();
        let mut line_i_0 = 0;
        let mut line_x = match indentation {
            FlowIndent::First{indent} => indent,
            _ => 0.0
        };
        let mut x = line_x;
        let mut n_lines = 0;

        for i in 0..n {
            if i > line_i_0 {
                // Not the first element in the line
                x = x + x_spacing;
            }
            let child_w = child_reqs[i].size().size();
            x = x + child_w;

            if x > box_x_alloc.alloc_size() && i > line_i_0 {
                let line_alloc = box_x_alloc.indent(line_x);
                // Record the existing line
                lines.push(FlowLine::new_x(line_x, line_i_0, i));
                let line_child_allocs = LAlloc::alloc_linear
                (&child_reqs[line_i_0..i],
                                     &box_x_req.dedent(line_x), line_alloc.pos_in_parent(),
                                     line_alloc.alloc_size(), line_alloc.ref_point(),
                                     x_spacing, None);
                child_allocs.extend(line_child_allocs);
                n_lines = n_lines + 1;

                // Start a new line
                line_i_0 = i;
                line_x = match indentation {
                    FlowIndent::ExceptFirst{indent} => indent,
                    _ => 0.0
                };
                x = line_x + child_w;
            }
        }

        {
            let line_alloc = box_x_alloc.indent(line_x);

            lines.push(FlowLine::new_x(line_x, line_i_0, n));
            let line_child_allocs = LAlloc::alloc_linear(&child_reqs[line_i_0..n],
                                 &box_x_req.dedent(line_x), line_alloc.pos_in_parent(),
                                 line_alloc.alloc_size(), line_alloc.ref_point(), x_spacing, None);
            child_allocs.extend(line_child_allocs);
        }

        return (child_allocs, lines);
    }
}

pub fn requisition_y(child_y_reqs: &[&LReq], y_spacing: f64, lines: &mut Vec<FlowLine>) -> LReq {
    for i in 0..lines.len() {
        let line_y_req = {
            let line = &lines[i];
            LReq::perpendicular_acc(&child_y_reqs[line.start..line.end])
        };
        lines[i].y_req = line_y_req;
    }

    let line_y_reqs: Vec<&LReq> = lines.iter().map(|x| &x.y_req).collect();
    return LReq::linear_acc(&line_y_reqs, y_spacing, None);
}

pub fn alloc_y(box_y_req: &LReq, box_y_alloc: &LAlloc,
               child_reqs: &[&LReq], y_spacing: f64,
               lines: &mut Vec<FlowLine>) -> Vec<LAlloc> {
    // Allocate lines
    let line_y_allocs = {
        let line_y_reqs: Vec<&LReq> = lines.iter().map(|l| &l.y_req).collect();
        LAlloc::alloc_linear(&line_y_reqs, box_y_req,
            box_y_alloc.pos_in_parent(), box_y_alloc.alloc_size(),
            box_y_alloc.ref_point(), y_spacing, None)

    };

    // Allocate children
    let mut child_allocs = Vec::with_capacity(child_reqs.len());
    for l in 0..lines.len() {
        let line = &lines[l];
        let line_y_alloc = &line_y_allocs[l];
        for i in line.start..line.end {
            let child_alloc = LAlloc::alloced_from_region(child_reqs[i],
                                                line_y_alloc.pos_in_parent(),
                                                line_y_alloc.alloc_size(),
                                                line_y_alloc.ref_point());
            child_allocs.push(child_alloc);
        }
    }

    for l in 0..lines.len() {
        lines[l].pos_y_in_parent = line_y_allocs[l].pos_in_parent();
    }
    return child_allocs;
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


    fn f_layout(x_reqs: &Vec<&LReq>, y_reqs: &Vec<&LReq>, x_spacing: f64,
                y_spacing: f64, indentation: FlowIndent, width: f64) ->
                    (LReq, LReq, Vec<LAlloc>, Vec<LAlloc>, Vec<FlowLine>) {
        let n = x_reqs.len();
        assert_eq!(n, y_reqs.len());

        let box_x_req = requisition_x(x_reqs, x_spacing, indentation);
        let box_x_alloc = LAlloc::new(0.0, width, width);

        let mut allocs_and_lines = alloc_x(&box_x_req, &box_x_alloc, &x_reqs,
                                x_spacing, indentation);

        let box_y_req = {requisition_y(y_reqs, y_spacing, &mut allocs_and_lines.1)};
        let box_y_alloc = LAlloc::new_from_req(&box_y_req, 0.0);

        let y_allocs = {alloc_y(&box_y_req, &box_y_alloc, &y_reqs, y_spacing, &mut allocs_and_lines.1)};

        return (box_x_req, box_y_req, allocs_and_lines.0, y_allocs, allocs_and_lines.1);
    }

    #[test]
    fn test_flow_layout() {
        let ch0x = LReq::new_fixed_size(10.0);
        let ch1x = LReq::new_fixed_size(20.0);
        let ch2x = LReq::new_fixed_size(30.0);
        let ch0y = LReq::new_fixed_ref(6.0, 4.0);
        let ch1y = LReq::new_fixed_ref(8.0, 2.0);
        let ch2y = LReq::new_fixed_ref(5.0, 6.0);

        let (box_x_req, box_y_req, x_allocs, y_allocs, lines) = f_layout(
            &vec![&ch0x, &ch1x, &ch2x], &vec![&ch0y, &ch1y, &ch2y],
            0.0, 0.0, FlowIndent::NoIndent, 60.0);

        assert_eq!(box_x_req, LReq::new_flex_size(60.0, 30.0, 0.0));
        assert_eq!(box_y_req, LReq::new_fixed_size(14.0));

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].y_req, LReq::new_fixed_ref(8.0, 6.0));

        assert_eq!(x_allocs[0], LAlloc::new(0.0, 10.0, 10.0));
        assert_eq!(x_allocs[1], LAlloc::new(10.0, 20.0, 20.0));
        assert_eq!(x_allocs[2], LAlloc::new(30.0, 30.0, 30.0));

        assert_eq!(y_allocs[0], LAlloc::new_ref(2.0, 10.0, 10.0, 6.0));
        assert_eq!(y_allocs[1], LAlloc::new_ref(0.0, 10.0, 10.0, 8.0));
        assert_eq!(y_allocs[2], LAlloc::new_ref(3.0, 11.0, 11.0, 5.0));
    }

    #[test]
    fn test_flow_layout_large() {
        let n = 100;

        let x_reqs: Vec<LReq> = (0..n).map(|_| LReq::new_fixed_size(10.0)).collect();
        let y_reqs: Vec<LReq> = (0..n).map(|i| LReq::new_fixed_ref(6.0+(i%2) as f64,
                                                          3.0+(1-i%2) as f64)).collect();
        let x_req_refs: Vec<&LReq> = x_reqs.iter().collect();
        let y_req_refs: Vec<&LReq> = y_reqs.iter().collect();

        {
            let (box_x_req, box_y_req, x_allocs, y_allocs, lines) = f_layout(
                &x_req_refs, &y_req_refs,
                0.0, 0.0, FlowIndent::NoIndent, 1000.0);

            assert_eq!(lines.len(), 1);
            assert_eq!(lines[0].y_req, LReq::new_fixed_ref(7.0, 4.0));

            assert_eq!(box_x_req, LReq::new_flex_size(1000.0, 990.0, 0.0));
            assert_eq!(box_y_req, LReq::new_fixed_size(11.0));

            for i in 0..n {
                assert_eq!(x_allocs[i], LAlloc::new((i*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref((1-i%2) as f64, 10.0, 10.0, 6.0+(i%2) as f64));
            }
        }

        {
            let (box_x_req, box_y_req, x_allocs, y_allocs, lines) = f_layout(
                &x_req_refs, &y_req_refs,
                0.0, 0.0, FlowIndent::NoIndent, 500.0);

            assert_eq!(lines.len(), 2);
            assert_eq!(lines[0].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[1].y_req, LReq::new_fixed_ref(7.0, 4.0));

            assert_eq!(box_x_req, LReq::new_flex_size(1000.0, 990.0, 0.0));
            assert_eq!(box_y_req, LReq::new_fixed_size(22.0));

            for i in 0..50 {
                assert_eq!(x_allocs[i], LAlloc::new((i*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref((1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 50..n {
                assert_eq!(x_allocs[i], LAlloc::new(((i-50)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(11.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
        }

        {
            let (box_x_req, box_y_req, x_allocs, y_allocs, lines) = f_layout(
                &x_req_refs, &y_req_refs,
                0.0, 0.0, FlowIndent::NoIndent, 250.0);

            assert_eq!(lines.len(), 4);
            assert_eq!(lines[0].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[1].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[2].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[3].y_req, LReq::new_fixed_ref(7.0, 4.0));

            assert_eq!(box_x_req, LReq::new_flex_size(1000.0, 990.0, 0.0));
            assert_eq!(box_y_req, LReq::new_fixed_size(44.0));

            for i in 0..25 {
                assert_eq!(x_allocs[i], LAlloc::new((i*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref((1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 25..50 {
                assert_eq!(x_allocs[i], LAlloc::new(((i-25)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(11.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 50..75 {
                assert_eq!(x_allocs[i], LAlloc::new(((i-50)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(22.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 75..n {
                assert_eq!(x_allocs[i], LAlloc::new(((i-75)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(33.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
        }

        {
            let (box_x_req, box_y_req, x_allocs, y_allocs, lines) = f_layout(
                &x_req_refs, &y_req_refs,
                0.0, 0.0, FlowIndent::First{indent:15.0}, 300.0);

            assert_eq!(lines.len(), 4);
            assert_eq!(lines[0].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[1].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[2].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[3].y_req, LReq::new_fixed_ref(7.0, 4.0));

            assert_eq!(box_x_req, LReq::new_flex_size(1015.0, 990.0, 0.0));
            assert_eq!(box_y_req, LReq::new_fixed_size(44.0));

            for i in 0..28 {
                assert_eq!(x_allocs[i], LAlloc::new(15.0+(i*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref((1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 28..58 {
                assert_eq!(x_allocs[i], LAlloc::new(((i-28)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(11.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 58..88 {
                assert_eq!(x_allocs[i], LAlloc::new(((i-58)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(22.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 88..n {
                assert_eq!(x_allocs[i], LAlloc::new(((i-88)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(33.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
        }

        {
            let (box_x_req, box_y_req, x_allocs, y_allocs, lines) = f_layout(
                &x_req_refs, &y_req_refs,
                0.0, 0.0, FlowIndent::ExceptFirst{indent:15.0}, 300.0);

            assert_eq!(lines.len(), 4);
            assert_eq!(lines[0].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[1].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[2].y_req, LReq::new_fixed_ref(7.0, 4.0));
            assert_eq!(lines[3].y_req, LReq::new_fixed_ref(7.0, 4.0));

            assert_eq!(box_x_req, LReq::new_flex_size(1000.0, 975.0, 0.0));
            assert_eq!(box_y_req, LReq::new_fixed_size(44.0));

            for i in 0..30 {
                assert_eq!(x_allocs[i], LAlloc::new((i*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref((1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 30..58 {
                assert_eq!(x_allocs[i], LAlloc::new(15.0+((i-30)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(11.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 58..86 {
                assert_eq!(x_allocs[i], LAlloc::new(15.0+((i-58)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(22.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
            for i in 86..n {
                assert_eq!(x_allocs[i], LAlloc::new(15.0+((i-86)*10) as f64, 10.0, 10.0));
                assert_eq!(y_allocs[i], LAlloc::new_ref(33.0+(1-i%2) as f64, 10.0, 10.0,
                                                        6.0+(i%2) as f64));
            }
        }
    }

    #[bench]
    fn bench_flow_layout(bench: &mut test::Bencher) {
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

        for _ in 0..num_parents {
            parents.push(LBox::new_empty());
        }

        bench.iter(|| {
            for _ in 0..num_repeats {
                for i in 0..num_parents {
                    let mut x_allocs_and_lines: (Vec<LAlloc>, Vec<FlowLine>) = {
                        let child_x_req_refs = LBox::x_reqs(&children);
                        parents[i].x_req = requisition_x(&child_x_req_refs, 0.0, FlowIndent::NoIndent);
                        parents[i].x_alloc = LAlloc::new(0.0, 500.0, 500.0);


                        alloc_x(&parents[i].x_req, &parents[i].x_alloc,
                                &child_x_req_refs, 0.0, FlowIndent::NoIndent)
                    };
                    {
                        LBox::update_x_allocs(&mut children, &x_allocs_and_lines.0);
                    }

                    let y_allocs = {
                        let child_y_req_refs = LBox::y_reqs(&children);
                        parents[i].y_req = requisition_y(&child_y_req_refs, 0.0, &mut x_allocs_and_lines.1);
                        parents[i].y_alloc = LAlloc::new_from_req(&parents[i].y_req, 0.0);

                        alloc_y(&parents[i].y_req, &parents[i].y_alloc,
                                &child_y_req_refs, 0.0,
                                &mut x_allocs_and_lines.1)
                    };
                    {
                        LBox::update_y_allocs(&mut children, &y_allocs
                        );
                    }
                }
            }
        });
    }
}
