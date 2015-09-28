#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]
#![feature(path_ext)]
#![feature(convert)]

extern crate time;
extern crate gtk;
extern crate cairo;
extern crate lspace;
extern crate hyper;

use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::fs::File;
use std::path::Path;
use std::string::String;

use hyper::Client;
use hyper::header::Connection;

use gtk::traits::*;
use gtk::signal::Inhibit;

use lspace::pres::pres::Pres;
use lspace::pres::primitive::Column;
use lspace::pres::richtext::paragraph;
use lspace::lspace_area::LSpaceArea;

const DOWNLOAD_URL: &'static str = "http://www.gutenberg.org/files/2600/2600.txt";
const FORMATTED_FILENAME: &'static str = "war_and_peace_formatted.txt";

fn main() {
    // Initialise GTK
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());

    // Download War and Peace if necessary
    if !Path::new(FORMATTED_FILENAME).exists() {
        println!("Downloading War and Peace text...");

        let mut all_bytes: Vec<u8> = Vec::new();

        {
            let client = Client::new();
            let mut res = client.get(DOWNLOAD_URL)
                .header(Connection::close())
                .send().unwrap();

            let mut reader = BufReader::new(res);
            reader.read_to_end(&mut all_bytes);
        }

        {
            let mut f = File::create(FORMATTED_FILENAME).unwrap();
            let mut writer = BufWriter::new(f);
            writer.write_all(&all_bytes);
        }
    }

    println!("Loading War and Peace....");

    // Load the text of War and Peace
    let mut paragraphs: Vec<String> = Vec::new();
    let mut current_paragraph_lines: Vec<String> = Vec::new();
    let mut f = File::open(FORMATTED_FILENAME).unwrap();
    let mut reader = BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(line_string) => {
                // Remove leading and trailing whitespace
                let l = line_string.trim();

                if l.len() == 0 {
                    // Empty line; finish previous paragraph
                    {
                        let xs: Vec<&str> = current_paragraph_lines.iter().map(|x| x.as_str()).collect();
                        paragraphs.push(xs.join(" "));
                    }
                    current_paragraph_lines.clear();
                } else {
                    current_paragraph_lines.push(String::from(l));
                }
            },
            Err(..) => {panic!();}
        }
    }

    if current_paragraph_lines.len() > 0 {
        // Last paragraph
        let xs: Vec<&str> = current_paragraph_lines.iter().map(|x| x.as_str()).collect();
        paragraphs.push(xs.join(" "));
    }

    println!("Loaded War and Peace ({} paragraphs); creating presentation...", paragraphs.len());

    // Create a presentation of the text using the `lspace.pres` API
    let mut pres_paragraphs: Vec<Pres> = Vec::new();
    for ref para_text in paragraphs {
        pres_paragraphs.push(paragraph(para_text));
    }
    let content = Column::new(pres_paragraphs);

    println!("Loaded War and Peace; displaying....");

    // Create the LSpace area, showing our content
    let area = LSpaceArea::new(content);

    // Create a GTK window in which to place it
    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("Cairo API test");
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
