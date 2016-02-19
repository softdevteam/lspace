use elements::element::ElementRef;
use elements::container::TContainerElement;

/// Bin element trait; all bin elements should implement this and delegate implementation to
/// same named methods defined in `BinElementMut` below.
pub trait TBinElement : TContainerElement {
    fn get_child(&self) -> Option<ElementRef>;
    fn set_child(&self, self_ref: &ElementRef, child: ElementRef);
    fn clear_child(&self);
}


/// Bin element component, should be placed behind `RefCell` mutation barrier
/// Manages child element
pub struct BinComponentMut {
    child: Option<[ElementRef; 1]>,
}


const NO_CHILDREN: [ElementRef; 0] = [];

impl BinComponentMut {
    pub fn new() -> BinComponentMut {
        return BinComponentMut{child: None};
    }

    pub fn children(&self) -> &[ElementRef] {
        return match self.child {
            None => &NO_CHILDREN[..],
            Some(ref ch) => &ch[..]
        };
    }

    pub fn get_child(&self) -> Option<ElementRef> {
        let child: Option<ElementRef> = match self.child {
            None => None,
            Some(ref ch) => Some(ch[0].clone()),
        };
        return child;
    }

    pub fn set_child(&mut self, self_ref: &ElementRef, child: ElementRef) {
        match self.child {
            None => {},
            Some(ref ch) => ch[0].set_parent(None)
        };
        child.set_parent(Some(self_ref));
        self.child = Some([child]);

    }

    pub fn clear_child(&mut self) {
        match self.child {
            None => {},
            Some(ref ch) => ch[0].set_parent(None)
        };
        self.child = None;
    }
}
