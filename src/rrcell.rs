//! Basic Sharing mechanism

use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::Deref;
use std::fmt;

#[derive(Clone)]
pub struct RRCell<'a, T> {
    v: Rc<RefCell<&'a T>>
}

impl <'a, T> RRCell<'a, T> {
    pub fn new(t: &'a T)-> RRCell<'a, T> {
        RRCell{v: Rc::new(RefCell::new(t))}
    }
}

impl <'a, T> RRCell<'a, T> {
    fn borrow(&self) -> Ref<&T> {
        self.v.borrow()
    }

    fn borrow_mut(&self) -> RefMut<&'a T> {
        self.v.borrow_mut()
    }

    fn as_ptr(&self) -> *mut T {
        self.v.as_ptr()
    }
}


impl <'a, T: fmt::Display> fmt::Display for RRCell<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl <'a, T: fmt::Debug> fmt::Debug for RRCell<'_, &'a T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl <'a, T> Deref for RRCell<'a, T>{
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
        let s = RRCell::new("hello".to_string());
        let s2 = s.clone();
        s2.borrow_mut().push('!');
        println!("{:?}",s2);
        
        // Deref kicking in...
        let n = s2.len();
        
        println!("{:?}", n);
        
        // mutation has to be explicit
        s2.borrow_mut().push_str(" dolly");
        
        println!("{:?} {}",s2.borrow(), s);
        
        println!("{:?}", split(s2.clone()));
    }
}
