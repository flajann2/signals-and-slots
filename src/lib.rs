//! Library for Signals and Slots
#![feature(associated_type_defaults)]
#![feature(concat_idents)]

// TODO: Remove the following allow directives
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

pub mod rrcell;

use rrcell::RRCell;
use std::vec::Vec;
use std::iter::Iterator;
use std::marker::Sized;
use std::collections::VecDeque;
use std::boxed::Box;
use std::any::Any;
use std::default::Default;

use std::rc::Rc;
use std::cell::{RefCell,Ref, RefMut};
    
extern crate ident;
use ident::*;


pub trait Gizmo {
    fn emit_message(self, mess: &dyn Any);
    fn receive_message (self, mess: &dyn Any);
    fn remove(self, slot: &dyn Gizmo);
}

#[macro_export]
macro_rules! gizmo {
    (struct $widget:ident<$a:tt> { $($tt:tt)* } with_message = $message:path; ) => {
        #[derive(Clone)]
        struct $widget<$a> { $($tt)*
                         boilerplate: i32,
                         slots: Vec<RRCell<&$a dyn Gizmo>>,
                         messages: Vec<$message>,
        }

        impl <$a> Gizmo for $widget<$a> {
            fn emit_message(self, mess: &dyn Any) {
            }
            
            fn receive_message(self, mess: &dyn Any) {
            }

            fn remove(self, slot: &dyn Gizmo) {
            }
        }

        impl <$a> Default for $widget<$a> {
            fn default() -> $widget<$a> {
                $widget {
                    boilerplate: 0,
                    slots: vec![],
                    messages: vec![],
                    name: String::from("")                        
                }                
            }
        }
    };
}

#[macro_export]
macro_rules! wire {
    ($emitter:ident to $head:ident $(+ $tail:ident)*) => {{
        $emitter.slots.push($head.clone());
        $($emitter.slots.push($tail.clone());)*
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    enum SlideWMessage {
        Str(String),
        Num(i32),
        Empty
    }
 
    gizmo! {
        struct SlideW<'a> {
            name: String,
        } with_message = SlideWMessage;
    }

    impl <'a> SlideW<'a> {
        fn new(name: &str) -> RRCell<SlideW<'a>> {
            RRCell::new(SlideW{name: name.to_string(), ..Default::default()})
        }
    }

    
    #[test]
    fn test_basic_signal_slot() {
        let a = SlideW::new("alpha");
        let b = SlideW::new("beta");
        let c = SlideW::new("gamma");
        // a is the signal, both b and c are slots to receive a's signals
        wire!{ a to b + c };
        wire!{ c to a + b };

        // b removes itself from receiving a's signals
        a.remove(&b);        
    }
}
