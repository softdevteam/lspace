use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::ops::Deref;
use std::ffi::CStr;
use libc::c_char;

use cairo::{ffi, Context};
use glib::translate::*;

#[macro_export]
macro_rules! py_wrapper_destructor {
    ($ty_name:ident, $fn_name:ident) => {
    }
}


#[derive(Copy, Clone)]
pub struct PyPrimWrapper<T: Clone> {
    val: T
}

impl <T: Clone> PyPrimWrapper<T> {
    pub fn new(x: T) -> PyPrimWrapper<T> {
        PyPrimWrapper{val: x}
    }

    pub fn destroy(wrapper: Box<PyPrimWrapper<T>>) {
        // No-op
        // This method is present so that calling it from exported FFI functions
        // ensures that the types are correct, ensuring that the reference is correctly destroyed
    }

    pub fn get_value(wrapper: &PyPrimWrapper<T>) -> T {
        return wrapper.val.clone();
    }

    pub fn get_value_optional(wrapper_ptr: *mut PyPrimWrapper<T>) -> Option<T> {
        if !wrapper_ptr.is_null() {
            let wrapper: &PyPrimWrapper<T> = unsafe{&*wrapper_ptr};
            Some(PyPrimWrapper::get_value(wrapper))
        }
        else {
            None
        }
    }

    pub fn borrow<'a> (wrapper: &'a PyPrimWrapper<T>) -> &'a T {
        return &wrapper.val;
    }

    pub fn borrow_optional<'a>(wrapper_ptr: *mut PyPrimWrapper<T>) -> Option<&'a T> {
        if !wrapper_ptr.is_null() {
            let wrapper: &PyPrimWrapper<T> = unsafe{&*wrapper_ptr};
            Some(PyPrimWrapper::borrow(wrapper))
        }
        else {
            None
        }
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

    pub fn destroy(wrapper: Box<PyWrapper<T>>) {
        wrapper.m.borrow_mut().destroy();
    }

    pub fn destroy_optional(wrapper_ptr: *mut PyWrapper<T>) {
        if !wrapper_ptr.is_null() {
            let wrapper: Box<PyWrapper<T>> = unsafe{Box::from_raw(wrapper_ptr)};
            PyWrapper::destroy(wrapper);
        }
    }

    pub fn consume(wrapper: Box<PyWrapper<T>>) -> Box<T> {
        wrapper.m.borrow_mut().consume()
    }

    pub fn consume_optional(wrapper_ptr: *mut PyWrapper<T>) -> Option<Box<T>> {
        if !wrapper_ptr.is_null() {
            let wrapper: Box<PyWrapper<T>> = unsafe{Box::from_raw(wrapper_ptr)};
            Some(PyWrapper::consume(wrapper))
        }
        else {
            None
        }
    }

    pub fn borrow(wrapper: &PyWrapper<T>) -> Ref<Box<T>> {
        Ref::map(wrapper.m.borrow(), |x| x.borrow())
    }

    pub fn borrow_optional<'a>(wrapper_ptr: *mut PyWrapper<T>) -> Option<Ref<'a, Box<T>>> {
        if !wrapper_ptr.is_null() {
            let wrapper: &PyWrapper<T> = unsafe{&*wrapper_ptr};
            Some(PyWrapper::borrow(wrapper))
        }
        else {
            None
        }
    }
}


pub struct PyRcWrapper<T> {
    val: Rc<T>
}

impl <T> PyRcWrapper<T> {
    pub fn new(x: Rc<T>) -> PyRcWrapper<T> {
        PyRcWrapper{val: x}
    }

    pub fn from_value(x: T) -> PyRcWrapper<T> {
        PyRcWrapper{val: Rc::new(x)}
    }

    pub fn destroy(wrapper: Box<PyRcWrapper<T>>) {
        // No-op
        // This method is present so that calling it from exported FFI functions
        // ensures that the types are correct, ensuring that the reference is correctly destroyed
    }

    pub fn get_rc(wrapper: &PyRcWrapper<T>) -> Rc<T> {
        return wrapper.val.clone();
    }
}
