use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::string::String;
use std::hash::{Hash};
use std::mem::transmute;

use cairo::Context;
use cairo_sys::enums::{FontSlant, FontWeight};

use layout::lalloc::LAlloc;
use geom::bbox2::BBox2;
use geom::colour::{Colour, BLACK};
use elements::element_layout::{ElementReq, ElementAlloc};
use elements::element_ctx::{ElementContext};
use elements::element::{TElement, ElementRef, ElementParentMut};
use elements::container::{TContainerElement};
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use elements::root_element::{TRootElement};


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
    colour: Colour,
}

impl TextStyleParams {
    pub fn new(font_family: String, weight: TextWeight, slant: TextSlant,
               size: f64, colour: Colour) -> TextStyleParams {
        return TextStyleParams{font_family: font_family, weight: weight, slant: slant, size: size,
                               colour: colour};
    }

    pub fn default() -> TextStyleParams {
        return TextStyleParams{font_family: String::from("Sans serif"),
                               weight: TextWeight::Normal, slant: TextSlant::Normal,
                               size: 14.0, colour: BLACK};
    }

    pub fn with_family(font_family: String) -> TextStyleParams {
        return TextStyleParams{font_family: font_family,
                               weight: TextWeight::Normal, slant: TextSlant::Normal,
                               size: 14.0, colour: BLACK};
    }

    pub fn with_family_and_colour(font_family: String, colour: Colour) -> TextStyleParams {
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
        cairo_ctx.set_source_rgba(self.colour.r as f64, self.colour.g as f64, self.colour.b as f64,
                                  self.colour.a as f64);
    }
}

pub type TextReqKey = (String, TextWeight, TextSlant, i64, String);




struct TextElementMut {
    parent: ElementParentMut,
    req: Rc<ElementReq>,
    alloc: ElementAlloc,
    text: String,
}


pub struct TextElement {
    style: Rc<TextStyleParams>,
    m: RefCell<TextElementMut>,
}

impl TextElement {
    pub fn new(text: String, style: Rc<TextStyleParams>, cairo_ctx: &Context,
               elem_ctx: &RefCell<ElementContext>) -> TextElement {
        let req = elem_ctx.borrow_mut().text_shared_req(style.clone(), text.clone(), cairo_ctx);
        return TextElement{style: style,
                           m: RefCell::new(TextElementMut{
                                parent: ElementParentMut::new(),
                                req: req,
                                alloc: ElementAlloc::new(),
                                text: text}),
                           };
    }
}

impl TElement for TextElement {
    /// Interface acquisition
    fn as_container(&self) -> Option<&TContainerElement> {
        return None;
    }

    fn as_bin(&self) -> Option<&TBinElement> {
        return None
    }

    fn as_container_sequence(&self) -> Option<&TContainerSequenceElement> {
        return None
    }

    fn as_root_element(&self) -> Option<&TRootElement> {
        return None;
    }



    /// Parent get and set methods
    fn get_parent(&self) -> Option<ElementRef> {
        return self.m.borrow().parent.get().clone();
    }

    fn set_parent(&self, p: Option<&ElementRef>) {
        self.m.borrow_mut().parent.set(p);
    }


    fn element_req(&self) -> Ref<ElementReq> {
        return Ref::map(self.m.borrow(), |m| &(*m.req));
    }

    fn element_alloc(&self) -> Ref<ElementAlloc> {
        return Ref::map(self.m.borrow(), |m| &m.alloc);
    }

    /// Update element X allocation
    fn element_update_x_alloc(&self, x_alloc: &LAlloc) {
        self.m.borrow_mut().alloc.x_alloc.clone_from(x_alloc);
    }
    /// Update element Y allocation
    fn element_update_y_alloc(&self, y_alloc: &LAlloc) {
        self.m.borrow_mut().alloc.y_alloc.clone_from(y_alloc);
    }

    fn draw_self(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        let mm = self.m.borrow();
        let y = match mm.alloc.y_alloc.ref_point() {
            None => 0.0,
            Some(ref_point) => ref_point
        };
        cairo_ctx.move_to(0.0, y);
        self.style.apply(cairo_ctx);
        cairo_ctx.show_text(mm.text.as_str());
    }

    fn draw(&self, cairo_ctx: &Context, visible_region: &BBox2) {
        self.draw_self(cairo_ctx, visible_region);
    }

    fn update_x_req(&self) {
        // Nothing to do; requisition is shared
    }

    fn allocate_x(&self) {
        // Nothing to do; no children
    }

    fn update_y_req(&self) {
        // Nothing to do; requisition is shared
    }

    fn allocate_y(&self) {
        // Nothing to do; no children
    }
}
