use pres::pres::{Pres};
use pres::primitive::{Text, Flow};


pub fn paragraph(text: &String) -> Pres {
    let words = text.split(" ");
    let mut first = false;
    let mut pres_words: Vec<Pres> = Vec::new();
    for w in words {
        let ws = String::from(w);
        if !first {
            pres_words.push(Text::new(String::from(" ")));
        }
        pres_words.push(Text::new(ws));
    }

    return Flow::new(pres_words);
}
