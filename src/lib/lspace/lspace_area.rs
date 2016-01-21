#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate time;

use cairo::{Context, RectangleInt};

use geom::vector2::Vector2;
use geom::point2::Point2;
use geom::bbox2::BBox2;
use input::inputmodifier::InputModifierState;
use input::keyboard::Keyboard;
use input::pointer::{Pointer, PointerPosition};
use elements::element_ctx::ElementContext;
use elements::element::ElementRef;
use pres::pres::{Pres, PresBuildCtx};
use pres::primitive::root_containing;


pub struct LSpaceArea {
    width: i32,
    height: i32,

    input_mods: InputModifierState,
    input_keyboard: Keyboard,
    input_pointer: Pointer,

    elem_ctx: ElementContext,

    content: Pres,

    elem: Option<ElementRef>,

    initialised: bool,
    layout_required: bool
}

impl LSpaceArea {
    pub fn new(content: Pres) -> LSpaceArea {
        return LSpaceArea{width: 100, height: 100,
            input_mods: InputModifierState::new(),
            input_keyboard: Keyboard::new(),
            input_pointer: Pointer::new(),
            content: content,
            elem_ctx: ElementContext::new(),
            elem: None,
            initialised: false,
            layout_required: true};
    }

    fn new_document_in_root(&mut self, cairo_ctx: &Context) -> ElementRef {
        let pres_ctx = PresBuildCtx::new(&self.elem_ctx, cairo_ctx);
        let root_elem = root_containing(&self.content, &pres_ctx);
        return root_elem;
    }

    fn initialise(&mut self, cairo_ctx: &Context) {
        if !self.initialised {
            cairo_ctx.save();
            match &self.elem {
                &None => {
                    self.elem = Some(self.new_document_in_root(cairo_ctx));
                },
                &_ => {}
            };
            cairo_ctx.restore();

            self.initialised = true;
        }
    }

    pub fn on_realize(&mut self) {
    }

    pub fn on_unrealize(&mut self) {
    }

    pub fn on_size_allocate(&mut self, rect: &RectangleInt) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;

        self.layout_required = true;
    }

    pub fn on_button_press(&mut self, mod_state: InputModifierState, pos: Point2, button: u32) {
        self.input_mods = mod_state;
        self.input_pointer.set_position(PointerPosition::at_position(pos));
    }

    pub fn on_button_release(&mut self, mod_state: InputModifierState, pos: Point2, button: u32) {
        self.input_mods = mod_state;
        self.input_pointer.set_position(PointerPosition::at_position(pos));
    }

    pub fn on_enter(&mut self, mod_state: InputModifierState, pos: Point2) {
        self.input_mods = mod_state;
        self.input_pointer.set_position(PointerPosition::at_position(pos));
    }

    pub fn on_leave(&mut self, mod_state: InputModifierState, pos: Point2) {
        self.input_mods = mod_state;
        self.input_pointer.set_position(PointerPosition::out_of_bounds());
    }

    pub fn on_motion(&mut self, mod_state: InputModifierState, pos: Point2) {
        self.input_mods = mod_state;
        self.input_pointer.set_position(PointerPosition::at_position(pos));
    }

    pub fn on_scroll(&mut self, mod_state: InputModifierState, pos: Point2,
                 scroll_x: f64, scroll_y: f64) {
        self.input_mods = mod_state;
        self.input_pointer.set_position(PointerPosition::at_position(pos));
    }

    pub fn on_key_press(&mut self, mod_state: InputModifierState, key_val: u32, key_string: String) {
        self.input_mods = mod_state;
    }

    pub fn on_key_release(&mut self, mod_state: InputModifierState, key_val: u32, key_string: String) {
        self.input_mods = mod_state;
    }

    pub fn on_draw(&mut self, cairo_ctx: Context) {
        self.initialise(&cairo_ctx);
        self.layout();
        self.draw(&cairo_ctx);
    }

    fn layout(&mut self) {
        if self.layout_required {
            match &self.elem {
                &Some(ref re) => {
                    let e = re.as_root_element().unwrap();
                    let t1 = time::precise_time_ns();
                    let rx = e.root_requisition_x();
                    e.root_allocate_x(self.width as f64);
                    let ry = e.root_requisition_y();
                    e.root_allocate_y(ry);
                    let t2 = time::precise_time_ns();
                    println!("Layout time: {}", (t2-t1) as f64 * 1.0e-9);
                },
                &None => {}
            }
            self.layout_required = false;
        }
    }

    fn draw(&self, cairo_ctx: &Context) {
        match &self.elem {
            &Some(ref re) => {
                let e = re.as_root_element().unwrap();
                let t1 = time::precise_time_ns();
                e.draw(cairo_ctx, &BBox2::from_lower_size(Point2::origin(),
                        Vector2::new(self.width as f64, self.height as f64)));
                let t2 = time::precise_time_ns();
                println!("Render time: {}", (t2-t1) as f64 * 1.0e-9);
            },
            &None => {}
        }
    }
}
