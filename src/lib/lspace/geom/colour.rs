use std::ops::{Add, Sub, Mul, Neg};

use pyrs::PyPrimWrapper;


pub const BLACK: Colour = Colour{r: 0.0, g: 0.0, b: 0.0, a: 1.0};
pub const WHITE: Colour = Colour{r: 1.0, g: 1.0, b: 1.0, a: 1.0};
pub const TRANSPARENT: Colour = Colour{r: 0.0, g: 0.0, b: 0.0, a: 0.0};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Colour {
        return Colour{r: r, g: g, b: b, a: a};
    }
}

// ADDITION

impl Add<Colour> for Colour {
    type Output=Colour;

    fn add(self, x: Colour) -> Colour {
        return Colour{r: self.r + x.r, g: self.g + x.g, b: self.b + x.b, a: self.a + x.a};
    }
}

impl <'a, 'b> Add<&'a Colour> for &'b Colour {
    type Output=Colour;

    fn add(self, x: &'a Colour) -> Colour {
        return Colour{r: self.r + x.r, g: self.g + x.g, b: self.b + x.b, a: self.a + x.a};
    }
}

impl <'a> Add<&'a Colour> for Colour {
    type Output=Colour;

    fn add(self, x: &'a Colour) -> Colour {
        return Colour{r: self.r + x.r, g: self.g + x.g, b: self.b + x.b, a: self.a + x.a};
    }
}

impl <'a> Add<Colour> for &'a Colour {
    type Output=Colour;

    fn add(self, x: Colour) -> Colour {
        return Colour{r: self.r + x.r, g: self.g + x.g, b: self.b + x.b, a: self.a + x.a};
    }
}

// SUBTRACTION

impl Sub<Colour> for Colour {
    type Output=Colour;

    fn sub(self, x: Colour) -> Colour {
        return Colour{r: self.r - x.r, g: self.g - x.g, b: self.b - x.b, a: self.a - x.a};
    }
}

impl <'a, 'b> Sub<&'a Colour> for &'b Colour {
    type Output=Colour;

    fn sub(self, x: &'a Colour) -> Colour {
        return Colour{r: self.r - x.r, g: self.g - x.g, b: self.b - x.b, a: self.a - x.a};
    }
}

impl <'a> Sub<&'a Colour> for Colour {
    type Output=Colour;

    fn sub(self, x: &'a Colour) -> Colour {
        return Colour{r: self.r - x.r, g: self.g - x.g, b: self.b - x.b, a: self.a - x.a};
    }
}

impl <'a> Sub<Colour> for &'a Colour {
    type Output=Colour;

    fn sub(self, x: Colour) -> Colour {
        return Colour{r: self.r - x.r, g: self.g - x.g, b: self.b - x.b, a: self.a - x.a};
    }
}

// MULTIPLICATION BY f32

impl Mul<f32> for Colour {
    type Output=Colour;

    fn mul(self, x: f32) -> Colour {
        return Colour{r: self.r * x, g: self.g * x, b: self.b * x, a: self.a * x};
    }
}

impl <'a> Mul<f32> for &'a Colour {
    type Output=Colour;

    fn mul(self, x: f32) -> Colour {
        return Colour{r: self.r * x, g: self.g * x, b: self.b * x, a: self.a * x};
    }
}

// MULTIPLICATION BY Colour

impl Mul<Colour> for Colour {
    type Output=Colour;

    fn mul(self, x: Colour) -> Colour {
        return Colour{r: self.r * x.r, g: self.g * x.g, b: self.b * x.b, a: self.a * x.a};
    }
}

impl <'a, 'b> Mul<&'a Colour> for &'b Colour {
    type Output=Colour;

    fn mul(self, x: &'a Colour) -> Colour {
        return Colour{r: self.r * x.r, g: self.g * x.g, b: self.b * x.b, a: self.a * x.a};
    }
}

impl <'a> Mul<&'a Colour> for Colour {
    type Output=Colour;

    fn mul(self, x: &'a Colour) -> Colour {
        return Colour{r: self.r * x.r, g: self.g * x.g, b: self.b * x.b, a: self.a * x.a};
    }
}

impl <'a> Mul<Colour> for &'a Colour {
    type Output=Colour;

    fn mul(self, x: Colour) -> Colour {
        return Colour{r: self.r * x.r, g: self.g * x.g, b: self.b * x.b, a: self.a * x.a};
    }
}

