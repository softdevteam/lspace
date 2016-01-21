use std::rc::Rc;

use graphics::border;
use layout::flow_layout;
use elements::element::{TElement, ElementRef, elem_as_ref};
use elements::{text_element, column, row, flow, root_element, border_element};
use elements::bin::{TBinElement};
use elements::container_sequence::{TContainerSequenceElement};
use pres::pres::{Pres, TPres, PresBuildCtx};


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
