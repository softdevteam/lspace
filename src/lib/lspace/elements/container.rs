use cairo::Context;

use graphics::rect2d::Rect2D;

use elements::element::{TElement, ElementChildRef};


pub trait TContainerElement : TElement {
    fn children<'a>(&'a self) -> &'a Vec<ElementChildRef>;
    fn children_mut<'a>(&'a mut self) -> &'a mut Vec<ElementChildRef>;

    fn draw_children(&self, cairo_ctx: &Context, visible_region: &Rect2D) {
        for chref in self.children() {
            let child = chref.get();
            let xa = child.x_alloc();
            let ya = child.y_alloc();
            let child_rect = Rect2D::from_allocs(xa, ya);
            if child_rect.intersects(visible_region) {
                let dx = xa.pos_in_parent();
                let dy = ya.pos_in_parent();
                let visible_region_child_space = visible_region.offset(-dx, -dy);
                cairo_ctx.save();
                cairo_ctx.translate(dx, dy);
                child.draw(cairo_ctx, &visible_region_child_space);
                cairo_ctx.restore();
            }
        }
    }

    fn update_children_x_req(&mut self) {
        for child in self.children_mut() {
            child.get_mut().update_x_req();
        }
    }

    fn update_children_y_req(&mut self) {
        for child in self.children_mut() {
            child.get_mut().update_y_req();
        }
    }

    fn allocate_children_x(&mut self) {
        for child in self.children_mut() {
            child.get_mut().allocate_x();
        }
    }

    fn allocate_children_y(&mut self) {
        for child in self.children_mut() {
            child.get_mut().allocate_y();
        }
    }
}
