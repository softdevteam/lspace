use std::ops::{Add, Sub, Mul};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        return Vector2{x: x, y: y};
    }

    pub fn zero() -> Vector2 {
        return Vector2{x: 0.0, y: 0.0};
    }
}

// ADDITION

impl Add<Vector2> for Vector2 {
    type Output=Vector2;

    fn add(self, b: Vector2) -> Vector2 {
        return Vector2{x: self.x + b.x, y: self.y + b.y};
    }
}

impl <'a, 'b> Add<&'a Vector2> for &'b Vector2 {
    type Output=Vector2;

    fn add(self, b: &'a Vector2) -> Vector2 {
        return Vector2{x: self.x + b.x, y: self.y + b.y};
    }
}

impl <'a> Add<&'a Vector2> for Vector2 {
    type Output=Vector2;

    fn add(self, b: &'a Vector2) -> Vector2 {
        return Vector2{x: self.x + b.x, y: self.y + b.y};
    }
}

impl <'a> Add<Vector2> for &'a Vector2 {
    type Output=Vector2;

    fn add(self, b: Vector2) -> Vector2 {
        return Vector2{x: self.x + b.x, y: self.y + b.y};
    }
}

// SUBTRACTION

impl Sub<Vector2> for Vector2 {
    type Output=Vector2;

    fn sub(self, b: Vector2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a, 'b> Sub<&'a Vector2> for &'b Vector2 {
    type Output=Vector2;

    fn sub(self, b: &'a Vector2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a> Sub<&'a Vector2> for Vector2 {
    type Output=Vector2;

    fn sub(self, b: &'a Vector2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}

impl <'a> Sub<Vector2> for &'a Vector2 {
    type Output=Vector2;

    fn sub(self, b: Vector2) -> Vector2 {
        return Vector2{x: self.x - b.x, y: self.y - b.y};
    }
}


// MULTIPLICATION

impl Mul<f64> for Vector2 {
    type Output=Vector2;

    fn mul(self, b: f64) -> Vector2 {
        return Vector2{x: self.x * b, y: self.y * b};
    }
}

impl <'a> Mul<f64> for &'a Vector2 {
    type Output=Vector2;

    fn mul(self, b: f64) -> Vector2 {
        return Vector2{x: self.x * b, y: self.y * b};
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

    #[test]
    fn test_constructor() {
        assert_eq!(Vector2::new(1.0, 2.0).x, 1.0);
        assert_eq!(Vector2::new(1.0, 2.0).y, 2.0);
        assert_eq!(Vector2::zero(), Vector2::new(0.0, 0.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(Vector2::new(1.0, 2.0) + Vector2::new(2.0, 3.0), Vector2::new(3.0, 5.0));
        assert_eq!(&Vector2::new(1.0, 2.0) + Vector2::new(2.0, 3.0), Vector2::new(3.0, 5.0));
        assert_eq!(Vector2::new(1.0, 2.0) + &Vector2::new(2.0, 3.0), Vector2::new(3.0, 5.0));
        assert_eq!(&Vector2::new(1.0, 2.0) + &Vector2::new(2.0, 3.0), Vector2::new(3.0, 5.0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vector2::new(1.0, 2.0) - Vector2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
        assert_eq!(&Vector2::new(1.0, 2.0) - Vector2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
        assert_eq!(Vector2::new(1.0, 2.0) - &Vector2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
        assert_eq!(&Vector2::new(1.0, 2.0) - &Vector2::new(2.0, 4.0), Vector2::new(-1.0, -2.0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vector2::new(1.0, 2.0) * 2.0, Vector2::new(2.0, 4.0));
        assert_eq!(&Vector2::new(1.0, 2.0) * 2.0, Vector2::new(2.0, 4.0));
    }
}
