use std::ops::{Add, Sub};

use graphics::vector2::Vector2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2 {
    pub x: f64,
    pub y: f64
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Point2 {
        return Point2{x: x, y: y};
    }

    pub fn origin() -> Point2 {
        return Point2{x: 0.0, y: 0.0};
    }
}

// ADDITION

impl Add<Vector2> for Point2 {
    type Output=Point2;

    fn add(self, b: Vector2) -> Point2 {
        return Point2{x: self.x + b.x, y: self.y + b.y};
    }
}

impl <'a> Add<Vector2> for &'a Point2 {
    type Output=Point2;

    fn add(self, b: Vector2) -> Point2 {
        return Point2{x: self.x + b.x, y: self.y + b.y};
    }
}

impl <'a> Add<&'a Vector2> for Point2 {
    type Output=Point2;

    fn add(self, b: &'a Vector2) -> Point2 {
        return Point2{x: self.x + b.x, y: self.y + b.y};
    }
}

impl <'a, 'b> Add<&'a Vector2> for &'b Point2 {
    type Output=Point2;

    fn add(self, b: &'a Vector2) -> Point2 {
        return Point2{x: self.x + b.x, y: self.y + b.y};
    }
}

// SUBTRACTION

impl Sub<Point2> for Point2 {
    type Output=Vector2;

    fn sub(self, b: Point2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a> Sub<Point2> for &'a Point2 {
    type Output=Vector2;

    fn sub(self, b: Point2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a> Sub<&'a Point2> for Point2 {
    type Output=Vector2;

    fn sub(self, b: &'a Point2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a, 'b> Sub<&'a Point2> for &'b Point2 {
    type Output=Vector2;

    fn sub(self, b: &'a Point2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}


impl Sub<Vector2> for Point2 {
    type Output=Point2;

    fn sub(self, b: Vector2) -> Point2 {
        return Point2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a> Sub<Vector2> for &'a Point2 {
    type Output=Point2;

    fn sub(self, b: Vector2) -> Point2 {
        return Point2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a> Sub<&'a Vector2> for Point2 {
    type Output=Point2;

    fn sub(self, b: &'a Vector2) -> Point2 {
        return Point2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a, 'b> Sub<&'a Vector2> for &'b Point2 {
    type Output=Point2;

    fn sub(self, b: &'a Vector2) -> Point2 {
        return Point2{x: self.x - b.x, y: self.y - b.y};
    }
}


//
// TESTS
//

#[cfg(test)]
mod tests {
    extern crate test;

    use std::mem;
    use super::*;
    use graphics::vector2::Vector2;

    #[test]
    fn test_constructor() {
        assert_eq!(Point2::new(1.0, 2.0).x, 1.0);
        assert_eq!(Point2::new(1.0, 2.0).y, 2.0);
        assert_eq!(Point2::origin(), Point2::new(0.0, 0.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(Point2::new(1.0, 2.0) + Vector2::new(2.0, 3.0), Point2::new(3.0, 5.0));
        assert_eq!(&Point2::new(1.0, 2.0) + Vector2::new(2.0, 3.0), Point2::new(3.0, 5.0));
        assert_eq!(Point2::new(1.0, 2.0) + &Vector2::new(2.0, 3.0), Point2::new(3.0, 5.0));
        assert_eq!(&Point2::new(1.0, 2.0) + &Vector2::new(2.0, 3.0), Point2::new(3.0, 5.0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Point2::new(1.0, 2.0) - Point2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
        assert_eq!(&Point2::new(1.0, 2.0) - Point2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
        assert_eq!(Point2::new(1.0, 2.0) - &Point2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
        assert_eq!(&Point2::new(1.0, 2.0) - &Point2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));

        assert_eq!(Point2::new(1.0, 2.0) - Vector2::new(2.0, 4.0), Point2::new(-1.0, -2.0));
        assert_eq!(&Point2::new(1.0, 2.0) - Vector2::new(2.0, 4.0), Point2::new(-1.0, -2.0));
        assert_eq!(Point2::new(1.0, 2.0) - &Vector2::new(2.0, 4.0), Point2::new(-1.0, -2.0));
        assert_eq!(&Point2::new(1.0, 2.0) - &Vector2::new(2.0, 4.0), Point2::new(-1.0, -2.0));
    }
}