// NEGATION

impl Neg for Colour {
    type Output=Colour;

    fn neg(self) -> Colour {
        return Colour{r: -self.r, g: -self.g, b: -self.b, a: -self.a};
    }
}

impl <'a> Neg for &'a Colour {
    type Output=Colour;

    fn neg(self) -> Colour {
        return Colour{r: -self.r, g: -self.g, b: -self.b, a: -self.a};
    }
}


pub type PyColour = PyPrimWrapper<Colour>;
pub type PyColourOwned = Box<PyColour>;

// Function exported to Python for creating a boxed `Colour`
#[no_mangle]
pub extern "C" fn new_colour(r: f64, g: f64, b: f64, a: f64) -> PyColourOwned {
    Box::new(PyColour::new(Colour::new(r as f32, g as f32, b as f32, a as f32)))
}

#[no_mangle]
pub extern "C" fn destroy_colour(wrapper: PyColourOwned) {
    PyColour::destroy(wrapper);
}




//
// TESTS
//

mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.4).r, 0.1);
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.4).g, 0.2);
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.4).b, 0.3);
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.4).a, 0.4);
    }

    #[test]
    fn test_add() {
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.25) + Colour::new(0.5, 0.6, 0.2, 0.75),
                   Colour::new(0.6, 0.8, 0.5, 1.0));
        assert_eq!(&Colour::new(0.1, 0.2, 0.3, 0.25) + Colour::new(0.5, 0.6, 0.2, 0.75),
                   Colour::new(0.6, 0.8, 0.5, 1.0));
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.25) + &Colour::new(0.5, 0.6, 0.2, 0.75),
                   Colour::new(0.6, 0.8, 0.5, 1.0));
        assert_eq!(&Colour::new(0.1, 0.2, 0.3, 0.25) + &Colour::new(0.5, 0.6, 0.2, 0.75),
                   Colour::new(0.6, 0.8, 0.5, 1.0));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Colour::new(0.6, 0.5, 0.6, 0.75) - Colour::new(0.1, 0.2, 0.3, 0.25),
                   Colour::new(0.5, 0.3, 0.3, 0.5));
        assert_eq!(&Colour::new(0.6, 0.5, 0.6, 0.75) - Colour::new(0.1, 0.2, 0.3, 0.25),
                   Colour::new(0.5, 0.3, 0.3, 0.5));
        assert_eq!(Colour::new(0.6, 0.5, 0.6, 0.75) - &Colour::new(0.1, 0.2, 0.3, 0.25),
                   Colour::new(0.5, 0.3, 0.3, 0.5));
        assert_eq!(&Colour::new(0.6, 0.5, 0.6, 0.75) - &Colour::new(0.1, 0.2, 0.3, 0.25),
                   Colour::new(0.5, 0.3, 0.3, 0.5));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Colour::new(0.1, 0.2, 0.3, 0.4) * 2.0, Colour::new(0.2, 0.4, 0.6, 0.8));
        assert_eq!(&Colour::new(0.1, 0.2, 0.3, 0.4) * 2.0, Colour::new(0.2, 0.4, 0.6, 0.8));

        assert_eq!(Colour::new(0.1, 0.2, 0.25, 0.5) * Colour::new(0.5, 2.0, 3.0, 0.25),
                   Colour::new(0.05, 0.4, 0.75, 0.125));
        assert_eq!(&Colour::new(0.1, 0.2, 0.25, 0.5) * Colour::new(0.5, 2.0, 3.0, 0.25),
                   Colour::new(0.05, 0.4, 0.75, 0.125));
        assert_eq!(Colour::new(0.1, 0.2, 0.25, 0.5) * &Colour::new(0.5, 2.0, 3.0, 0.25),
                   Colour::new(0.05, 0.4, 0.75, 0.125));
        assert_eq!(&Colour::new(0.1, 0.2, 0.25, 0.5) * &Colour::new(0.5, 2.0, 3.0, 0.25),
                   Colour::new(0.05, 0.4, 0.75, 0.125));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Colour::new(0.1, 0.2, 0.3, 0.4), Colour::new(-0.1, -0.2, -0.3, -0.4));
        assert_eq!(-&Colour::new(0.1, 0.2, 0.3, 0.4), Colour::new(-0.1, -0.2, -0.3, -0.4));
    }
}
