use std::cell::{RefCell, Ref};
use std::ffi::CStr;
use libc::c_char;

use cairo::{ffi, Context};
use glib::translate::*;

#[macro_export]
macro_rules! py_wrapper_destructor {
    ($ty_name:ident, $fn_name:ident) => {
    }
}

//
// Python wrapper that manages a Rust object such that Python can hold a reference to it, while
// allowing it to be borrowed as a reference for use by Rust functions or consumed in order to
// transfer its ownership to be passed to Rust functions that need to consume it.
//
struct PyWrapperMut<T: ?Sized> {
    val: Option<Box<T>>
}

impl <T: ?Sized> PyWrapperMut<T> {
    /// Destroy an object
    fn destroy(&mut self) {
        self.val = None;
    }

    /// Consume object; attempting to comsume or borrow an already consumed object results
    /// in a panic.
    fn consume(&mut self) -> Box<T> {
        if self.val.is_none() {
            panic!("Attempting to consume PyWrapper value that has already been consumed")
        }
        self.val.take().unwrap()
    }

    /// Borrow reference
    fn borrow(&self) -> &Box<T> {
        if self.val.is_none() {
            panic!("Attempting to borrow PyWrapper value that has already been consumed")
        }
        let r = self.val.as_ref().unwrap();
        r
    }
}

/// Python wrapper for Rust objects
pub struct PyWrapper<T: ?Sized> {
    m: RefCell<PyWrapperMut<T>>
}

impl <T> PyWrapper<T> {
    pub fn new(x: T) -> PyWrapper<T> {
        PyWrapper{m: RefCell::new(PyWrapperMut{val: Some(Box::new(x))})}
    }
}

impl <T: ?Sized> PyWrapper<T> {
    pub fn from_boxed(x: Box<T>) -> PyWrapper<T> {
        PyWrapper{m: RefCell::new(PyWrapperMut{val: Some(x)})}
    }

    pub fn destroy(&self) {
        self.m.borrow_mut().destroy();
    }

    pub fn consume(&self) -> Box<T> {
        self.m.borrow_mut().consume()
    }

    pub fn borrow(&self) -> Ref<Box<T>> {
        Ref::map(self.m.borrow(), |x| x.borrow())
    }
}


