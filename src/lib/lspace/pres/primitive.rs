use std::rc::Rc;
use std::cell::RefCell;

use layout::flow_layout;
use elements::element::{TElement, ElementRef, elem_as_ref};
use elements::{text_element, column, row, flow, root_element};
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
        let elem = elem_as_ref(column::ColumnElement::new(0.0));
        elem.as_container_sequence().unwrap().set_children(&elem, &child_elems);
        return elem;
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
        let elem = elem_as_ref(row::RowElement::new(0.0));
        elem.as_container_sequence().unwrap().set_children(&elem, &child_elems);
        return elem;
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
        let elem = elem_as_ref(flow::FlowElement::new(0.0, 0.0, flow_layout::FlowIndent::NoIndent));
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
