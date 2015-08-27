use layout::lreq::{LNatSize, LFlex, LReq, fast_min, fast_max};

pub const EPSILON : f64 = 1.0e-6;
pub const ONE_MINUS_EPSILON : f64 = 1.0 - EPSILON;
pub const ONE_PLUS_EPSILON : f64 = 1.0 + EPSILON;


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LAlloc {
    pos_in_parent: f64,
    alloc_size: f64,
    actual_size: f64,
    ref_point: Option<f64>
}


/// A representation of a space allocation; space that has been allocated to an element.
impl LAlloc {
    /// Construct an empty `LAlloc`.
    pub fn new_empty() -> LAlloc {
        return LAlloc{pos_in_parent: 0.0, alloc_size: 0.0, actual_size: 0.0, ref_point: None};
    }

    /// Consruct a `LAlloc`
    pub fn new(pos_in_parent: f64, alloc_size: f64, actual_size: f64) -> LAlloc {
        return LAlloc{pos_in_parent: pos_in_parent, alloc_size: alloc_size,
                      actual_size: actual_size, ref_point: None};
    }

    /// Consruct a `LAlloc`
    pub fn new_ref(pos_in_parent: f64, alloc_size: f64, actual_size: f64,
                   ref_point: f64) -> LAlloc {
        return LAlloc{pos_in_parent: pos_in_parent, alloc_size: alloc_size,
                      actual_size: actual_size, ref_point: Some(ref_point)};
    }


    /// Get position of this element relative to parent space.
    pub fn pos_in_parent(&self) -> f64 {
        return self.pos_in_parent;
    }

    /// The the amount of space allocated to this element.
    pub fn alloc_size(&self) -> f64 {
        return self.alloc_size;
    }

    /// The the element's actual size; in cases where not enough space was available and this
    /// element was allocated less than it requires, the actual size is the space required to
    /// display it; use this for building bounding boxes for the purpose of event processing
    /// and culling during rendering.
    pub fn actual_size(&self) -> f64 {
        return self.actual_size;
    }

    /// Get the optional reference point position, as an offset from the position relative
    /// to parent space
    pub fn ref_point(&self) -> Option<f64> {
        return self.ref_point;
    }


    /// Private helper: update all fields, setting the reference point to `None`
    fn update(&mut self, pos_in_parent: f64, alloc_size: f64, actual_size: f64) {
        self.pos_in_parent = pos_in_parent;
        self.alloc_size = alloc_size;
        self.actual_size = actual_size;
        self.ref_point = None;
    }

    /// Private helper: update all fields, setting the reference point to `Some(ref_point)`
    fn update_ref(&mut self, pos_in_parent: f64, alloc_size: f64, actual_size: f64,
                  ref_point: f64) {
        self.pos_in_parent = pos_in_parent;
        self.alloc_size = alloc_size;
        self.actual_size = actual_size;
        self.ref_point = Some(ref_point);
    }

