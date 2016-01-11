use std::rc::Rc;
use std::cell::RefCell;

use layout::flow_layout;
use elements::element::{TElement, ElementRef};
use elements::{text_element, column, row, flow, root_element};
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
        return ElementRef::new(elem);
    }
}


pub struct Column {
    children: Vec<Pres>
}

impl Column {
    pub fn new(children: Vec<Pres>) -> Pres {
        return Box::new(Column{children: children});
    }
}

impl TPres for Column {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child_elems = self.children.iter().map(|p| p.build(pres_ctx)).collect();
        let elem = column::ColumnElement::new(child_elems, 0.0);
        return ElementRef::new(elem);
    }
}


pub struct Row {
    children: Vec<Pres>
}

impl Row {
    pub fn new(children: Vec<Pres>) -> Pres {
        return Box::new(Row{children: children});
    }
}

impl TPres for Row {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child_elems = self.children.iter().map(|p| p.build(pres_ctx)).collect();
        let elem = row::RowElement::new(child_elems, 0.0);
        return ElementRef::new(elem);
    }
}


pub struct Flow {
    children: Vec<Pres>
}

impl Flow {
    pub fn new(children: Vec<Pres>) -> Pres {
        return Box::new(Flow{children: children});
    }
}

impl TPres for Flow {
    fn build(&self, pres_ctx: &PresBuildCtx) -> ElementRef {
        let child_elems = self.children.iter().map(|p| p.build(pres_ctx)).collect();
        let elem = flow::FlowElement::new(child_elems, 0.0, 0.0, flow_layout::FlowIndent::NoIndent);
        return ElementRef::new(elem);
    }
}


pub fn root_containing(p: &Pres, ctx: &PresBuildCtx) -> root_element::RootElement {
    let e = p.build(ctx);
    return root_element::RootElement::new(e);
}
