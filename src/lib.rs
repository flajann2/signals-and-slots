//! Library for Signals and Slots
#![feature(associated_type_defaults)]
#![feature(concat_idents)]

// TODO: Remove the following allow directives
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use std::vec::Vec;
use std::iter::Iterator;
use std::marker::Sized;
use std::collections::VecDeque;
use std::boxed::Box;
use std::any::Any;
use std::default::Default;

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
        struct $widget<$a> { $($tt)*
                         boilerplate: i32,
                         slots: Vec<&$a dyn Gizmo>,
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
    ($emitter:ident to $head:ident $(+ $tail:ident)*) => {
        $emitter.slots.push(&$head);
        $($emitter.slots.push(&$tail);)*
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        fn new(name: &str) -> SlideW<'a> {
            SlideW{name: name.to_string(), ..Default::default()}
        }
    }

    
    #[test]
    fn test_basic_signal_slot() {
        let mut a = SlideW::new("alpha");
        let mut b = SlideW::new("beta");
        let mut c = SlideW::new("gamma");
        // a is the signal, both b and c are slots to receive a's signals
        wire!{ a to b + c };
        wire!{ c to a + b };

        // b removes itself from receiving a's signals
        a.remove(&b);        
    }
}