    /// Allocate this element - a child element - space in the specified region that would be
    /// contained within the space of a parent element.
    ///
    /// Allocates the element represented by `self` space from the given region.
    ///
    /// Parameters:
    /// `child_req` : the space requirements of the element represented by `self`; an instance of `LReq`
    /// `region_pos` : the position of the region in the parent space; most elements that have
    /// multiple children will want to give them different positions
    /// `region_size` : the size of the region
    /// `region_ref` : the reference point of the region (optional)
    pub fn alloc_from_region(&mut self, child_req: &LReq, region_pos: f64, region_size: f64,
                             region_ref: Option<f64>) {
        match child_req.size().before_and_after_ref_opt() {
            Some((req_before, req_after)) => {
                // Both the allocation region and the requisition have a reference point

                // If the region's reference point is not aligned with the required
                // reference point, compute the offset
                let (ref_offset, alloc_before) = match region_ref {
                    Some(avail_ref) => (fast_max(avail_ref - req_before, 0.0), avail_ref),
                    None => (0.0, req_before),
                };

                // Compute the natural and minimum sizes and end points
                // End points are the sizes that include the offset that aligns the required
                // reference point with the region reference point
                let natural_size = child_req.size().size();
                let minimum_size = natural_size - child_req.flex().shrink();
                let natural_end = natural_size + ref_offset;
                let minimum_end = minimum_size + ref_offset;

                if region_size < minimum_end * ONE_MINUS_EPSILON {
                    // Insufficient space
                    self.update_ref(ref_offset + region_pos, region_size - ref_offset,
                                    minimum_size, alloc_before - ref_offset);
                } else if region_size > natural_end * ONE_PLUS_EPSILON {
                    // More space than natural size
                    if child_req.flex().stretch() > 0.0 {
                        // Stretch to take up additional space
                        self.update_ref(region_pos, region_size, region_size, alloc_before);
                    } else {
                        // No stretch; don't take up additional space
                        self.update_ref(ref_offset + region_pos, natural_size, natural_size,
                                        alloc_before - ref_offset);
                    }
                } else {
                    // region_size is between minimum and natural end
                    let alloc_size = region_size - ref_offset;
                    self.update_ref(ref_offset + region_pos, alloc_size, alloc_size,
                                    alloc_before - ref_offset);
                }
            },

            None => {
                // No reference point required
                let natural_size = child_req.size().size();
                let minimum_size = natural_size - child_req.flex().shrink();

                if region_size < minimum_size * ONE_MINUS_EPSILON {
                    // Insufficient space; allocate the available space, while giving
                    // `minimum_size` as the actual size for bounding box and rendering purposes
                    self.update(region_pos, region_size, minimum_size);
                } else if region_size < natural_size * ONE_PLUS_EPSILON ||
                          child_req.flex().stretch() > 0.0 {
                    // Either:
                    // - region_size is between minimum and natural size
                    // or
                    // - region_size is > natural size and the box can stretch to use the
                    // additional space
                    //
                    // Allocate all the available space
                    self.update(region_pos, region_size, region_size);
                } else {
                    // region_size is > natural and no stretch; don't take up additional space
                    self.update(region_pos, natural_size, natural_size);
                }
            }
        }
    }


    fn _alloc_children_linear(n: usize,
                              child_reqs: &[&LReq], child_allocs: &mut [&mut LAlloc],
                              start_pos: f64, space_between: f64) -> f64 {
        let mut pos = start_pos;
        for i in 0..n {
            let creq = child_reqs[i];
            let csz = creq.size().size();
            child_allocs[i].alloc_from_region(creq, pos, csz, creq.size().before_ref_opt());
            pos = pos + csz + space_between;
        }
        return pos;
    }

    fn _alloc_children_linear_shrink(n: usize,
                            child_reqs: &[&LReq], child_allocs: &mut [&mut LAlloc],
                            start_pos: f64, space_between: f64, shrink_frac: f64) -> f64 {
        let mut pos = start_pos;
        for i in 0..n {
            let creq = child_reqs[i];
            let csz = creq.size().size() - creq.flex().shrink() * shrink_frac;

            child_allocs[i].alloc_from_region(creq, pos, csz, creq.size().before_ref_opt());
            pos = pos + csz + space_between;
        }
        return pos;
    }

    fn _alloc_children_linear_stretch(n: usize,
                            child_reqs: &[&LReq], child_allocs: &mut [&mut LAlloc],
                            start_pos: f64, space_between: f64, stretch_factor: f64) -> f64 {
        // `stretch_factor = stretch_space / stretch_sum`
        // where `stretch_space` is the additional space available over the natural size
        // and `stretch_sum` is the sum of all the stretch factors from the child
        // requisitions
        // multiplying `stretch_factor` by req.size().stretch() will compute the extra space
        // to alloate to that child
        let mut pos = start_pos;
        for i in 0..n {
            let creq = child_reqs[i];
            let cstretch = (creq.flex().stretch() as f64) * stretch_factor;
            let sz = creq.size().size();
            let csz = sz + cstretch;
            let cref = match creq.size().before_ref_opt() {
                None => None,
                Some(before_ref) => Some(before_ref + cstretch * before_ref / sz),
            };

            child_allocs[i].alloc_from_region(creq, pos, csz, cref);
            pos = pos + csz + space_between;
        }
        return pos;
    }

