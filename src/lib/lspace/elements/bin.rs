use std::cell::Ref;

use elements::element::ElementRef;
use elements::container::TContainerElement;

pub trait TBinElement : TContainerElement {
    fn get_child(&self) -> Option<Ref<ElementRef>>;
    fn set_child(&self, self_ref: &ElementRef, child: ElementRef);
    fn clear_child(&self);
}
