use std::cell::Ref;

use elements::element::ElementRef;
use elements::container::TContainerElement;

pub trait TContainerSequenceElement : TContainerElement {
    fn get_children(&self) -> Ref<Vec<ElementRef>>;
    fn set_children(&self, self_ref: &ElementRef, children: &Vec<ElementRef>);
}
