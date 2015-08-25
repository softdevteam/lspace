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

    /// Allocate this element space in the specified region
    ///
    /// Allocates the element represented by `self` space from the given region.
    ///
    /// Parameters:
    /// `req` : the space requirements of the element represented by `self`; an instance of `LReq`
    /// `region_pos` : the position of the region in the parent space; most elements that have
    /// multiple children will want to give them different positions
    /// `region_size` : the size of the region
    /// `region_ref` : the reference point of the region (optional)
    pub fn alloc_from_region(&mut self, req: &LReq, region_pos: f64, region_size: f64,
                             region_ref: Option<f64>) {
        match req.size().before_ref_opt() {
            Some(req_ref) => {
                // Both the allocation region and the requisition have a reference point

                // If the region's reference point is too close to the start, then compute
                // the offset required to move it to the required reference point
                let ref_offset = match region_ref {
                    Some(avail_ref) => fast_max(req_ref - avail_ref, 0.0),
                    None => 0.0,
                };

                // Compute the preferred and minimum sizes
                let preferred_size = req.size().size() + ref_offset;
                // Don't need to add `ref_offset` to `minimum_size` is its already added to
                // `preferred_size` that is used to compute `minimum_size`.
                let minimum_size = preferred_size - req.flex().shrink();

                if region_size < minimum_size * ONE_MINUS_EPSILON {
                    // Insufficient space
                    self.update_ref(region_pos, region_size, minimum_size, req_ref);
                } else if region_size > preferred_size * ONE_PLUS_EPSILON {
                    // More space than preferred size
                    if req.flex().stretch() > 0.0 {
                        // Stretch to take up additional space
                        let ref_point = match region_ref {
                            None => req_ref,
                            Some(avail_ref) => fast_max(avail_ref, req_ref),
                        };
                        self.update_ref(region_pos, region_size, region_size, ref_point);
                    } else {
                        let offset = match region_ref {
                            None => 0.0,
                            Some(avail_ref) =>
                                fast_min(fast_max(avail_ref - req_ref, 0.0), preferred_size)
                        };
                        // No stretch; don't take up additional space
                        self.update_ref(offset + region_pos, preferred_size, preferred_size,
                                        req_ref);
                    }
                } else {
                    // region_size is between minimum and preferred size
                    self.update_ref(region_pos, region_size, region_size, req_ref);
                }
            },

            None => {
                // No reference point required
                let preferred_size = req.size().size();
                let minimum_size = preferred_size - req.flex().shrink();

                if region_size < minimum_size * ONE_MINUS_EPSILON {
                    // Insufficient space; allocate the available space, while giving
                    // `minimum_size` as the actual size for bounding box and rendering purposes
                    self.update(region_pos, region_size, minimum_size);
                } else if region_size < preferred_size * ONE_PLUS_EPSILON ||
                          req.flex().stretch() > 0.0 {
                    // Either:
                    // - region_size is between minimum and preferred size
                    // or
                    // - region_size is > preferred size and the box can stretch to use the
                    // additional space
                    //
                    // Allocate all the available space
                    self.update(region_pos, region_size, region_size);
                } else {
                    // region_size is > preferred and no stretch; don't take up additional space
                    self.update(region_pos, preferred_size, preferred_size);
                }
            }
        }
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
        // Fixed size, too small
        assert_eq!(_alloc_region(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0),
            LAlloc::new_ref(0.0, 5.0, 10.0, 3.0));
        // Fixed size, too small with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0, 2.0),
            LAlloc::new_ref(0.0, 5.0, 11.0, 3.0));
        // Fixed size, too small with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 5.0, 3.0),
            LAlloc::new_ref(0.0, 5.0, 10.0, 3.0));

        // Fixed size, natural size
        assert_eq!(_alloc_region(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Fixed size, natural size with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0, 2.0),
            LAlloc::new_ref(0.0, 10.0, 11.0, 3.0));
        // Fixed size, natural size with ref
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 10.0, 3.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));

        // Fixed size, extra space
        assert_eq!(_alloc_region(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0),
            LAlloc::new_ref(0.0, 10.0, 10.0, 3.0));
        // Fixed size, extra space with ref too small
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0, 2.0),
            LAlloc::new_ref(0.0, 11.0, 11.0, 3.0));
        // Fixed size, extra space with ref in correct place
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0, 5.0),
            LAlloc::new_ref(2.0, 10.0, 10.0, 3.0));
        // Fixed size, extra space with ref further along; should offset position
        assert_eq!(_alloc_region_ref(
            &LReq::new_fixed_ref(3.0, 7.0), 0.0, 20.0, 5.0),
            LAlloc::new_ref(2.0, 10.0, 10.0, 3.0));
    }
}
