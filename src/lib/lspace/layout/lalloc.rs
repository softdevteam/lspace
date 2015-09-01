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


/// Linear allocation helper function
/// Allocate child elements their requested size, no flexibility
///
/// Parameters:
/// `n` : the number of child elements
/// `child_reqs` : child space requisitions
/// `child_allocs` : child allocations
/// `start_pos` : the position of the first child
/// `space_between` : optional amount of space between each child
///
/// Returns:
/// The position of a subsequent child
fn alloc_children_linear_fixed(n: usize,
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

/// Allocation helper function
/// Allocate child elements using flexibility
///
/// Parameters:
/// `n` : the number of child elements
/// `child_reqs` : child space requisitions
/// `child_allocs` : child allocations
/// `start_pos` : the position of the first child
/// `space_between` : optional amount of space between each child
/// `shrink_frac` : multiply a child's shrink space by this fraction to get the amount to shrink
/// the child element by
/// `stretch_factor` : multiply a child's stretchiness by this factor to get the amount to stretch
/// the child element by
///
/// Returns:
/// The position of a subsequent child
fn alloc_children_linear_flex(n: usize,
                              child_reqs: &[&LReq], child_allocs: &mut [&mut LAlloc],
                              start_pos: f64, space_between: f64,
                              shrink_frac: f64, stretch_factor: f64) -> f64 {
    let mut pos = start_pos;
    for i in 0..n {
        let creq = child_reqs[i];
        let cshrink = creq.flex().shrink() * shrink_frac;
        let cstretch = (creq.flex().stretch() as f64) * stretch_factor;

        let sz = creq.size().size();
        let csz = creq.size().size() - cshrink + cstretch;
        let cref = match creq.size().before_ref_opt() {
            None => None,
            Some(before_ref) => Some(before_ref + cstretch * before_ref / sz),
        };

        child_allocs[i].alloc_from_region(creq, pos, csz, cref);
        pos = pos + csz + space_between;
    }
    return pos;
}

/// Allocation helper function
/// Compute flexibility factors for the region. Given the region allocated size, its minimum and
/// natural size and its accumulated stretchiness (sum of stretchiness of children) this function
/// will determine if child elements should shrink or stretch and by how much.
///
/// Parameters:
/// `region_size` : the amount of space allocated to the region in which child elements are being
/// placed
/// `minimum_size` : the minimum size of the region
/// `natural_size` : the natural size of the region
/// `region_stretch` : sum of stretchiness of children
///
/// Returns:
/// None : No flexibility required
/// Some((shrink_fraction, stretch_factor)) : a tuple of the shrink fraction (fraction of shrink
/// space to apply for each child) and the stretch factor (multiply a child's stretchiness by
/// this factor to compute amount by which it should be stretched).
fn compute_flex_factors(region_size: f64, minimum_size: f64, natural_size: f64,
                        region_stretch: f32) -> Option<(f64, f64)> {
    if region_size < minimum_size * ONE_MINUS_EPSILON {
        // Insufficient space; allocate minimum space; shrink_frac=1.0
        return Some((1.0, 0.0));
    } else if region_size < natural_size * ONE_MINUS_EPSILON {
        // Region_size is between minimum and natural size
        let shrink_frac = (region_size - natural_size) /
                (minimum_size - natural_size);
        return Some((shrink_frac, 0.0));
    } else if region_size > natural_size * ONE_PLUS_EPSILON {
        let stretch_space = region_size - natural_size;

        if region_stretch > 0.0 {
            // Region size is greater than the natural size and stretch > 0
            let stretch_factor = stretch_space / region_stretch as f64;
            return Some((0.0, stretch_factor));
        } else {
            // This element won't stretch
            return None;
        }
    } else {
        // Region size matches natural size
        return None;
    }
}

/// Allocation helper function
/// Allocate child elements in a linear fashion
fn alloc_children_linear(n: usize, child_reqs: &[&LReq], child_allocs: &mut [&mut LAlloc],
                  natural_size: f64, minimum_size: f64, region_pos: f64, region_size: f64,
                  region_stretch: f32, space_between: f64, pad_start: bool) -> f64 {
    match compute_flex_factors(region_size, minimum_size, natural_size, region_stretch) {
        Some((shrink_frac, stretch_frac)) => {
            alloc_children_linear_flex(n, child_reqs, child_allocs, region_pos, space_between,
                                       shrink_frac, stretch_frac)
        },

        None => {
            let start = if pad_start {
                let stretch_space = region_size - natural_size;
                region_pos + stretch_space
            } else {
                region_pos
            };
            alloc_children_linear_fixed(n, child_reqs, child_allocs, start, space_between)
        }
    }
}

/// Allocation helper function
/// Allocate child elements in a linear fashion, region has NO reference point
fn alloc_children_linear_no_ref(n: usize, child_reqs: &Vec<&LReq>,
                                child_allocs: &mut Vec<&mut LAlloc>,
                                region_req: &LReq, region_pos: f64, region_size: f64,
                                space_between: f64) {
    match region_req.flex() {
        &LFlex::Fixed => {
            alloc_children_linear_fixed(n, child_reqs, child_allocs, region_pos,
                                        space_between);
        },
        &LFlex::Flex{..} => {
            let natural_size = region_req.size().size();
            let minimum_size = natural_size - region_req.flex().shrink();

            alloc_children_linear(n, &child_reqs[..],
                                  &mut child_allocs[..],
                                  natural_size, minimum_size,
                                  region_pos, region_size,
                                  region_req.flex().stretch(),
                                  space_between, false);
        },
    };
}

/// Allocation helper function
/// Allocate child elements in a linear fashion, region has reference point
fn alloc_children_linear_with_ref(n: usize, child_reqs: &Vec<&LReq>,
                                  child_allocs: &mut Vec<&mut LAlloc>,
                                  region_req: &LReq, region_pos: f64, region_size: f64,
                                  region_before_ref: f64, space_between: f64,
                                  ref_point_n: usize) {
    match region_req.flex() {
        &LFlex::Fixed => {
            // Compute the offset required to position the array of children so that
            // the required reference point aligns with the allocated reference point.
            // Assuming that the region requisition has its reference point
            // aligned with that of the correct child, this should be fine...
            let before_offset = fast_max(
                region_before_ref - region_req.size().before_ref(), 0.0);
            alloc_children_linear_fixed(n, &child_reqs[..], &mut child_allocs[..],
                                        before_offset + region_pos, space_between);
        },
        &LFlex::Flex{..} => {
            let req_at_ref_n = child_reqs[ref_point_n];

            let ref_n_shrink = req_at_ref_n.flex().shrink();
            let ref_n_stretch = req_at_ref_n.flex().stretch();

            // We have to allocate space in two distinct pieces; the piece before the reference
            // point and the piece after

            // Before reference point index
            let (shrink_before_ref, stretch_before_ref, nat_size_before_ref,
                        min_size_before_ref, region_size_before_ref,
                        spill_before_ref) = {
                let reqs = &child_reqs[..ref_point_n];

                let shrink = reqs.iter().fold(0.0,
                        |acc, x| acc + x.flex().shrink()) + ref_n_shrink * 0.5;
                let stretch = reqs.iter().fold(0.0,
                        |acc, x| acc + x.flex().stretch()) + ref_n_stretch * 0.5;

                let natural_size = region_req.size().before_ref();
                let minimum_size = natural_size - shrink;
                let avail_size = region_before_ref;
                // The spill is the additional space required to reach the minimum
                // required size.
                let spill = fast_max(minimum_size - avail_size, 0.0);

                (shrink, stretch, natural_size, minimum_size, avail_size, spill)
            };

            // After reference point index
            let (shrink_after_ref, stretch_after_ref, nat_size_after_ref,
                        min_size_after_ref, region_size_after_ref) = {
                let reqs = &child_reqs[ref_point_n+1..];

                let shrink = reqs.iter().fold(0.0,
                        |acc, x| acc + x.flex().shrink()) + ref_n_shrink * 0.5;
                let stretch = reqs.iter().fold(0.0,
                        |acc, x| acc + x.flex().stretch()) + ref_n_stretch * 0.5;

                let natural_size = region_req.size().after_ref();
                let minimum_size = natural_size - shrink;
                // take into account any spill over from before the ref point
                let avail_size = region_size - region_before_ref - spill_before_ref;

                (shrink, stretch, natural_size, minimum_size, avail_size)
            };

            let mut pos = region_pos;
            // Allocate children before ref point
            pos = alloc_children_linear(n, &child_reqs[..ref_point_n],
                                        &mut child_allocs[..ref_point_n],
                                        nat_size_before_ref, min_size_before_ref,
                                        pos, region_size_before_ref,
                                        stretch_before_ref,
                                        space_between, true);

            // Allocate child aligned with ref point
            // Before reference point: compute flex factors and apply
            let mut ref_child_before_ref = req_at_ref_n.size().before_ref();
            match compute_flex_factors(region_size_before_ref, min_size_before_ref,
                                       nat_size_before_ref, stretch_before_ref) {
                Some((shrink_frac, stretch_factor)) => {
                    ref_child_before_ref = ref_child_before_ref -
                        req_at_ref_n.flex().shrink() * shrink_frac * 0.5 +
                        req_at_ref_n.flex().stretch() as f64 * stretch_factor * 0.5;
                },
                None => {},
            };

            // After reference point: compute flex factors and apply
            let mut ref_child_after_ref = req_at_ref_n.size().after_ref();
            match compute_flex_factors(region_size_after_ref, min_size_after_ref,
                                       nat_size_after_ref, stretch_after_ref) {
                Some((shrink_frac, stretch_factor)) => {
                    ref_child_after_ref = ref_child_after_ref -
                        req_at_ref_n.flex().shrink() * shrink_frac * 0.5 +
                        req_at_ref_n.flex().stretch() as f64 * stretch_factor * 0.5;
                },
                None => {},
            };

            // Get the total size
            let ref_child_sz = ref_child_before_ref + ref_child_after_ref;
            // Allocate the child aligned with ref point
            child_allocs[ref_point_n].alloc_from_region(req_at_ref_n,
                pos, ref_child_after_ref, Some(ref_child_before_ref));
            // Advance position
            pos = pos + ref_child_sz + space_between;

            // Allocate children after reference point
            alloc_children_linear(n, &child_reqs[ref_point_n+1..],
                                  &mut child_allocs[ref_point_n+1..],
                                  nat_size_after_ref, min_size_after_ref,
                                  pos, region_size_after_ref,
                                  stretch_after_ref,
                                  space_between, false);
        },
    }
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
            // Both the allocation region and the requisition have a reference point
            Some((req_before, req_after)) => {
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

            // No reference point required
            None => {
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
                        region_req: &LReq, region_pos: f64, region_size: f64,
                        region_ref: Option<f64>, space_between: f64,
                        ref_point_index: Option<usize>) {
        let n = child_reqs.len();
        assert_eq!(n, child_allocs.len());

        // There are a few valid, consistent configurations that are identified and handled
        match (ref_point_index, region_req.size(), region_ref) {
            (None, &LNatSize::Size{..}, None) => {
                // NO REFERENCE POINTS
                alloc_children_linear_no_ref(n, child_reqs, child_allocs, region_req, region_pos,
                                             region_size, space_between);
            },

            (Some(ref_point_n), &LNatSize::Ref{..}, Some(region_before_ref)) => {
                assert!(ref_point_n < n);
                // REFERENCE POINT INDEX SPECIFIED
                // REQUISITION HAS REF POINT
                // REGION HAS REF POINT

                alloc_children_linear_with_ref(n, child_reqs, child_allocs, region_req,
                                               region_pos, region_size, region_before_ref,
                                               space_between, ref_point_n);
            },

            // EMPTY PARENT REQUISITIONS
            (_, &LNatSize::Empty, None) => {
                // Ensure children are all empty, otherwise we have a bug. Nothing else to do.
                for (i, ch) in child_reqs.iter().enumerate() {
                    if ch.size() != &LNatSize::Empty {
                        panic!("Empty parent requisition, non-empty child requisiton at \
                               index {}", i);
                    }
                }
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
