#![feature(test)]
#![feature(slice_patterns)]
#![feature(convert)]
#![feature(associated_type_defaults)]

extern crate cairo;
extern crate cairo_sys;
extern crate gdk;
extern crate gtk;

pub mod geom;
pub mod layout;
pub mod elements;
pub mod pres;

pub mod lspace_area;
