//! RRCell Sharing mechanism
//! RRCell owns the object that is to be
//! shared, and allows it to be so through borrow() and
//! borrow_mut().
//!
//! The wrapper object is cloned though
//! the sharing process, but the shared
//! object itself is not, allowing it to
//! be aliased in a safe manner.


use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::Deref;
use std::fmt;

#[derive(Clone)]
pub struct RRCell<T> {
    v: Rc<RefCell<T>>
}

impl <T> RRCell<T> {
    pub fn new(t: T)-> RRCell<T> {
        RRCell{v: Rc::new(RefCell::new(t))}
    }
}

impl <T> RRCell<T> {
    /// borrow the underlying shared object.
    pub fn borrow(&self) -> Ref<T> {
        self.v.borrow()
    }

    /// borrow the underlying shared object,
    /// allowing it to be mutable.
    pub fn borrow_mut(&self) -> RefMut<T> {
        let r = self.v.borrow_mut();
        r
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

impl <'a, T> Deref for RRCell<T>{
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe {self.as_ptr().as_ref().unwrap()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn split (s: RRCell<String>) -> Vec<String> {
        s.split_whitespace().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_rrcell() {
        let s = RRCell::new("Good morning".to_string());
        let s2 = s.clone();
        s.borrow_mut().push('!');
        println!("{:?}",s2);
        
        // Deref kicking in...
        let n = s2.len();
        
        println!("{:?}", n);
        
        // mutation has to be explicit
        s2.borrow_mut().push_str(" Dr. Chandra");
        
        println!("{:?} {}",s2.borrow(), s);
        
        println!("{:?}", split(s2.clone()));
    }
}
