use layout::lalloc::LAlloc;

pub struct Rect2D {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect2D {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rect2D {
        return Rect2D{x: x, y: y, w: w, h: h};
    }

    pub fn from_allocs(x_alloc: &LAlloc, y_alloc: &LAlloc) -> Rect2D {
        return Rect2D{x: x_alloc.pos_in_parent(),
                      y: y_alloc.pos_in_parent(),
                      w: x_alloc.actual_size(),
                      h: y_alloc.actual_size()};
    }


    pub fn right(&self) -> f64 {
        return self.x + self.w;
    }

    pub fn bottom(&self) -> f64 {
        return self.y + self.h;
    }


    pub fn offset(&self, dx: f64, dy: f64) -> Rect2D {
        return Rect2D{x: self.x + dx, y: self.y + dy, w: self.w, h: self.h};
    }

    pub fn intersects(&self, r: &Rect2D) -> bool {
        return self.x < r.right() && self.y < r.bottom() && self.right() > r.x && self.bottom() > r.y;
    }
}
