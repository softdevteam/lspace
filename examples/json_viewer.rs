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

use rustc_serialize::json::{self, Json};

use gtk::traits::*;
use gtk::signal::Inhibit;

use lspace::graphics::border::Border;
use lspace::geom::colour::Colour;
use lspace::layout::flow_layout::FlowIndent;
use lspace::elements::text_element::{TextStyleParams, TextWeight, TextSlant};
use lspace::pres::pres::Pres;
use lspace::pres::primitive;
use lspace::lspace_area::LSpaceArea;


/// Style sheet
struct StyleSheet {
    text: Rc<TextStyleParams>,

    column_y_spacing: f64,

    row_x_spacing: f64,

    flow_x_spacing: f64,
    flow_y_spacing: f64,
    flow_indentation: FlowIndent,
}

impl StyleSheet {
    fn default() -> StyleSheet {
        StyleSheet{
            text: Rc::new(TextStyleParams::new("Sans serif".to_string(),
                TextWeight::Normal, TextSlant::Normal, 12.0,
                Colour{r: 0.0, g: 0.0, b: 0.0, a: 1.0})),

            column_y_spacing: 0.0,

            row_x_spacing: 0.0,

            flow_x_spacing: 0.0,
            flow_y_spacing: 0.0,
            flow_indentation: FlowIndent::NoIndent,
        }

    }

    fn with_values(&self,
                   text_font_family: Option<String>,
                   text_weight: Option<TextWeight>,
                   text_slant: Option<TextSlant>,
                   text_size: Option<f64>,
                   text_colour: Option<Colour>,

                   column_y_spacing: Option<f64>,

                   row_x_spacing: Option<f64>,

                   flow_x_spacing: Option<f64>,
                   flow_y_spacing: Option<f64>,
                   flow_indentation: Option<FlowIndent>) -> StyleSheet {
        let text = match (text_font_family, text_weight, text_slant, text_size, text_colour) {
            (None, None, None, None, None) => self.text.clone(),
            (a, b, c, d, e) => Rc::new(TextStyleParams{
                font_family: a.map_or(self.text.font_family.clone(), |v| v.clone()),
                weight: b.unwrap_or(self.text.weight),
                slant: c.unwrap_or(self.text.slant),
                size: d.unwrap_or(self.text.size),
                colour: e.unwrap_or(self.text.colour)})
        };

        StyleSheet{
            text: text,

            column_y_spacing: column_y_spacing.unwrap_or(self.column_y_spacing),

            row_x_spacing: row_x_spacing.unwrap_or(self.row_x_spacing),

            flow_x_spacing: flow_x_spacing.unwrap_or(self.flow_x_spacing),
            flow_y_spacing: flow_y_spacing.unwrap_or(self.flow_y_spacing),
            flow_indentation: flow_indentation.unwrap_or(self.flow_indentation),
        }
    }

    fn use_column(&self) -> StyleSheet {
        StyleSheet{
            text: self.text.clone(),
            column_y_spacing: 0.0,
            row_x_spacing: self.row_x_spacing,
            flow_x_spacing: self.flow_x_spacing,
            flow_y_spacing: self.flow_y_spacing,
            flow_indentation: self.flow_indentation
        }
    }

    fn use_row(&self) -> StyleSheet {
        StyleSheet{
            text: self.text.clone(),
            column_y_spacing: self.column_y_spacing,
            row_x_spacing: 0.0,
            flow_x_spacing: self.flow_x_spacing,
            flow_y_spacing: self.flow_y_spacing,
            flow_indentation: self.flow_indentation
        }
    }

    fn use_flow(&self) -> StyleSheet {
        StyleSheet{
            text: self.text.clone(),
            column_y_spacing: self.column_y_spacing,
            row_x_spacing: self.row_x_spacing,
            flow_x_spacing: 0.0,
            flow_y_spacing: 0.0,
            flow_indentation: FlowIndent::NoIndent
        }
    }
}


/// Helper function for converting JSON data to a Rust struct
fn json_to_struct<T, F>(j: &Json, field_names: &Vec<&str>, convert: F) -> T
        where F: Fn(&Vec<&Json>, &json::Object) -> T {
    let obj = j.as_object().unwrap();
    let field_values = field_names.iter().map(|x|
            obj.get(&x.to_string()).unwrap()).collect();
    convert(&field_values, obj)
}

/// Helper function for converting JSON data to a Rust enum
fn json_to_enum_struct<T, F>(j: &Json, field_name: &str, convert: F) -> T
        where F: Fn(&str, &json::Object) -> T {
    let obj = j.as_object().unwrap();
    let string_value = obj.get(field_name).unwrap().as_string().unwrap();
    convert(string_value, obj)
}

/// Helper function for converting JSON data to a Rust enum
fn json_str_to_enum<T, F>(j: &Json, convert: F) -> T
        where F: Fn(&str) -> T {
    let string_value = j.as_string().unwrap();
    convert(string_value)
}