    fn _alloc_children(n: usize, child_reqs: &[&LReq], child_allocs: &mut [&mut LAlloc],
                       natural_size: f64, minimum_size: f64, region_pos: f64, region_size: f64,
                       region_stretch: f32, space_between: f64, pad_start: bool) -> f64 {
        if region_size < minimum_size * ONE_MINUS_EPSILON {
            // Insufficient space; allocate minimum space; shrink_frac=1.0
            return LAlloc::_alloc_children_linear_shrink(n, child_reqs, child_allocs,
                region_pos, space_between, 1.0);
        } else if region_size < natural_size * ONE_MINUS_EPSILON {
            // Region_size is between minimum and natural size
            let shrink_frac = (region_size - natural_size) /
                    (minimum_size - natural_size);
            return LAlloc::_alloc_children_linear_shrink(n, child_reqs, child_allocs,
                region_pos, space_between, shrink_frac);
        } else if region_size > natural_size * ONE_PLUS_EPSILON {
            let stretch_space = region_size - natural_size;

            if region_stretch > 0.0 {
                // Region size is greater than the natural size and stretch > 0
                let stretch_factor = stretch_space / region_stretch as f64;
                return LAlloc::_alloc_children_linear_stretch(n, child_reqs, child_allocs,
                    region_pos, space_between, stretch_factor);
            } else {
                let start = if pad_start {region_pos + stretch_space} else {region_pos};
                // Region size matches natural size
                return LAlloc::_alloc_children_linear(n, child_reqs, child_allocs, start,
                    space_between);
            }
        } else {
            // Region size matches natural size
            return LAlloc::_alloc_children_linear(n, child_reqs, child_allocs, region_pos,
                space_between);
        }
    }

