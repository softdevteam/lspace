use geom::point2::Point2;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointerPosition {
    OutOfBounds,
    AtPosition(Point2),
}

impl PointerPosition {
    pub fn out_of_bounds() -> PointerPosition {
        return PointerPosition::OutOfBounds;
    }

    pub fn at_position(pos: Point2) -> PointerPosition {
        return PointerPosition::AtPosition(pos);
    }

    pub fn is_within_bounds(&self) -> bool {
        return match self {
            &PointerPosition::OutOfBounds => false,
            &PointerPosition::AtPosition(..) => true
        }
    }

    pub fn position<'a>(&'a self) -> Option<&'a Point2> {
        return match self {
            &PointerPosition::OutOfBounds => None,
            &PointerPosition::AtPosition(ref pos) => Some(pos)
        };
    }
}

pub struct Pointer {
    position: PointerPosition,
}

impl Pointer {
    pub fn new() -> Pointer {
        return Pointer{position: PointerPosition::out_of_bounds()};
    }


    pub fn position<'a>(&'a self) -> &'a PointerPosition {
        return &self.position;
    }

    pub fn set_position(&mut self, pos: &PointerPosition) {
        self.position = *pos;
    }
}
