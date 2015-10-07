use std::rc::Rc;

use pres::pres::{Pres};
use pres::primitive::{Text, Flow};
use elements::text_element::TextStyleParams;


pub fn paragraph(text: &String, style: &Rc<TextStyleParams>) -> Pres {
    let words = text.split(" ");
    let mut first = false;
    let mut pres_words: Vec<Pres> = Vec::new();
    for w in words {
        let ws = String::from(w);
        if !first {
            pres_words.push(Text::new(String::from(" "), style.clone()));
        }
        pres_words.push(Text::new(ws, style.clone()));
    }

    return Flow::new(pres_words);
}