    /// Allocate space to an array of child elements that are aligned along the axis represented
    /// by the requisitions and allocations.
    ///
    /// Parameters:
    /// 'child_reqs' : a vector containing an `LReq` for each child element
    /// 'child_allocs' : a vector containing an `LAlloc` for each child element
    /// 'region_req' : the `LReq` for the region within the parent element
    /// `region_pos`, `region_size`, `region_ref` : the position, size and optional reference
    /// point that describe the region in which the child elements are to be positioned
    /// `space_between` : the amount of space to insert between child elements
    /// `ref_point_index` : an optional index that identifies a child element that is to have its
    /// reference point aligned with that of the region
    pub fn alloc_linear(child_reqs: &Vec<&LReq>, child_allocs: &mut Vec<&mut LAlloc>,
                        region_req: &LReq,
                        region_pos: f64, region_size: f64, region_ref: Option<f64>,
                        space_between: f64, ref_point_index: Option<usize>) {
        let n = child_reqs.len();
        assert_eq!(n, child_allocs.len());

        // There are a few valid, consistent configurations that are identified and handled
        match (ref_point_index, region_req.size(), region_ref) {
            (None, &LNatSize::Size{..}, None) => {
                // NO REFERENCE POINTS
                match region_req.flex() {
                    &LFlex::Fixed => {
                        LAlloc::_alloc_children_linear(n, child_reqs, child_allocs, region_pos,
                            space_between);
                    },
                    &LFlex::Flex{..} => {
                        let natural_size = region_req.size().size();
                        let minimum_size = natural_size - region_req.flex().shrink();

                        LAlloc::_alloc_children(n, &child_reqs[..],
                                                &mut child_allocs[..],
                                                natural_size, minimum_size,
                                                region_pos, region_size,
                                                region_req.flex().stretch(),
                                                space_between, false);
                    },
                }
            },

            (Some(ref_point_n), &LNatSize::Ref{..}, Some(region_before_ref)) => {
                assert!(ref_point_n < n);
                // REFERENCE POINT INDEX SPECIFIED
                // REQUISITION HAS REF POINT
                // REGION HAS REF POINT
                match region_req.flex() {
                    &LFlex::Fixed => {
                        // Compute the offset required to position the array of children so that
                        // the required reference point aligns with the allocated reference point.
                        // Assuming that the region requisition has its reference point
                        // aligned with that of the correct child, this should be fine...
                        let before_offset = fast_max(
                            region_before_ref - region_req.size().before_ref(), 0.0);
                        LAlloc::_alloc_children_linear(n, &child_reqs[..],
                            &mut child_allocs[..],
                            before_offset + region_pos, space_between);
                    },
                    &LFlex::Flex{..} => {
                        let req_at_ref_n = child_reqs[ref_point_n];

                        let ref_n_shrink = req_at_ref_n.flex().shrink();
                        let ref_n_stretch = req_at_ref_n.flex().stretch();

                        // Before reference point index
                        let (shrink_before_ref, stretch_before_ref, nat_size_before_ref,
                                    min_size_before_ref, region_size_before_ref) = {
                            let reqs = &child_reqs[..ref_point_n];

                            let shrink = reqs.iter().fold(0.0, |acc, x| acc + x.flex().shrink()) + ref_n_shrink * 0.5;
                            let stretch = reqs.iter().fold(0.0, |acc, x| acc + x.flex().stretch()) + ref_n_stretch * 0.5;;

                            let natural_size = region_req.size().before_ref();
                            let minimum_size = natural_size - shrink;
                            let avail_size = region_before_ref - req_at_ref_n.size().before_ref() - space_between;

                            (shrink, stretch, natural_size, minimum_size, avail_size)
                        };

                        // After reference point index
                        let (shrink_after_ref, stretch_after_ref, nat_size_after_ref,
                                    min_size_after_ref, region_size_after_ref) = {
                            let reqs = &child_reqs[ref_point_n+1..];

                            let shrink = reqs.iter().fold(0.0, |acc, x| acc + x.flex().shrink()) + ref_n_shrink * 0.5;
                            let stretch = reqs.iter().fold(0.0, |acc, x| acc + x.flex().stretch()) + ref_n_stretch * 0.5;;

                            let natural_size = region_req.size().after_ref();
                            let minimum_size = natural_size - shrink;
                            let avail_size = (region_size - region_before_ref) - req_at_ref_n.size().after_ref() - space_between;

                            (shrink, stretch, natural_size, minimum_size, region_size)
                        };

                        // Detect the case where the available space before the ref point
                        // is less than the minimumn space before the ref point
                        if region_size_before_ref < min_size_before_ref * ONE_MINUS_EPSILON {
                            let before_ref_overflow = min_size_before_ref - region_size_before_ref;

                        }

                        // let mut pos = region_pos;
                        // pos = LAlloc::_alloc_children(n, req_before_ref_n,
                        //     allocs_before_ref_n,
                        //     natural_size, minimum_size, pos, region_size,
                        //     stretch, space_between);

                        panic!("Not implemented");
                    },
                }
            },

            // EMPTY PARENT REQUISITIONS
            (_, &LNatSize::Empty, None) => {
                // Ensure children are all empty
                for (i, ch) in child_reqs.iter().enumerate() {
                    if ch.size() != &LNatSize::Empty {
                        panic!("Empty parent requisition, non-empty child requisiton at \
                               index {}", i);
                    }
                }

                // Nothing to do
            },

            (_, &LNatSize::Empty, _) => {
                panic!("Empty parent requisition but region allocation has reference point");
            }

            // HANDLE INCONSISTENT CONFIGURATIONS BY IDENTIFYING THEM AND PANICKING
            (_, _, _) => {
                let a = match ref_point_index {
                    None => "NOT PRESENT",
                    Some(_) => "PRESENT",
                };
                let b = match region_req.size() {
                    &LNatSize::Empty => "NOT PRESENT",
                    &LNatSize::Size{..} => "NOT PRESENT",
                    &LNatSize::Ref{..} => "PRESENT",
                };
                let c = match region_ref {
                    None => "NOT PRESENT",
                    Some(_) => "PRESENT",
                };
                panic!("INCONSISTENT CONFIGURATION: ref point index {}, region requisition ref \
                        point {}, region allocation ref point {}", a, b, c);
            },
        };
    }
}


