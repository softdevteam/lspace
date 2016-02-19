use std::cell::Cell;

use geom::point2::Point2;
use geom::affinexform2::AffineXform2;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointerPosition {
    OutOfBounds,
    AtPosition(Point2),
}

impl PointerPosition {
    pub fn out_of_bounds() -> PointerPosition {
        PointerPosition::OutOfBounds
    }

    pub fn at_position(pos: Point2) -> PointerPosition {
        PointerPosition::AtPosition(pos)
    }

    pub fn is_within_bounds(&self) -> bool {
        match self {
            &PointerPosition::OutOfBounds => false,
            &PointerPosition::AtPosition(..) => true
        }
    }

    pub fn position<'a>(&'a self) -> Option<&'a Point2> {
        match self {
            &PointerPosition::OutOfBounds => None,
            &PointerPosition::AtPosition(ref pos) => Some(pos)
        }
    }

    pub fn transformed(&self, xform: &AffineXform2) -> PointerPosition {
        match self {
            &PointerPosition::OutOfBounds => PointerPosition::OutOfBounds,
            &PointerPosition::AtPosition(ref p) => PointerPosition::AtPosition(xform * p)
        }
    }
}


pub trait TPointer<'a> {
    fn position(&self) -> PointerPosition;
    fn transformed(&'a self, x: &AffineXform2) -> TransformedPointer;
    fn concrete_pointer(&'a self) -> &'a Pointer;
}


pub struct Pointer {
    position: Cell<PointerPosition>,
}

impl Pointer {
    pub fn new() -> Pointer {
        Pointer{position: Cell::new(PointerPosition::out_of_bounds())}
    }


    pub fn set_position(&self, pos: PointerPosition) {
        self.position.set(pos);
    }
}

impl <'a> TPointer<'a> for Pointer {
    fn position(&self) -> PointerPosition {
        self.position.get()
    }

    fn transformed(&'a self, x: &AffineXform2) -> TransformedPointer<'a> {
        TransformedPointer::new(self, x)
    }

    fn concrete_pointer(&'a self) -> &'a Pointer {
        self
    }
}


pub struct TransformedPointer<'a> {
    underlying_pointer: &'a Pointer,
    xform: AffineXform2
}

impl <'a> TransformedPointer<'a> {
    fn new(underlying_pointer: &'a Pointer, xform: &AffineXform2) -> TransformedPointer<'a> {
        TransformedPointer{underlying_pointer: underlying_pointer, xform: *xform}
    }
}

impl <'a> TPointer<'a> for TransformedPointer<'a> {
    fn position(&self) -> PointerPosition {
        self.underlying_pointer.position.get().transformed(&self.xform)
    }

    fn transformed(&'a self, x: &AffineXform2) -> TransformedPointer<'a> {
        TransformedPointer::new(self.underlying_pointer, &(x * self.xform))
    }

    fn concrete_pointer(&'a self) -> &'a Pointer {
        self.underlying_pointer.concrete_pointer()
    }
}
