use std;
use std::cell::Ref;

use cairo::Context;

use geom::fastminmax::{fast_min, fast_max};
use geom::colour::{Colour, PyColour};
use pyrs::{PyWrapper, PyRcWrapper};


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Border {
    SolidBorder{
        thickness: f64,
        inset: f64,
        rounding: f64,
        border_colour: Colour,
        background_colour: Option<Colour>
    },
    FilledBorder{
        left_margin: f64,
        right_margin: f64,
        top_margin: f64,
        bottom_margin: f64,
        rounding: f64,
        background_colour: Option<Colour>
    }
}


impl Border {
    pub fn new_solid(thickness: f64, inset: f64, rounding: f64,
                     border_colour: &Colour, background_colour: Option<&Colour>) -> Border {
        Border::SolidBorder{thickness: thickness, inset: inset, rounding: rounding,
            border_colour: border_colour.clone(),
            background_colour: background_colour.map(|x| x.clone())}
    }

    pub fn new_filled(left_margin: f64, right_margin: f64,
                      top_margin: f64, bottom_margin: f64, rounding: f64,
                      background_colour: Option<&Colour>) -> Border {
        Border::FilledBorder{left_margin: left_margin, right_margin: right_margin,
            top_margin: top_margin, bottom_margin: bottom_margin, rounding: rounding,
            background_colour: background_colour.map(|x| x.clone())}
    }


    pub fn left_margin(&self) -> f64 {
        match self {
            &Border::SolidBorder{thickness: t, inset: i, ..} => t + i,
            &Border::FilledBorder{left_margin: m, ..} => m,
        }
    }

    pub fn right_margin(&self) -> f64 {
        match self {
            &Border::SolidBorder{thickness: t, inset: i, ..} => t + i,
            &Border::FilledBorder{right_margin: m, ..} => m,
        }
    }

    pub fn top_margin(&self) -> f64 {
        match self {
            &Border::SolidBorder{thickness: t, inset: i, ..} => t + i,
            &Border::FilledBorder{top_margin: m, ..} => m,
        }
    }

    pub fn bottom_margin(&self) -> f64
    {
        match self {
            &Border::SolidBorder{thickness: t, inset: i, ..} => t + i,
            &Border::FilledBorder{bottom_margin: m, ..} => m,
        }
    }

    pub fn add_clip_path(&self, cairo_ctx: &Context, x: f64, y: f64, w: f64, h: f64) {
        match self {
            &Border::SolidBorder{thickness: t, rounding: r, ..} => {
                Border::border_path(cairo_ctx, x + t * 0.5, y + t * 0.5, w - t, h - t, r);
            },
            &Border::FilledBorder{rounding: r, ..} => {
                Border::border_path(cairo_ctx, x, y, w, h, r);
            }
        };
    }

    pub fn draw(&self, cairo_ctx: &Context, x: f64, y: f64, w: f64, h: f64) {
        match self {
            &Border::SolidBorder{thickness: t, rounding: r, border_colour: col, ..} => {
                // The preferred approach would be to call get_source() to get the current
                // source (paint colour / pattern / gradient / etc), take a copy before we set
                // our target colour, do our drawing, then restore the original.
                // Unfortunately a bug in the Rust bindings prevents this from working, so for
                // now we just set it to black (default) when we are done.
                // More correct code is:
                // ```let prev_source = cairo_ctx.get_source();```
                // then at the bottom, instead of:
                // ```cairo_ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);```
                // use:
                // ```cairo_ctx.set_source(&*prev_source);```
                let prev_width = cairo_ctx.get_line_width();

                cairo_ctx.new_path();
                Border::border_path(cairo_ctx, x + t * 0.5, y + t * 0.5, w - t, h - t, r);
                cairo_ctx.set_line_width(t);
                cairo_ctx.set_source_rgba(col.r as f64, col.g as f64, col.b as f64, col.a as f64);
                cairo_ctx.stroke();

                cairo_ctx.set_line_width(prev_width);
                cairo_ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            },
            _ => {}
        };
    }

