use cairo::Context;

use std::cell::Ref;

use geom::vector2::Vector2;
use geom::bbox2::BBox2;

use layout::lreq::LReq;
use layout::lalloc::LAlloc;

use elements::element_ctx::ElementLayoutContext;
use elements::element::{TElement, ElementRef};


pub trait TContainerElement : TElement {
    fn children(&self) -> Ref<[ElementRef]>;

    fn draw_children(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        for child in self.children().iter() {
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

    fn update_children_x_req(&self, layout_ctx: &ElementLayoutContext) -> bool {
        let mut changed: bool = false;
        for child in self.children().iter() {
            changed = changed | child.update_x_req(layout_ctx);
        }
        return changed;
    }

    fn update_children_y_req(&self) -> bool {
        let mut changed: bool = false;
        for child in self.children().iter() {
            changed = changed | child.update_y_req();
        }
        return changed;
    }

    fn allocate_children_x(&self, alloc_xs: &Vec<LAlloc>) -> bool {
        let mut child_y_reqs_dirty: bool = false;
        for c in self.children().iter().zip(alloc_xs) {
            child_y_reqs_dirty = child_y_reqs_dirty | c.0.allocate_x(c.1);
        }
        return child_y_reqs_dirty;
    }

    fn allocate_children_y(&self, alloc_ys: &Vec<LAlloc>) {
        for c in self.children().iter().zip(alloc_ys) {
            c.0.allocate_y(c.1);
        }
    }

    fn compute_x_req(&self) -> LReq;
    fn compute_child_x_allocs(&self) -> Vec<LAlloc>;
    fn compute_y_req(&self) -> LReq;
    fn compute_child_y_allocs(&self) -> Vec<LAlloc>;

    fn container_update_x_req(&self, layout_ctx: &ElementLayoutContext) -> bool {
        if !self.element_alloc().is_x_req_update_required() {
            return false;
        }
        let children_changed = self.update_children_x_req(layout_ctx);
        let mut changed: bool = false;
        if children_changed {
            let x_req = self.compute_x_req();
            changed = self.element_update_x_req(&x_req);
        }
        let mut alloc_mut = self.element_alloc_mut();
        alloc_mut.x_req_updated();
        if children_changed || changed {
            alloc_mut.x_alloc_dirty();
        }
        return changed;
    }

    fn container_allocate_x(&self, x_alloc: &LAlloc) -> bool {
        let update_needed;
        {
            let mut elem_alloc = self.element_alloc_mut();
            let changed = elem_alloc.update_x_alloc(x_alloc);
            update_needed = changed | elem_alloc.is_x_alloc_update_required();
            elem_alloc.x_alloc_updated();
        }
        if !update_needed {
            return false;
        }
        let x_allocs = self.compute_child_x_allocs();
        let child_y_reqs_dirty = self.allocate_children_x(&x_allocs);
        if child_y_reqs_dirty {
            let mut elem_alloc = self.element_alloc_mut();
            elem_alloc.y_req_dirty();
            elem_alloc.y_alloc_dirty();
        }
        return child_y_reqs_dirty;
    }

    fn container_update_y_req(&self) -> bool {
        if !self.element_alloc().is_y_req_update_required() {
            return false;
        }
        self.update_children_y_req();
        let y_req = self.compute_y_req();
        let changed = self.element_update_y_req(&y_req);
        let mut alloc_mut = self.element_alloc_mut();
        alloc_mut.y_req_updated();
        if changed {
            alloc_mut.y_alloc_dirty();
        }
        return changed;
    }

    fn container_allocate_y(&self, y_alloc: &LAlloc) {
        let update_needed;
        {
            let mut elem_alloc = self.element_alloc_mut();
            let changed = elem_alloc.update_y_alloc(y_alloc);
            update_needed = changed | elem_alloc.is_y_alloc_update_required();
            elem_alloc.y_alloc_updated();
        }
        if update_needed {
            let y_allocs = self.compute_child_y_allocs();
            self.allocate_children_y(&y_allocs);
        }
    }
}
