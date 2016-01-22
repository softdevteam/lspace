
use std::rc::Rc;
use std::ffi::CStr;
use libc::c_char;

use graphics::border;
use layout::flow_layout;
use elements::element::{TElement, ElementRef, elem_as_ref};
use elements::{text_element, column, row, flow, root_element, border_element};
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use geom::colour::Colour;
use pres::pres::{Pres, TPres, PresBuildCtx};
use pyrs::PyWrapper;


pub struct Text {
    text: String,
    style: Rc<text_element::TextStyleParams>,
}

impl Text {
    pub fn new(text: String, style: Rc<text_element::TextStyleParams>) -> Pres {
        return Box::new(Text{text: text, style: style});
    }
}

impl TPres for Text {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let elem = text_element::TextElement::new(self.text.clone(), self.style.clone(),
                                                  pres_ctx.cairo_ctx, &pres_ctx.elem_ctx);
        return elem_as_ref(elem);
    }
}


pub struct Border {
    child: Pres,
    border: Rc<border::Border>
}

impl Border {
    pub fn new(child: Pres, border: &Rc<border::Border>) -> Pres {
        Box::new(Border{child: child, border: border.clone()})
    }
}

impl TPres for Border {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child = self.child.build(pres_ctx);
        let elem = elem_as_ref(border_element::BorderElement::new(&self.border));
        elem.as_bin().unwrap().set_child(&elem, child);
        elem
    }
}


pub struct Column {
    children: Vec<Pres>,
    y_spacing: f64,
}

impl Column {
    pub fn new(children: Vec<Pres>) -> Pres {
        return Box::new(Column{children: children, y_spacing: 0.0});
    }

    pub fn new_full(children: Vec<Pres>, y_spacing: f64) -> Pres {
        return Box::new(Column{children: children, y_spacing: y_spacing});
    }
}

impl TPres for Column {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child_elems = self.children.iter().map(|p| p.build(pres_ctx)).collect();
        let elem = elem_as_ref(column::ColumnElement::new(self.y_spacing));
        elem.as_container_sequence().unwrap().set_children(&elem, &child_elems);
        return elem;
    }
}


pub struct Row {
    children: Vec<Pres>,
    x_spacing: f64,
}

impl Row {
    pub fn new(children: Vec<Pres>) -> Pres {
        return Box::new(Row{children: children, x_spacing: 0.0});
    }

    pub fn new_full(children: Vec<Pres>, x_spacing: f64) -> Pres {
        return Box::new(Row{children: children, x_spacing: x_spacing});
    }
}

impl TPres for Row {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child_elems = self.children.iter().map(|p| p.build(pres_ctx)).collect();
        let elem = elem_as_ref(row::RowElement::new(self.x_spacing));
        elem.as_container_sequence().unwrap().set_children(&elem, &child_elems);
        return elem;
    }
}


pub struct Flow {
    children: Vec<Pres>,
    x_spacing: f64,
    y_spacing: f64,
    indentation: flow_layout::FlowIndent,
}

impl Flow {
    pub fn new(children: Vec<Pres>) -> Pres {
        return Box::new(Flow{children: children, x_spacing: 0.0, y_spacing: 0.0,
                             indentation: flow_layout::FlowIndent::NoIndent});
    }

    pub fn new_full(children: Vec<Pres>, x_spacing: f64, y_spacing: f64,
                    indentation: flow_layout::FlowIndent) -> Pres {
        return Box::new(Flow{children: children, x_spacing: x_spacing, y_spacing: y_spacing,
                        indentation: indentation});
    }
}

impl TPres for Flow {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child_elems = self.children.iter().map(|p| p.build(pres_ctx)).collect();
        let elem = elem_as_ref(flow::FlowElement::new(self.x_spacing, self.y_spacing,
                                                      self.indentation));
        elem.as_container_sequence().unwrap().set_children(&elem, &child_elems);
        return elem;
    }
}


pub fn root_containing(p: &Pres, ctx: &PresBuildCtx) -> ElementRef {
    let child = p.build(ctx);
    let elem = elem_as_ref(root_element::RootElement::new());
    elem.as_bin().unwrap().set_child(&elem, child);
    return elem;
}




pub struct TextStyleRef {
    val: Rc<text_element::TextStyleParams>
}




// Function exported to Python for creating a boxed `Colour`
#[no_mangle]
pub extern "C" fn new_colour(r: f64, g: f64, b: f64, a: f64) -> Box<PyWrapper<Colour>> {
    Box::new(PyWrapper::new(Colour::new(r as f32, g as f32, b as f32, b as f32)))
}

#[no_mangle]
pub extern "C" fn destroy_colour(wrapper: Box<PyWrapper<Colour>>) {
    wrapper.destroy();
}


// Function exported to Python for creating a boxed `TextStyleParams`
#[no_mangle]
pub extern "C" fn new_text_style_params(font_family: *mut c_char, bold: u16, italic: u16,
                                        size: f64, colour: Box<PyWrapper<Colour>>) -> Box<PyWrapper<TextStyleRef>> {
    let family = unsafe{CStr::from_ptr(font_family)};
    Box::new(PyWrapper::new(TextStyleRef{val: Rc::new(text_element::TextStyleParams::new(
        family.to_str().unwrap().to_string(),
        if bold>0 { text_element::TextWeight::Bold } else {text_element::TextWeight::Normal},
        if italic>0 { text_element::TextSlant::Italic } else {text_element::TextSlant::Normal},
        size,
        *colour.consume()))}))
}

// Function exported to Python for creating a boxed `TextStyleParams`
#[no_mangle]
pub extern "C" fn new_text_style_params_default() -> Box<PyWrapper<TextStyleRef>> {
    Box::new(PyWrapper::new(TextStyleRef{val: Rc::new(text_element::TextStyleParams::default())}))
}

#[no_mangle]
pub extern "C" fn destroy_text_style_params(wrapper: Box<PyWrapper<TextStyleRef>>) {
    wrapper.destroy();
}


// Function exported to Python for creating a boxed `TextStyleParams`
#[no_mangle]
pub extern "C" fn new_text(text: *mut c_char, style: Box<PyWrapper<TextStyleRef>>) -> Box<PyWrapper<TPres>> {
    let t = unsafe{CStr::from_ptr(text)};
    Box::new(PyWrapper::from_boxed(Text::new(t.to_str().unwrap().to_string(),
                                      style.consume().val)))
}

#[no_mangle]
pub extern "C" fn destroy_pres(wrapper: Box<PyWrapper<TPres>>) {
    wrapper.destroy();
}




//
//// Function exported to Python for creating a boxed `TextVisual`
//#[no_mangle]
//pub extern "C" fn new_pres_text(text: *mut c_char, r: f64, g: f64, b: f64) -> Box<PyWrapper<TextVisual>> {
//    let t = unsafe{CStr::from_ptr(text)};
//    Box::new(PyWrapper::new(TextVisual{text: t.to_str().unwrap().to_string(), r: r, g: g, b: b}))
//}
//
//py_wrapper_destructor!(TextVisual, destroy_text_visual);
