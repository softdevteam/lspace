#![feature(test)]
#![feature(rc_unique)]

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate test;
    extern crate lspace;

    use std::rc::Rc;
    use std::cell::{RefCell, Ref, RefMut};

    use self::lspace::layout::lreq::LReq;
    use self::lspace::layout::lalloc::LAlloc;
    use self::lspace::layout::{flow_layout, vertical_layout};

    //
    // This benchmark implements a simplified version of the element type hierarhcy that
    // performs only layout. It builds an element tree of approx 500k 'text' elements,
    // organised into paragraphs.
    // The type used to represent a parent to child reference can be varied by uncommenting
    // sections below. The benchmark will report the time take to perform a full layout
    // of the element tree.
    //
    // An implementation that used generics was attempted. Unfortunately, various attempts
    // at this all fell foul of Rust's borrow checker, so this was abandoned.
    //
    // Results are presented just below.
    //

    // Benchmark results summary
    //
    // When varying the reference type within the `ElementChildRef` structure, the following
    // results were obtained:
    //
    // Box<TElement>: ~49,940,000 ns                (Base case; boxed trait)
    // Box<Box<TElement>>: ~53,300,000 ns           (1 additional pointer indirection)
    // RefCell<Box<TElement>>: ~62,260,000 ns       (interior mutability)
    // Rc<Box<TElement>>: ~62,100,000 ns            (multiple ownership, mutate via `Rc::get_mut`)
    // Rc<RefCell<Box<TElement>>>: ~76,880,000 ns   (everything)
    //
    // To run the benchmark yourself, uncomment the appropriate `ElementChildRef`
    // implementation and run with `cargo bench`.


    //
    // Box<TElement>: ~ 49,940,000 ns
    //
    // pub struct ElementChildRef {
    //     x: Box<TElement>
    // }
    //
    // pub type ElementBorrow<'a> = &'a Box<TElement>;
    // pub type ElementMutBorrow<'a> = &'a mut Box<TElement>;
    //
    // impl ElementChildRef {
    //     pub fn new<T: TElement + 'static>(x: T) -> ElementChildRef {
    //         return ElementChildRef{x: Box::new(x)};
    //     }
    //
    //     pub fn get(&self) -> ElementBorrow {
    //         return &self.x;
    //     }
    //
    //     pub fn get_mut(&mut self) -> ElementMutBorrow {
    //         return &mut self.x;
    //     }
    // }


    //
    // Box<Box<TElement>>: ~ 53,300,000 ns
    //
    // pub struct ElementChildRef {
    //     x: Box<Box<TElement>>
    // }
    //
    // pub type ElementBorrow<'a> = &'a Box<Box<TElement>>;
    // pub type ElementMutBorrow<'a> = &'a mut Box<Box<TElement>>;
    //
    // impl ElementChildRef {
    //     pub fn new<T: TElement + 'static>(x: T) -> ElementChildRef {
    //         return ElementChildRef{x: Box::new(Box::new(x))};
    //     }
    //
    //     pub fn get(&self) -> ElementBorrow {
    //         return &self.x;
    //     }
    //
    //     pub fn get_mut(&mut self) -> ElementMutBorrow {
    //         return &mut self.x;
    //     }
    // }


    //
    // RefCell<Box<TElement>>: ~ 62,260,000 ns
    //
    // pub struct ElementChildRef {
    //     x: RefCell<Box<TElement>>
    // }
    //
    // pub type ElementBorrow<'a> = Ref<'a, Box<TElement>>;
    // pub type ElementMutBorrow<'a> = RefMut<'a, Box<TElement>>;
    //
    // impl ElementChildRef {
    //     pub fn new<T: TElement + 'static>(x: T) -> ElementChildRef {
    //         return ElementChildRef{x: RefCell::new(Box::new(x))};
    //     }
    //
    //     pub fn get(&self) -> ElementBorrow {
    //         return self.x.borrow();
    //     }
    //
    //     pub fn get_mut(&mut self) -> ElementMutBorrow {
    //         return self.x.borrow_mut()
    //         ;
    //     }
    // }


    //
    // Rc<Box<TElement>>: ~ 62,100,000 ns
    //
    pub struct ElementChildRef {
        x: Rc<Box<TElement>>
    }

    pub type ElementBorrow<'a> = &'a Rc<Box<TElement>>;
    pub type ElementMutBorrow<'a> = &'a mut Box<TElement>;

    impl ElementChildRef {
        pub fn new<T: TElement + 'static>(x: T) -> ElementChildRef {
            return ElementChildRef{x: Rc::new(Box::new(x))};
        }

        pub fn get(&self) -> ElementBorrow {
            return &self.x;
        }

        pub fn get_mut(&mut self) -> ElementMutBorrow {
            return Rc::get_mut(&mut self.x).unwrap();
        }
    }


    //
    // Rc<RefCell<Box<TElement>>>: ~ 76,880,000 ns
    //
    // pub struct ElementChildRef {
    //     x: Rc<RefCell<Box<TElement>>>
    // }
    //
    // pub type ElementBorrow<'a> = Ref<'a, Box<TElement>>;
    // pub type ElementMutBorrow<'a> = RefMut<'a, Box<TElement>>;
    //
    // impl ElementChildRef {
    //     pub fn new<T: TElement + 'static>(x: T) -> ElementChildRef {
    //         return ElementChildRef{x: Rc::new(RefCell::new(Box::new(x)))};
    //     }
    //
    //     pub fn get(&self) -> ElementBorrow {
    //         return self.x.borrow();
    //     }
    //
    //     pub fn get_mut(&mut self) -> ElementMutBorrow {
    //         return self.x.borrow_mut();
    //     }
    // }


    pub struct ElementReq {
        pub x_req: LReq,
        pub y_req: LReq,
    }


    impl ElementReq {
        pub fn new() -> ElementReq {
            return ElementReq{x_req: LReq::new_empty(), y_req: LReq::new_empty()};
        }

        pub fn new_from_reqs(x_req: LReq, y_req: LReq) -> ElementReq {
            return ElementReq{x_req: x_req, y_req: y_req};
        }
    }


    pub struct ElementAlloc {
        pub x_alloc: LAlloc,
        pub y_alloc: LAlloc,
    }


    impl ElementAlloc {
        pub fn new() -> ElementAlloc {
            return ElementAlloc{x_alloc: LAlloc::new_empty(), y_alloc: LAlloc::new_empty()};
        }
    }


    pub trait TElementLayout {
        fn element_req(&self) -> &ElementReq;
        fn element_alloc(&self) -> &ElementAlloc;
        fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc);

        fn x_req(&self) -> &LReq {
            return &self.element_req().x_req;
        }

        fn x_alloc(&self) -> &LAlloc {
            return &self.element_alloc().x_alloc;
        }

        fn x_req_and_mut_alloc(&mut self) -> (&LReq, &mut LAlloc) {
            let ra = self.element_req_and_mut_alloc();
            return (&ra.0.x_req, &mut ra.1.x_alloc);
        }

        fn y_req(&self) -> &LReq {
            return &self.element_req().y_req;
        }

        fn y_alloc(&self) -> &LAlloc {
            return &self.element_alloc().y_alloc;
        }

        fn y_req_and_mut_alloc(&mut self) -> (&LReq, &mut LAlloc) {
            let ra = self.element_req_and_mut_alloc();
            return (&ra.0.y_req, &mut ra.1.y_alloc);
        }
    }


    pub trait TElement : TElementLayout {
        fn update_x_req(&mut self) {
        }

        fn allocate_x(&mut self) {
        }

        fn update_y_req(&mut self) {
        }

        fn allocate_y(&mut self) {
        }
    }


    pub trait TContainerElement : TElement {
        fn children<'a>(&'a self) -> &'a Vec<ElementChildRef>;
        fn children_mut<'a>(&'a mut self) -> &'a mut Vec<ElementChildRef>;

        fn update_children_x_req(&mut self) {
            for child in self.children_mut() {
                child.get_mut().update_x_req();
            }
        }

        fn update_children_y_req(&mut self) {
            for child in self.children_mut() {
                child.get_mut().update_y_req();
            }
        }

        fn allocate_children_x(&mut self) {
            for child in self.children_mut() {
                child.get_mut().allocate_x();
            }
        }

        fn allocate_children_y(&mut self) {
            for child in self.children_mut() {
                child.get_mut().allocate_y();
            }
        }
    }


    //
    // TEXT
    //

    pub struct TextElement {
        req: Rc<ElementReq>,
        alloc: ElementAlloc,
    }


    impl TextElement {
        pub fn new(req: Rc<ElementReq>) -> TextElement {
            return TextElement{req: req.clone(),
                               alloc: ElementAlloc::new()};
        }
    }


    impl TElementLayout for TextElement {
        fn element_req(&self) -> &ElementReq {
            return &*self.req;
        }

        fn element_alloc(&self) -> &ElementAlloc {
            return &self.alloc;
        }

        fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc) {
            return (&*self.req, &mut self.alloc);
        }
    }


    impl TElement for TextElement {
        fn update_x_req(&mut self) {
            // Nothing to do; requisition is shared
        }

        fn allocate_x(&mut self) {
            // Nothing to do; no children
        }

        fn update_y_req(&mut self) {
            // Nothing to do; requisition is shared
        }

        fn allocate_y(&mut self) {
            // Nothing to do; no children
        }
    }


    //
    // FLOW
    //

    pub struct FlowElement {
        req: ElementReq,
        alloc: ElementAlloc,
        children: Vec<ElementChildRef>,
        lines: Vec<flow_layout::FlowLine>
    }


    impl FlowElement {
        pub fn new(children: Vec<ElementChildRef>) -> FlowElement {
            return FlowElement{req: ElementReq::new(), alloc: ElementAlloc::new(),
                               children: children, lines: Vec::new()};
        }
    }


    impl TElementLayout for FlowElement {
        fn element_req(&self) -> &ElementReq {
            return &self.req;
        }

        fn element_alloc(&self) -> &ElementAlloc {
            return &self.alloc;
        }

        fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc) {
            return (&self.req, &mut self.alloc);
        }
    }


    impl TElement for FlowElement {
        fn update_x_req(&mut self) {
            self.update_children_x_req();
            let child_refs: Vec<ElementBorrow> = self.children.iter().map(|c| c.get()).collect();
            let child_x_reqs: Vec<&LReq> = child_refs.iter().map(|c| c.x_req()).collect();
            self.req.x_req = flow_layout::requisition_x(&child_x_reqs, 0.0, flow_layout::FlowIndent::NoIndent);
        }

        fn allocate_x(&mut self) {
            {
                let mut child_refs: Vec<ElementMutBorrow> = self.children.iter_mut().map(|c| c.get_mut()).collect();
                let mut x_pairs: Vec<(&LReq, &mut LAlloc)> = child_refs.iter_mut().map(
                        |c| c.x_req_and_mut_alloc()).collect();
                self.lines = flow_layout::alloc_x(&self.req.x_req,
                        &self.alloc.x_alloc.without_position(),
                        &mut x_pairs, 0.0, flow_layout::FlowIndent::NoIndent);

            }
            self.allocate_children_x();
        }

        fn update_y_req(&mut self) {
            self.update_children_y_req();
            let child_refs: Vec<ElementBorrow> = self.children.iter().map(|c| c.get()).collect();
            let child_y_reqs: Vec<&LReq> = child_refs.iter().map(|c| c.y_req()).collect();
            self.req.y_req = flow_layout::requisition_y(&child_y_reqs, 0.0, &mut self.lines);
        }

        fn allocate_y(&mut self) {
            {
                let mut child_refs: Vec<ElementMutBorrow> = self.children.iter_mut().map(|c| c.get_mut()).collect();
                let mut y_pairs: Vec<(&LReq, &mut LAlloc)> = child_refs.iter_mut().map(
                        |c| c.y_req_and_mut_alloc()).collect();
                flow_layout::alloc_y(&self.req.y_req,
                    &self.alloc.y_alloc.without_position(),
                    &mut y_pairs, 0.0, &mut self.lines);
            }
            self.allocate_children_y();
        }
    }


    impl TContainerElement for FlowElement {
        fn children<'a>(&'a self) -> &'a Vec<ElementChildRef> {
            return &self.children;
        }

        fn children_mut<'a>(&'a mut self) -> &'a mut Vec<ElementChildRef> {
            return &mut self.children;
        }
    }


    //
    // COLUMN
    //

    pub struct ColumnElement {
        req: ElementReq,
        alloc: ElementAlloc,
        children: Vec<ElementChildRef>,
    }


    impl ColumnElement {
        pub fn new(children: Vec<ElementChildRef>) -> ColumnElement {
            return ColumnElement{req: ElementReq::new(), alloc: ElementAlloc::new(),
                                 children: children};
        }
    }


    impl TElementLayout for ColumnElement {
        fn element_req(&self) -> &ElementReq {
            return &self.req;
        }

        fn element_alloc(&self) -> &ElementAlloc {
            return &self.alloc;
        }

        fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc) {
            return (&self.req, &mut self.alloc);
        }
    }


    impl TElement for ColumnElement {
        fn update_x_req(&mut self) {
            self.update_children_x_req();
            let child_refs: Vec<ElementBorrow> = self.children.iter().map(|c| c.get()).collect();
            let child_x_reqs: Vec<&LReq> = child_refs.iter().map(|c| c.x_req()).collect();
            self.req.x_req = vertical_layout::requisition_x(&child_x_reqs);
        }

        fn allocate_x(&mut self) {
            {
                let mut child_refs: Vec<ElementMutBorrow> = self.children.iter_mut().map(|c| c.get_mut()).collect();
                let mut x_pairs: Vec<(&LReq, &mut LAlloc)> = child_refs.iter_mut().map(
                        |c| c.x_req_and_mut_alloc()).collect();
                vertical_layout::alloc_x(&self.req.x_req,
                        &self.alloc.x_alloc.without_position(), &mut x_pairs);
            }
            self.allocate_children_x();
        }

        fn update_y_req(&mut self) {
            self.update_children_y_req();
            let child_refs: Vec<ElementBorrow> = self.children.iter().map(|c| c.get()).collect();
            let child_y_reqs: Vec<&LReq> = child_refs.iter().map(|c| c.y_req()).collect();
            self.req.y_req = vertical_layout::requisition_y(&child_y_reqs, 0.0, None);
        }

        fn allocate_y(&mut self) {
            {
                let mut child_refs: Vec<ElementMutBorrow> = self.children.iter_mut().map(|c| c.get_mut()).collect();
                let mut y_pairs: Vec<(&LReq, &mut LAlloc)> = child_refs.iter_mut().map(
                        |c| c.y_req_and_mut_alloc()).collect();
                vertical_layout::alloc_y(&self.req.y_req,
                        &self.alloc.y_alloc.without_position(),
                        &mut y_pairs, 0.0, None);
            }
            self.allocate_children_y();
        }
    }


    impl TContainerElement for ColumnElement {
        fn children<'a>(&'a self) -> &'a Vec<ElementChildRef> {
            return &self.children;
        }

        fn children_mut<'a>(&'a mut self) -> &'a mut Vec<ElementChildRef> {
            return &mut self.children;
        }
    }


    //
    // ROOT
    //

    pub struct RootElement {
        req: ElementReq,
        alloc: ElementAlloc,
        children: Vec<ElementChildRef>,
    }

    impl RootElement {
        pub fn new(child: ElementChildRef) -> RootElement {
            return RootElement{req: ElementReq::new(), alloc: ElementAlloc::new(),
                               children: vec![child]};
        }

        pub fn root_requisition_x(&mut self) -> f64 {
            self.update_x_req();
            return self.req.x_req.size().size();
        }

        pub fn root_allocate_x(&mut self, width: f64) {
            self.alloc.x_alloc = LAlloc::new_from_req_in_avail_size(&self.req.x_req, 0.0, width);
            self.allocate_x();
        }

        pub fn root_requisition_y(&mut self) -> f64 {
            self.update_y_req();
            return self.req.y_req.size().size();
        }

        pub fn root_allocate_y(&mut self, height: f64) {
            self.alloc.y_alloc = LAlloc::new_from_req_in_avail_size(&self.req.y_req, 0.0, height);
            self.allocate_y();
        }
    }

    impl TElementLayout for RootElement {
        fn element_req(&self) -> &ElementReq {
            return &self.req;
        }

        fn element_alloc(&self) -> &ElementAlloc {
            return &self.alloc;
        }

        fn element_req_and_mut_alloc(&mut self) -> (&ElementReq, &mut ElementAlloc) {
            return (&self.req, &mut self.alloc);
        }
    }

    impl TElement for RootElement {
        fn update_x_req(&mut self) {
            self.update_children_x_req();
            self.req.x_req = self.children[0].get().x_req().clone();
        }

        fn allocate_x(&mut self) {
            self.children[0].get_mut().x_req_and_mut_alloc().1.clone_from(&self.alloc.x_alloc);
            self.allocate_children_x();
        }

        fn update_y_req(&mut self) {
            self.update_children_y_req();
            self.req.y_req = self.children[0].get().y_req().clone();
        }

        fn allocate_y(&mut self) {
            self.children[0].get_mut().y_req_and_mut_alloc().1.clone_from(&self.alloc.y_alloc);
            self.allocate_children_y();
        }
    }

    impl TContainerElement for RootElement {
        fn children<'a>(&'a self) -> &'a Vec<ElementChildRef> {
            return &self.children;
        }

        fn children_mut<'a>(&'a mut self) -> &'a mut Vec<ElementChildRef> {
            return &mut self.children;
        }
    }


    //
    // CONTENT BUILDING FUNCTIONS
    //

    fn build_text_reqs(n_reqs: usize, req_rep_space: usize) -> Vec<Rc<ElementReq>> {
        return (0..n_reqs).map(|x| {
            Rc::new(ElementReq::new_from_reqs(LReq::new_fixed_size((50 + (x % req_rep_space) * 10) as f64),
                                      LReq::new_fixed_ref(7.0, 3.0)))
        }).collect();
    }

    fn build_flow(mut pos: usize, n_words: usize, text_reqs: &Vec<Rc<ElementReq>>) -> (usize, FlowElement) {
        let mut text_elements = Vec::with_capacity(n_words);

        for i in 0..n_words {
            let req = text_reqs[pos].clone();
            let elem = ElementChildRef::new(TextElement::new(req));
            text_elements.push(elem);
            pos = pos + 1;
            if pos >= text_reqs.len() {
                pos = 0;
            }
        }

        let f = FlowElement::new(text_elements);
        return (pos, f);
    }

    fn build_root(n_paras: usize, n_words: usize, n_reqs: usize, req_rep_space: usize) -> RootElement {
        let text_reqs = build_text_reqs(n_reqs, req_rep_space);
        let mut flow_elements = Vec::with_capacity(n_paras);
        let mut pos = 0;
        for i_para in 0..n_paras {
            let (pos2, f) = build_flow(pos, n_words, &text_reqs);
            pos = pos2;
            flow_elements.push(ElementChildRef::new(f));
        }

        let c = ColumnElement::new(flow_elements);
        return RootElement::new(ElementChildRef::new(c));
    }

    fn build_test_element_tree() -> RootElement {
        return build_root(16384, 32, 1024, 11);
    }

    fn perform_layout(root: &mut RootElement, width: f64) {
        let rx = root.root_requisition_x();
        root.root_allocate_x(width);
        let ry = root.root_requisition_y();
        root.root_allocate_y(ry);
    }

    #[bench]
    fn bench_reftype(bench: &mut test::Bencher) {
        let mut root = build_test_element_tree();

        bench.iter(move || {
            perform_layout(&mut root, 1024.0);
        });
    }
}