    pub fn draw_background(&self, cairo_ctx: &Context, x: f64, y: f64, w: f64, h: f64) {
        match self {
            &Border::SolidBorder{thickness: t, rounding: r, background_colour: Some(col), ..} => {
                // Please see comments in draw() method concerning correct 'source' handling.

                cairo_ctx.new_path();
                Border::border_path(cairo_ctx, x + t * 0.5, y + t * 0.5, w - t, h - t, r);
                cairo_ctx.set_source_rgba(col.r as f64, col.g as f64, col.b as f64, col.a as f64);
                cairo_ctx.fill();

                cairo_ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            },
            &Border::FilledBorder{rounding: r, background_colour: Some(col), ..} => {
                // Please see comments in draw() method concerning correct 'source' handling.

                cairo_ctx.new_path();
                Border::border_path(cairo_ctx, x, y, w, h, r);
                cairo_ctx.set_source_rgba(col.r as f64, col.g as f64, col.b as f64, col.a as f64);
                cairo_ctx.fill();

                cairo_ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            },
            _ => {}
        };
    }


    fn border_path(cairo_ctx: &Context, x: f64, y: f64, w: f64, h: f64,
                   rounding: f64) {
        if rounding == 0.0 {
            cairo_ctx.rectangle(x, y, w, h);
        }
        else {
            // Ensure rounding radius is not so large that the rounded arcs would overlap
            // one another
            let r = fast_min(fast_min(rounding, w * 0.5), h * 0.5);
            // Compute horizontal and vertical line lengths
            let inner_w = w - r * 2.0;
            let inner_h = h - r * 2.0;

            // Top left arc, counter-clockwise
            cairo_ctx.move_to(x + r, y);
            cairo_ctx.arc_negative(x + r, y + r, r, std::f64::consts::PI*3.0/2.0, std::f64::consts::PI);

            // Left vertical line
            if inner_h > 0.0 {
                cairo_ctx.line_to(x, y + h - r);
            }

            // Bottom left arc
            cairo_ctx.arc_negative(x + r, y + h - r, r, std::f64::consts::PI, std::f64::consts::PI/2.0);

            // Bottom horizontal line
            if inner_w > 0.0 {
                cairo_ctx.line_to(x + w - r, y + h);
            }

            // Bottom right arc
            cairo_ctx.arc_negative(x + w - r, y + h - r, r, std::f64::consts::PI/2.0, 0.0);

            // Right vertical line
            if inner_h > 0.0 {
                cairo_ctx.line_to(x + w, y + r);
            }

            // Top right arc
            cairo_ctx.arc_negative(x + w - r, y + r, r, 0.0, std::f64::consts::PI*3.0/2.0);

            // Close
            cairo_ctx.close_path();
        }
    }
}


pub type PyBorder = PyRcWrapper<Border>;
pub type PyBorderOwned = Box<PyBorder>;

// Function exported to Python for creating a boxed `Border`, `SolidBorder` variant
#[no_mangle]
pub extern "C" fn new_solid_border(thickness: f64, inset: f64, rounding: f64,
                                   border_colour: &PyColour,
                                   background_colour_optional: *mut PyColour)
                                        -> PyBorderOwned {
    let border = match PyColour::borrow_optional(background_colour_optional) {
        None =>
            Border::new_solid(thickness, inset, rounding, &*PyColour::borrow(border_colour),
                              None),
        Some(ref_box_backg) =>
            Border::new_solid(thickness, inset, rounding, &*PyColour::borrow(border_colour),
                              Some(&ref_box_backg)),
    };
    Box::new(PyBorder::from_value(border))
}

// Function exported to Python for creating a boxed `Border`, `FilledBorder` variant
#[no_mangle]
pub extern "C" fn new_filled_border(left_margin: f64, right_margin: f64,
                                    top_margin: f64, bottom_margin: f64, rounding: f64,
                                    background_colour_optional: *mut PyColour)
                                        -> PyBorderOwned {
    let border = match PyColour::borrow_optional(background_colour_optional) {
        None => Border::new_filled(left_margin, right_margin, top_margin, bottom_margin,
                                   rounding, None),
        Some(ref_box_backg) => Border::new_filled(left_margin, right_margin, top_margin,
                                                  bottom_margin, rounding,
                                                  Some(&ref_box_backg)),
    };
    Box::new(PyBorder::from_value(border))
}

#[no_mangle]
pub extern "C" fn destroy_gfx_border(wrapper: PyBorderOwned) {
    PyBorder::destroy(wrapper);
}