/// Convert JSON representation TextWeight
fn json_to_text_weight(j: &Json) -> TextWeight {
    json_str_to_enum(j, |str_value| {
        match str_value {
            "normal" => TextWeight::Normal,
            "bold" => TextWeight::Bold,
            _ => panic!(format!("Unknown TextWeight weight - {}", str_value))
        }
    })
}

/// Convert JSON representation TextSlant
fn json_to_text_slant(j: &Json) -> TextSlant {
    json_str_to_enum(j, |str_value| {
        match str_value {
            "normal" => TextSlant::Normal,
            "italic" => TextSlant::Italic,
            _ => panic!(format!("Unknown TextSlant slant - {}", str_value))
        }
    })
}

/// Convert JSON representation Colour
fn json_to_colour(j: &Json) -> Colour {
    json_to_struct(j, &vec!["r", "g", "b", "a"], |vals, obj| {
        Colour::new(vals[0].as_f64().unwrap() as f32, vals[1].as_f64().unwrap() as f32,
                    vals[2].as_f64().unwrap() as f32, vals[3].as_f64().unwrap() as f32)
    })
}

/// Convert JSON representation FlowIndent
fn json_to_flow_indent(j: &Json) -> FlowIndent {
    json_to_enum_struct(j, "indent_type", |str_value, obj| {
        match str_value {
            "no_indent" => FlowIndent::NoIndent,
            "first" => FlowIndent::First{indent: obj.get("indent").unwrap().as_f64().unwrap()},
            "except_first" =>
                    FlowIndent::ExceptFirst{indent: obj.get("indent").unwrap().as_f64().unwrap()},
            _ => panic!(format!("Unknown FlowIndent indent_type - {}", str_value))
        }
    })
}

/// Convert JSON representation Border
fn json_to_border(j: &Json) -> Border
{
    json_to_enum_struct(j, "border_type", |str_value, obj| {
        match str_value {
            "solid" => Border::new_solid(
                obj.get("thickness").unwrap().as_f64().unwrap(),
                obj.get("inset").unwrap().as_f64().unwrap(),
                obj.get("rounding").unwrap().as_f64().unwrap(),
                json_to_colour(obj.get("colour").unwrap()),
                obj.get("background_colour").map(|x| json_to_colour(x))
            ),
            "filled" => Border::new_filled(
                obj.get("left_margin").unwrap().as_f64().unwrap(),
                obj.get("right_margin").unwrap().as_f64().unwrap(),
                obj.get("top_margin").unwrap().as_f64().unwrap(),
                obj.get("bottom_margin").unwrap().as_f64().unwrap(),
                obj.get("rounding").unwrap().as_f64().unwrap(),
                obj.get("background_colour").map(|x| json_to_colour(x))
            ),
            _ => panic!(format!("Unknown Border border_type - {}", str_value))
        }
    })
}

/// Convert Json input data to presentation types
fn json_to_pres(j: &Json, style: &StyleSheet) -> Pres {
    let obj = j.as_object().unwrap();
    let obj_type = obj.get("__type__").unwrap().as_string().unwrap();
    match obj_type {
        "Text" => primitive::Text::new(obj.get("text").unwrap().as_string().unwrap().to_string(),
                            style.text.clone()),
        "Column" => {
            let children = obj.get("children").unwrap().as_array().unwrap().iter().map(|x|
                    json_to_pres(&x, &style.use_column())).collect();
            primitive::Column::new_full(children, style.column_y_spacing)
        },
        "Row" => {
            let children = obj.get("children").unwrap().as_array().unwrap().iter().map(|x|
                    json_to_pres(&x, &style.use_row())).collect();
            primitive::Row::new_full(children, style.row_x_spacing)
        },
        "Flow" => {
            let children = obj.get("children").unwrap().as_array().unwrap().iter().map(|x|
                    json_to_pres(&x, &style.use_flow())).collect();
            primitive::Flow::new_full(children, style.flow_x_spacing, style.flow_y_spacing,
                           style.flow_indentation)
        },
        "Border" => {
            let child = json_to_pres(&obj.get("child").unwrap(), &style);
            let border = Rc::new(json_to_border(j));
            primitive::Border::new(child, &border)
        },
        "ApplyStyleSheet" => {
            json_to_pres(obj.get("child").unwrap(), &style.with_values(
                obj.get("text_font_family").and_then(|x| x.as_string()).map(|x| x.to_string()),
                obj.get("text_weight").map(|x| json_to_text_weight(x)),
                obj.get("text_slant").map(|x| json_to_text_slant(x)),
                obj.get("text_size").and_then(|x| x.as_f64()),
                obj.get("text_colour").map(|x| json_to_colour(x)),

                obj.get("column_y_spacing").and_then(|x| x.as_f64()),

                obj.get("row_x_spacing").and_then(|x| x.as_f64()),

                obj.get("flow_x_spacing").and_then(|x| x.as_f64()),
                obj.get("flow_y_spacing").and_then(|x| x.as_f64()),
                obj.get("flow_indentation").map(|x| json_to_flow_indent(x)),
            ))
        }
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
    let style = StyleSheet::default();
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

