use std::rc::Rc;
use std::ffi::CStr;
use std::slice;
use std::mem;
use libc::c_char;

use graphics::border;
use layout::flow_layout;
use elements::element::{TElement, ElementRef, elem_as_ref};
use elements::{text_element, column, row, flow, root_element, border_element};
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use pres::pres::{Pres, TPres, PresBuildCtx, PyPres, PyPresOwned};
use pyrs::{PyPrimWrapper, PyWrapper};


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
                                                  &pres_ctx.elem_ctx);
        return elem_as_ref(elem);
    }
}


pub struct Border {
    child: Pres,
    border: Rc<border::Border>
}

impl Border {
    pub fn new(child: Pres, border: Rc<border::Border>) -> Pres {
        Box::new(Border{child: child, border: border})
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



pub type PyFlowIndent = PyPrimWrapper<flow_layout::FlowIndent>;
pub type PyFlowIntendOwned = Box<PyFlowIndent>;

// Functions exported to Python for creating wrapped `FlowIndent`
#[no_mangle]
pub extern "C" fn new_flow_indent_no_indent() -> PyFlowIntendOwned {
    Box::new(PyFlowIndent::new(flow_layout::FlowIndent::NoIndent))
}

#[no_mangle]
pub extern "C" fn new_flow_indent_first(indent: f64) -> PyFlowIntendOwned {
    Box::new(PyFlowIndent::new(flow_layout::FlowIndent::First{indent: indent}))
}

#[no_mangle]
pub extern "C" fn new_flow_indent_except_first(indent: f64) -> PyFlowIntendOwned {
    Box::new(PyFlowIndent::new(flow_layout::FlowIndent::ExceptFirst{indent: indent}))
}

#[no_mangle]
pub extern "C" fn destroy_flow_indent(wrapper: PyFlowIntendOwned) {
    PyFlowIndent::destroy(wrapper);
}



// Function exported to Python for creating a wrapped `Text`
#[no_mangle]
pub extern "C" fn new_text(text: *mut c_char,
                           style: &text_element::PyTextStyleParams) -> PyPresOwned {
    let t = unsafe{CStr::from_ptr(text)};
    Box::new(PyPres::from_boxed(Text::new(t.to_str().unwrap().to_string(),
                                          text_element::PyTextStyleParams::get_rc(style))))
}


// Function exported to Python for creating a wrapped `Border`
#[no_mangle]
pub extern "C" fn new_border_pres(child: PyPresOwned, border: &border::PyBorder) -> PyPresOwned {
    Box::new(PyPres::from_boxed(Border::new(PyPres::consume(child),
                                            border::PyBorder::get_rc(border))))
}


// Helper function for converting a C array of wrapped children into a Vec<Pres>
fn unwrap_and_consume_pres_array(pres_array: *mut PyPresOwned, n_pres: usize) -> Vec<Pres>{
    // Convert from raw to a slice
    let pres_slice: &[PyPresOwned] = unsafe{slice::from_raw_parts(pres_array, n_pres)};
    // Consume
    let pres_vec: Vec<Pres> = pres_slice.iter().map(|p_ref: &PyPresOwned| {
        unsafe {
            // Presentations are of type `PyPresOwned`; a boxed type or pointer
            // `p` is a `&PyPresOwned`
            // Transmute:
            // - &PyPresOwned -> *PyPresOwned -> *usize
            let p_as_usize_ptr: *const usize = mem::transmute(p_ref);
            // Deref the *usize pointer to get a `usize` (transmuted `PyPresOwned`)
            let p_as_usize: usize = *p_as_usize_ptr;
            // Transmute the `usize` back to `PyPresOwned`
            let p: PyPresOwned = mem::transmute(p_as_usize);
            // Consume and return
            PyPres::consume(p)
        }
    }).collect();
    pres_vec
}

// Function exported to Python for creating a wrapped `Column`
#[no_mangle]
pub extern "C" fn new_column(children_arr: *mut PyPresOwned, n_children: usize,
                             y_spacing: f64) -> PyPresOwned {
    let children = unwrap_and_consume_pres_array(children_arr, n_children);
    Box::new(PyPres::from_boxed(Column::new_full(children, y_spacing)))
}

// Function exported to Python for creating a wrapped `Row`
#[no_mangle]
pub extern "C" fn new_row(children_arr: *mut PyPresOwned, n_children: usize,
                          x_spacing: f64) -> PyPresOwned {
    let children = unwrap_and_consume_pres_array(children_arr, n_children);
    Box::new(PyPres::from_boxed(Row::new_full(children, x_spacing)))
}

// Function exported to Python for creating a wrapped `Flow`
#[no_mangle]
pub extern "C" fn new_flow(children_arr: *mut PyPresOwned, n_children: usize,
                           x_spacing: f64, y_spacing: f64, indent: &PyFlowIndent) -> PyPresOwned {
    let children = unwrap_and_consume_pres_array(children_arr, n_children);
    Box::new(PyPres::from_boxed(Flow::new_full(children, x_spacing, y_spacing,
                                               PyFlowIndent::get_value(indent))))
}
