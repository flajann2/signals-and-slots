//! Basic Sharing mechanism

use std::rc::Rc;
use std::cell::{RefCell,Ref, RefMut};
use std::ops::Deref;
use std::fmt;

#[derive(Clone)]
struct RRCell<T> {
    v: Rc<RefCell<T>>
}

impl <T> RRCell<T> {
    fn new(t: T)-> RRCell<T> {
        RRCell{v: Rc::new(RefCell::new(t))}
    }
}

impl <T> RRCell<T> {
    fn borrow(&self) -> Ref<T> {
        self.v.borrow()
    }

    fn borrow_mut(&self) -> RefMut<T> {
        self.v.borrow_mut()
    }

    fn as_ptr(&self) -> *mut T {
        self.v.as_ptr()
    }
}


impl <T: fmt::Display> fmt::Display for RRCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl <T: fmt::Debug> fmt::Debug for RRCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl <'a,T> Deref for RRCell<T>{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe {self.as_ptr().as_ref().unwrap()}
    }

}
