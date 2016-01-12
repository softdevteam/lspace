use std::cell::Ref;

use elements::element::ElementRef;
use elements::container::TContainerElement;


pub trait TContainerSequenceElement : TContainerElement {
    fn get_children(&self) -> Ref<Vec<ElementRef>>;
    fn set_children(&self, self_ref: &ElementRef, children: &Vec<ElementRef>);
}


/// Container sequence element component, should be placed behind `RefCell` mutation barrier
/// Manages child elements
pub struct ContainerSequenceComponentMut {
    children: Vec<ElementRef>,
}

impl ContainerSequenceComponentMut {
    pub fn new() -> ContainerSequenceComponentMut {
        return ContainerSequenceComponentMut{children: Vec::new()};
    }

    pub fn children(&self) -> &[ElementRef] {
        return &self.children[..];
    }

    pub fn get_children(&self) -> &Vec<ElementRef> {
        return &self.children;
    }

    pub fn set_children(&mut self, self_ref: &ElementRef, children: &Vec<ElementRef>) {
        for child in self.children.iter() {
            child.set_parent(None);
        }
        for child in children {
            child.set_parent(Some(self_ref));
        }
        self.children.clone_from(children);
    }
}