//
// TESTS
//

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate test;

    use std::mem;
    use self::rand::distributions::{Range, IndependentSample};
    use super::*;
    use layout::lreq::{LNatSize, LFlex, LReq};


    //
    //
    // LAlloc
    //
    //
    #[test]
    fn test_lalloc_new_and_accessors() {
        let a = LAlloc::new_empty();
        assert_eq!(a.pos_in_parent(), 0.0);
        assert_eq!(a.alloc_size(), 0.0);
        assert_eq!(a.actual_size(), 0.0);
        assert_eq!(a.ref_point(), None);

        let b = LAlloc::new(1.0, 10.0, 20.0);
        assert_eq!(b.pos_in_parent(), 1.0);
        assert_eq!(b.alloc_size(), 10.0);
        assert_eq!(b.actual_size(), 20.0);
        assert_eq!(b.ref_point(), None);

        let c = LAlloc::new_ref(1.0, 10.0, 20.0, 5.0);
        assert_eq!(c.pos_in_parent(), 1.0);
        assert_eq!(c.alloc_size(), 10.0);
        assert_eq!(c.actual_size(), 20.0);
        assert_eq!(c.ref_point(), Some(5.0));
    }

    #[test]
    fn test_lalloc_update() {
        let mut a = LAlloc::new_empty();
        a.update(1.0, 10.0, 20.0);
        assert_eq!(a.pos_in_parent(), 1.0);
        assert_eq!(a.alloc_size(), 10.0);
        assert_eq!(a.actual_size(), 20.0);
        assert_eq!(a.ref_point(), None);

        let mut b = LAlloc::new_empty();
        b.update_ref(1.0, 10.0, 20.0, 5.0);
        assert_eq!(b.pos_in_parent(), 1.0);
        assert_eq!(b.alloc_size(), 10.0);
        assert_eq!(b.actual_size(), 20.0);
        assert_eq!(b.ref_point(), Some(5.0));
    }

    fn _alloc_region(req: &LReq, region_pos: f64, region_size: f64) -> LAlloc {
        let mut a = LAlloc::new_empty();
        a.alloc_from_region(req, region_pos, region_size, None);
        return a;
    }

    fn _alloc_region_ref(req: &LReq, region_pos: f64, region_size: f64,
                         region_ref: f64) -> LAlloc {
        let mut a = LAlloc::new_empty();
        a.alloc_from_region(req, region_pos, region_size, Some(region_ref));
        return a;
    }

    #[test]
    fn test_alloc_from_region() {
        //
        // FIXED SIZE
        //
        // Fixed size, too small
        assert_eq!(_alloc_region(
            &LReq::new_fixed_size(10.0), 0.0, 5.0),
            LAlloc::new(0.0, 5.0, 10.0));
        // Fixed size, too small with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_size(10.0), 0.0, 5.0, 2.5),
            LAlloc::new(0.0, 5.0, 10.0));

        // Fixed size, natural size
        assert_eq!(_alloc_region(
            &LReq::new_fixed_size(10.0), 0.0, 10.0),
            LAlloc::new(0.0, 10.0, 10.0));
        // Fixed size, natural size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_size(10.0), 0.0, 10.0, 6.0),
            LAlloc::new(0.0, 10.0, 10.0));

        // Fixed size, extra space
        assert_eq!(_alloc_region(
            &LReq::new_fixed_size(10.0), 0.0, 20.0),
            LAlloc::new(0.0, 10.0, 10.0));
        // Fixed size, extra space with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_size(10.0), 0.0, 20.0, 12.0),
            LAlloc::new(0.0, 10.0, 10.0));

        //
        // FLEXIBLE SIZE WITH SHRINK
        //
        // Flexible size with shrink, too small
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 4.0),
            LAlloc::new(0.0, 4.0, 5.0));
        // Flexible size with shrink, too small with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 4.0, 2.5),
            LAlloc::new(0.0, 4.0, 5.0));

        // Flexible size with shrink, minimum size
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 5.0),
            LAlloc::new(0.0, 5.0, 5.0));
        // Flexible size with shrink, minimum size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 5.0, 3.0),
            LAlloc::new(0.0, 5.0, 5.0));

        // Flexible size with shrink, between minimum and natural size
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 7.0),
            LAlloc::new(0.0, 7.0, 7.0));
        // Flexible size with shrink, between minimum and natural size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 7.0, 4.0),
            LAlloc::new(0.0, 7.0, 7.0));

        // Flexible size with shrink, natural size
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 10.0),
            LAlloc::new(0.0, 10.0, 10.0));
        // Flexible size with shrink, natural size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 10.0, 6.0),
            LAlloc::new(0.0, 10.0, 10.0));

        // Flexible size with shrink, extra space
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 20.0),
            LAlloc::new(0.0, 10.0, 10.0));
        // Flexible size with shrink, extra space with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 0.0), 0.0, 20.0, 12.0),
            LAlloc::new(0.0, 10.0, 10.0));

        //
        // FLEXIBLE SIZE WITH (ARBITRARY) STRETCH
        //
        // Flexible size with stretch, too small
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 0.0, 3.14), 0.0, 5.0),
            LAlloc::new(0.0, 5.0, 10.0));
        // Flexible size with stretch, too small with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 0.0, 3.14), 0.0, 5.0, 2.5),
            LAlloc::new(0.0, 5.0, 10.0));

        // Flexible size with stretch, natural size
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 0.0, 3.14), 0.0, 10.0),
            LAlloc::new(0.0, 10.0, 10.0));
        // Flexible size with stretch, natural size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 0.0, 3.14), 0.0, 10.0, 6.0),
            LAlloc::new(0.0, 10.0, 10.0));

        // Flexible size with stretch, extra space
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 0.0, 3.14), 0.0, 20.0),
            LAlloc::new(0.0, 20.0, 20.0));
        // Flexible size with stretch, extra space with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 0.0, 3.14), 0.0, 20.0, 12.0),
            LAlloc::new(0.0, 20.0, 20.0));

        //
        // FLEXIBLE SIZE WITH SHRINK AND (ARBITRARY) STRETCH
        //
        // Flexible size with shrink and stretch, too small
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 3.14), 0.0, 4.0),
            LAlloc::new(0.0, 4.0, 5.0));
        // Flexible size with shrink and stretch, too small with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 3.14), 0.0, 4.0, 2.0),
            LAlloc::new(0.0, 4.0, 5.0));

        // Flexible size with shrink and stretch, extra space
        assert_eq!(_alloc_region(
            &LReq::new_flex_size(10.0, 5.0, 3.14), 0.0, 20.0),
            LAlloc::new(0.0, 20.0, 20.0));
        // Flexible size with shrink and stretch, extra space with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_size(10.0, 5.0, 3.14), 0.0, 20.0, 12.0),
            LAlloc::new(0.0, 20.0, 20.0));

        //
        // ********** AS ABOVE BUT WITH REFERENCE POINT **********
        //
        // FIXED SIZE
        //
        // Fixed ref, too small
        assert_eq!(_alloc_region(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0),
            LAlloc::new_ref(0.0, 5.0, 10.0, 3.0));
        // Fixed ref, too small with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0, 2.0),
            LAlloc::new_ref(0.0, 5.0, 10.0, 2.0));
        // Fixed ref, too small with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0, 3.0),
            LAlloc::new_ref(0.0, 5.0, 10.0, 3.0));
        // Fixed ref, too small with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0, 4.0),
            LAlloc::new_ref(1.0, 4.0, 10.0, 3.0));

        // Fixed ref, natural size
        assert_eq!(_alloc_region(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Fixed ref, natural size with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0, 2.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 2.0));
        // Fixed ref, natural size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0, 3.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Fixed ref, natural size with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0, 4.0),
            LAlloc::new_ref(1.0, 9.0, 10.0, 3.0));

        // Fixed ref, extra space
        assert_eq!(_alloc_region(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Fixed ref, extra space with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0, 2.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 2.0));
        // Fixed ref, extra space with ref in correct place
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0, 3.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Fixed ref, extra space with ref further along; should offset position
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0, 5.0),
            LAlloc::new_ref(2.0, 10.0, 10.0, 3.0));

        //
        // FLEXIBLE SIZE WITH SHRINK
        //
        // Flexible ref with shrink, too small
        assert_eq!(_alloc_region(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 4.0),
            LAlloc::new_ref(0.0, 4.0, 5.0, 3.0));
        // Flexible ref with shrink, too small with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 4.0, 2.0),
            LAlloc::new_ref(0.0, 4.0, 5.0, 2.0));
        // Flexible ref with shrink, too small with correct ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 4.0, 3.0),
            LAlloc::new_ref(0.0, 4.0, 5.0, 3.0));
        // Flexible ref with shrink, too small with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 4.0, 4.0),
            LAlloc::new_ref(1.0, 3.0, 5.0, 3.0));

        // Flexible ref with shrink, minimum size
        assert_eq!(_alloc_region(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 5.0),
            LAlloc::new_ref(0.0, 5.0, 5.0, 3.0));
        // Flexible ref with shrink, minimum size with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 5.0, 2.0),
            LAlloc::new_ref(0.0, 5.0, 5.0, 2.0));
        // Flexible ref with shrink, minimum size with ref correct
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 5.0, 3.0),
            LAlloc::new_ref(0.0, 5.0, 5.0, 3.0));
        // Flexible ref with shrink, minimum size with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 5.0, 4.0),
            LAlloc::new_ref(1.0, 4.0, 5.0, 3.0));

        // Flexible ref with shrink, natural size
        assert_eq!(_alloc_region(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 10.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Flexible ref with shrink, natural size with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 10.0, 2.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 2.0));
        // Flexible ref with shrink, natural size with ref correct
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 10.0, 3.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Flexible ref with shrink, natural size with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 10.0, 4.0),
            LAlloc::new_ref(1.0, 9.0, 9.0, 3.0));

        // Flexible ref with shrink, extra space
        assert_eq!(_alloc_region(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 20.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Flexible ref with shrink, extra space with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 20.0, 2.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 2.0));
        // Flexible ref with shrink,extra space with ref correct
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 20.0, 3.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Flexible ref with shrink, extra space with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 0.0), 0.0, 20.0, 4.0),
            LAlloc::new_ref(1.0, 10.0, 10.0, 3.0));

        //
        // FLEXIBLE SIZE WITH SHRINK AND (ARBITRARY) STRETCH
        //
        // Flexible ref with shrink and stretch, natural size
        assert_eq!(_alloc_region(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 10.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Flexible ref with shrink and stretch, natural size with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 10.0, 2.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 2.0));
        // Flexible ref with shrink and stretch, natural size with ref correct
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 10.0, 3.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Flexible ref with shrink and stretch, natural size with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 10.0, 4.0),
            LAlloc::new_ref(1.0, 9.0, 9.0, 3.0));

        // Flexible ref with shrink and stretch, extra space
        assert_eq!(_alloc_region(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 20.0),
            LAlloc::new_ref(0.0, 20.0, 20.0, 3.0));
        // Flexible ref with shrink and stretch, extra space with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 20.0, 2.0),
            LAlloc::new_ref(0.0, 20.0, 20.0, 2.0));
        // Flexible ref with shrink and stretch,extra space with ref correct
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 20.0, 3.0),
            LAlloc::new_ref(0.0, 20.0, 20.0, 3.0));
        // Flexible ref with shrink and stretch, extra space with ref offset
        assert_eq!(_alloc_region_ref(
            &LReq::new_flex_ref(3.0, 7.0, 5.0, 3.14), 0.0, 20.0, 4.0),
            LAlloc::new_ref(0.0, 20.0, 20.0, 4.0));
    }
}
