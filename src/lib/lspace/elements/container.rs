use cairo::Context;

use std::cell::Ref;

use geom::vector2::Vector2;
use geom::bbox2::BBox2;

use elements::element::{TElement, ElementRef};


pub trait TContainerElement : TElement {
    fn children(&self) -> Ref<Vec<ElementRef>>;

    fn draw_children(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        for child_ref in self.children().iter() {
            let child = child_ref.get();
            let a = child.element_alloc();
            let child_bbox = BBox2::from_allocs(&a.x_alloc, &a.y_alloc);
            if child_bbox.intersects(visible_region) {
                let dx = a.x_alloc.pos_in_parent();
                let dy = a.y_alloc
                .pos_in_parent();
                let visible_region_child_space = visible_region.offset(&Vector2::new(-dx, -dy));
                cairo_ctx.save();
                cairo_ctx.translate(dx, dy);
                child.draw(cairo_ctx, &visible_region_child_space);
                cairo_ctx.restore();
            }
        }
    }

    fn update_children_x_req(&self) {
        for child in self.children().iter() {
            child.get().update_x_req();
        }
    }

    fn update_children_y_req(&self) {
        for child in self.children().iter() {
            child.get().update_y_req();
        }
    }

    fn allocate_children_x(&self) {
        for child in self.children().iter() {
            child.get().allocate_x();
        }
    }

    fn allocate_children_y(&self) {
        for child in self.children().iter() {
            child.get().allocate_y();
        }
    }
}
