#![feature(convert)]
#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate time;
extern crate gtk;
extern crate cairo;
extern crate lspace;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;
use std::string::String;
use std::rc::Rc;
use std::env;

use rustc_serialize::json::Json;

use gtk::traits::*;
use gtk::signal::Inhibit;

use lspace::geom::colour::Colour;
use lspace::layout::flow_layout::FlowIndent;
use lspace::elements::text_element::{TextStyleParams};
use lspace::pres::pres::Pres;
use lspace::pres::primitive::{Column, Row, Flow, Text};
use lspace::lspace_area::LSpaceArea;


/// Convert JSON representation of FlowIndent to FlowIndent
fn json_to_flow_indent(j: &Json) -> FlowIndent {
    let obj = j.as_object().unwrap();
    let indent_type = obj.get("indent_type").unwrap().as_string().unwrap();
    match indent_type {
        "no_indent" => FlowIndent::NoIndent,
        "first" => FlowIndent::First{indent: obj.get("indent").unwrap().as_f64().unwrap()},
        "except_first" => FlowIndent::ExceptFirst{indent: obj.get("indent").unwrap().as_f64().unwrap()},
        _ => panic!(format!("Unknown FlowIndent indent_type - {}", indent_type))
    }
}

/// Convert Json input data to presentation types
pub fn json_to_pres(j: &Json, style: &Rc<TextStyleParams>) -> Pres {
    let obj = j.as_object().unwrap();
    let obj_type = obj.get("__type__").unwrap().as_string().unwrap();
    match obj_type {
        "Text" => Text::new(obj.get("text").unwrap().as_string().unwrap().to_string(), style.clone()),
        "Column" => {
            Column::new_full(obj.get("children").unwrap().as_array().unwrap().iter().map(|x| json_to_pres(&x, style)).collect(),
                             obj.get("y_spacing").unwrap().as_f64().unwrap(),
            )},
        "Row" => {
            Row::new_full(obj.get("children").unwrap().as_array().unwrap().iter().map(|x| json_to_pres(&x, style)).collect(),
                             obj.get("x_spacing").unwrap().as_f64().unwrap(),
            )},
        "Flow" => {
            Flow::new_full(obj.get("children").unwrap().as_array().unwrap().iter().map(|x| json_to_pres(&x, style)).collect(),
                             obj.get("x_spacing").unwrap().as_f64().unwrap(),
                             obj.get("y_spacing").unwrap().as_f64().unwrap(),
                             json_to_flow_indent(&obj.get("indentation").unwrap()),
            )},
        _ => panic!(format!("Unknown __type__ - {}", obj_type))
    }
}


fn main() {
    // Initialise GTK
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    // String buffer for JSON data
    let mut encoded: String = String::new();

    // Get the path of the file to load from the command line arguments, or read data from STDIN
    // if not path was provided
    let mut args = env::args();
    args.next();
    match args.next() {
        None => {
            // No path; read from STDIN
            io::stdin().read_to_string(&mut encoded).unwrap();
        },
        Some(path) =>  {
            // Load the file
            println!("Loading {}....", path);
            let mut f = File::open(path).unwrap();
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut encoded).unwrap();
        }
    };

    // Load the JSON content and decode
    let j = Json::from_str(encoded.as_str()).unwrap();

    // Create the presentation from the loaded structure
    println!("Creating presentation...");
    let style = Rc::new(TextStyleParams::with_family_and_colour(String::from("Courier New"),
                            Colour::new(0.1, 0.225, 0.35, 1.0)));
    let content = json_to_pres(&j, &style);

    // Create the LSpace area, showing our content
    println!("Displaying....");
    let area = LSpaceArea::new(content);

    // Create a GTK window in which to place it
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("JSON presentation viewer");
    window.add(area.borrow().gtk_widget());
    window.set_default_size(800, 500);

    // Quit the GTK main loop when the window is closed
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    // Show everything
    window.show_all();

    // Enter GTK main loop
    gtk::main();
}

