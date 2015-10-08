use std::rc::Rc;
use std::cell::RefCell;
use std::string::String;
use std::hash::{Hash};
use std::mem::transmute;

use cairo::Context;
use cairo_sys::enums::{FontSlant, FontWeight};

use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;
use elements::element_ctx::{ElementContext};
use elements::element::{ElementReq, ElementAlloc, TElementLayout, TElement};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextWeight {
    Normal,
    Bold,
}

impl TextWeight {
    pub fn as_cairo(&self) -> FontWeight {
        match self {
            &TextWeight::Normal => FontWeight::Normal,
            &TextWeight::Bold => FontWeight::Bold,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TextSlant {
    Normal,
    Italic,
}

impl TextSlant {
    pub fn as_cairo(&self) -> FontSlant {
        match self {
            &TextSlant::Normal => FontSlant::Normal,
            &TextSlant::Italic => FontSlant::Italic,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextStyleParams {
    font_family: String,
    weight: TextWeight,
    slant: TextSlant,
    size: f64,
    colour: (f32, f32, f32, f32),
}

impl TextStyleParams {
    pub fn new(font_family: String, weight: TextWeight, slant: TextSlant,
               size: f64, colour: (f32, f32, f32, f32)) -> TextStyleParams {
        return TextStyleParams{font_family: font_family, weight: weight, slant: slant, size: size,
                               colour: colour};
    }

    pub fn default() -> TextStyleParams {
        return TextStyleParams{font_family: String::from("Sans serif"),
                               weight: TextWeight::Normal, slant: TextSlant::Normal,
                               size: 14.0, colour: (0.0, 0.0, 0.0, 0.0)};
    }

    pub fn with_family(font_family: String) -> TextStyleParams {
        return TextStyleParams{font_family: font_family,
                               weight: TextWeight::Normal, slant: TextSlant::Normal,
                               size: 14.0, colour: (0.0, 0.0, 0.0, 0.0)};
    }

    pub fn with_family_and_colour(font_family: String,
                                  colour: (f32, f32, f32, f32)) -> TextStyleParams {
        return TextStyleParams{font_family: font_family,
                               weight: TextWeight::Normal, slant: TextSlant::Normal,
                               size: 14.0, colour: colour};
    }

    pub fn text_req_key(&self, text: String) -> TextReqKey {
        // In all the cases we care about the font size will not be an invalid FP number.
        // As a consequence of Rust's stick-up-its-arse handling of floats we have to use
        // some nasty unsafe code to get something that we can get a hash code from...
        let hashable_size: i64 = unsafe{transmute(self.size)};
        return (self.font_family.clone(), self.weight, self.slant, hashable_size, text);
    }

    pub fn apply(&self, cairo_ctx: &Context) {
        cairo_ctx.select_font_face(self.font_family.as_str(), self.slant.as_cairo(),
                                   self.weight.as_cairo());
        cairo_ctx.set_font_size(self.size);
        cairo_ctx.set_source_rgba(self.colour.0 as f64, self.colour.1 as f64, self.colour.2 as f64,
                                  self.colour.3 as f64);
    }
}

pub type TextReqKey = (String, TextWeight, TextSlant, i64, String);





pub struct TextElement {
    req: Rc<ElementReq>,
    style: Rc<TextStyleParams>,
    alloc: ElementAlloc,
    text: String,
}

impl TextElement {
    pub fn new(text: String, style: Rc<TextStyleParams>, cairo_ctx: &Context,
               elem_ctx: &RefCell<ElementContext>) -> TextElement {
        let req = elem_ctx.borrow_mut().text_shared_req(style.clone(), text.clone(), cairo_ctx);
        return TextElement{text: text,
                           req: req,
                           style: style,
                           alloc: ElementAlloc::new()};
    }
}

impl TElementLayout for TextElement {
    fn element_req(&self) -> &ElementReq {
        return &*self.req;
    }

    fn element_alloc(&self) -> &ElementAlloc {
        return &self.alloc;
    }

    fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc) {
        return (&*self.req, &mut self.alloc);
    }
}

impl TElement for TextElement {
    fn draw_self(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        let y = match self.alloc.y_alloc.ref_point() {
            None => 0.0,
            Some(ref_point) => ref_point
        };
        cairo_ctx.move_to(0.0, y);
        self.style.apply(cairo_ctx);
        cairo_ctx.show_text(self.text.as_str());
    }

    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        self.draw_self(cairo_ctx, visible_region);
    }

    fn update_x_req(&mut self) {
        // Nothing to do; requisition is shared
    }

    fn allocate_x(&mut self) {
        // Nothing to do; no children
    }

    fn update_y_req(&mut self) {
        // Nothing to do; requisition is shared
    }

    fn allocate_y(&mut self) {
        // Nothing to do; no children
    }
}
